import { invoke } from '@tauri-apps/api/tauri'

export async function callTauriFunction<T> (name: string, args?: Record<string, any>): Promise<T> {
  return await invoke(name, args)
}
