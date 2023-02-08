import { invoke } from "@tauri-apps/api/tauri"

export async function getExecMode(): Promise<string> {
  const rawOutput = await invoke("get_exec_mode") as string;  

  return rawOutput;
}
