use crate::memory::{Memory, ProcessMemoryInfo};
use serde::Serialize;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfo {
    pub os_name: Option<String>,
    pub kernel_version: Option<String>,
    pub os_version: Option<String>,
    pub long_os_version: Option<String>,
    pub host_name: Option<String>,
    pub cpu_count: usize,
}

/// Single JSON object for `get_used_memory` (no tuple / nested arrays).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UsedMemorySnapshot {
    pub memory: Memory,
    pub top_processes: Vec<ProcessMemoryRow>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessMemoryRow {
    pub pid: u32,
    #[serde(flatten)]
    pub info: ProcessMemoryInfo,
}

#[tauri::command]
pub fn get_used_memory(top_n: Option<usize>) -> UsedMemorySnapshot {
    let n = top_n.unwrap_or(20).clamp(1, 200);
    let mem = Memory::get();
    let top_processes = mem
        .first(n)
        .into_iter()
        .map(|(pid, info)| ProcessMemoryRow { pid, info })
        .collect();
    UsedMemorySnapshot {
        memory: mem,
        top_processes,
    }
}

#[tauri::command]
pub fn get_system_info() -> SystemInfo {
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything()),
    );
    sys.refresh_cpu_all();
    sys.refresh_memory();

    SystemInfo {
        os_name: System::name(),
        kernel_version: System::kernel_version(),
        os_version: System::os_version(),
        long_os_version: System::long_os_version(),
        host_name: System::host_name(),
        cpu_count: sys.cpus().len(),
    }
}
