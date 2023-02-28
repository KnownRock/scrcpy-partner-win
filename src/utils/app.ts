import { callTauriFunction } from './tauri'

export async function getExecMode (): Promise<string> {
  return await callTauriFunction<string>('get_exec_mode')
}

export async function showMainWindow (): Promise<void> {
  await callTauriFunction('show_main_window')
}

export async function showToolWindow (): Promise<void> {
  await callTauriFunction('show_tool_window')
}

export async function init (): Promise<void> {
  await callTauriFunction('init')
}
