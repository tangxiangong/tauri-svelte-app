import { invoke } from "@tauri-apps/api/core";

export const greetRust = async (name: string): Promise<string> => {
  return await invoke("greet", { name });
};