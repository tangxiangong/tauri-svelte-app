use crate::utils::Storage;
use itertools::Itertools;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use sysinfo::{MemoryRefreshKind, ProcessRefreshKind, RefreshKind, System, UpdateKind};
use tree_ds::prelude::*;

const VIRTUAL_ROOT_PID: u32 = 0;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Memory {
    pub total_memory: Storage,
    pub used_memory: Storage,
    pub total_swap: Storage,
    pub used_swap: Storage,
    #[serde(skip_serializing)]
    pub processes: HashMap<u32, ProcessMemoryInfo>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessMemoryInfo {
    pub memory: Storage,
    pub raw_memory: u64,
    pub name: String,
    pub exe: Option<String>,
    pub parent: Option<u32>,
    pub root: Option<String>,
    pub total_memory: Storage,
}

impl Memory {
    pub fn get() -> Self {
        let mut sys = System::new_with_specifics(
            RefreshKind::nothing()
                .with_memory(MemoryRefreshKind::everything())
                .with_processes(
                    ProcessRefreshKind::nothing()
                        .with_exe(UpdateKind::Always)
                        .with_memory()
                        .with_root(UpdateKind::Always),
                ),
        );
        sys.refresh_all();

        let total_memory = Storage::from_bytes(sys.total_memory());
        let used_memory = Storage::from_bytes(sys.used_memory());
        let total_swap = Storage::from_bytes(sys.total_swap());
        let used_swap = Storage::from_bytes(sys.used_swap());
        let mut processes = sys
            .processes()
            .iter()
            .map(|(pid, process)| {
                let raw_memory = process.memory();
                let memory = Storage::from_bytes(raw_memory);
                let name = process.name().to_string_lossy().to_string();
                let exe = process.exe().map(|path| path.to_string_lossy().to_string());
                let parent = process.parent().map(|pid| pid.as_u32());
                let root = process
                    .root()
                    .map(|path| path.to_string_lossy().to_string());
                let total_memory = memory.clone();

                (
                    pid.as_u32(),
                    ProcessMemoryInfo {
                        memory,
                        raw_memory,
                        name,
                        exe,
                        parent,
                        root,
                        total_memory,
                    },
                )
            })
            .collect::<HashMap<u32, ProcessMemoryInfo>>();

        let tree = build_tree(&processes).unwrap();

        aggregate_total_memory(&tree, &mut processes);

        Self {
            total_memory,
            used_memory,
            total_swap,
            used_swap,
            processes,
        }
    }

    pub fn tree(&self) -> anyhow::Result<Tree<u32, ()>> {
        build_tree(&self.processes)
    }

    pub fn first(&self, n: usize) -> Vec<(u32, ProcessMemoryInfo)> {
        self.processes
            .iter()
            .filter(|(_, process)| matches!(process.parent, None | Some(1)))
            .sorted_by(|(_, a), (_, b)| b.total_memory.to_bytes().cmp(&a.total_memory.to_bytes()))
            .take(n)
            .map(|(pid, process)| (*pid, process.clone()))
            .collect_vec()
    }
}

fn aggregate_total_memory(tree: &Tree<u32, ()>, processes: &mut HashMap<u32, ProcessMemoryInfo>) {
    if let Some(root_node) = tree.get_node_by_id(&VIRTUAL_ROOT_PID)
        && let Ok(children) = root_node.get_children_ids()
    {
        for child_pid in children {
            let _ = calculate_subtree_memory(tree, processes, child_pid);
        }
    }
}

fn calculate_subtree_memory(
    tree: &Tree<u32, ()>,
    processes: &mut HashMap<u32, ProcessMemoryInfo>,
    pid: u32,
) -> Storage {
    let node = match tree.get_node_by_id(&pid) {
        Some(n) => n,
        None => return Storage::from_bytes(0),
    };

    if pid == 1 {
        if let Ok(children_ids) = node.get_children_ids() {
            for child_pid in children_ids {
                let _ = calculate_subtree_memory(tree, processes, child_pid);
            }
        }
        return processes
            .get(&pid)
            .map(|p| p.memory.clone())
            .unwrap_or_else(|| Storage::from_bytes(0));
    }

    let children_total = if let Ok(children_ids) = node.get_children_ids() {
        children_ids
            .iter()
            .map(|child_pid| calculate_subtree_memory(tree, processes, *child_pid))
            .fold(Storage::from_bytes(0), |acc, mem| acc + mem)
    } else {
        Storage::from_bytes(0)
    };

    let self_memory = processes
        .get(&pid)
        .map(|p| p.memory.clone())
        .unwrap_or_else(|| Storage::from_bytes(0));

    let total = &self_memory + &children_total;

    if let Some(process) = processes.get_mut(&pid) {
        process.total_memory = total.clone();
    }

    total
}

fn build_tree(processes: &HashMap<u32, ProcessMemoryInfo>) -> anyhow::Result<Tree<u32, ()>> {
    let mut tree = Tree::new(Some("Process Tree"));

    tree.add_node(Node::new(VIRTUAL_ROOT_PID, None), None)?;

    let mut pending_children: HashMap<u32, Vec<u32>> = HashMap::new();

    let mut added: HashSet<u32> = HashSet::new();

    for (pid, process) in processes {
        match process.parent {
            None => {
                tree.add_node(Node::new(*pid, None), Some(&VIRTUAL_ROOT_PID))?;
                added.insert(*pid);

                process_pending(&mut tree, &mut pending_children, &mut added, *pid)?;
            }
            Some(parent_pid) => {
                if added.contains(&parent_pid) {
                    tree.add_node(Node::new(*pid, None), Some(&parent_pid))?;
                    added.insert(*pid);

                    process_pending(&mut tree, &mut pending_children, &mut added, *pid)?;
                } else {
                    pending_children.entry(parent_pid).or_default().push(*pid);
                }
            }
        }
    }

    Ok(tree)
}

fn process_pending(
    tree: &mut Tree<u32, ()>,
    pending_children: &mut HashMap<u32, Vec<u32>>,
    added: &mut HashSet<u32>,
    parent_id: u32,
) -> anyhow::Result<()> {
    if let Some(children) = pending_children.remove(&parent_id) {
        for child_id in children {
            tree.add_node(Node::new(child_id, None), Some(&parent_id))?;
            added.insert(child_id);

            process_pending(tree, pending_children, added, child_id)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let info = Memory::get();
        let first = info.first(10);
        for (pid, process) in first {
            println!("{}: {}, {}", pid, process.name, process.total_memory);
        }
    }
}
