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

export async function init
(
  isToolMode: boolean = false,
  isAutoSaveLocationAndSize: boolean = false,
  isWindowBorderless: boolean = false,
  configId = ''
): Promise<void> {
  await callTauriFunction('init', {
    isToolMode,
    isAutoSaveLocationAndSize,
    isWindowBorderless,
    configId
  })
}

export async function getEnvArgs (): Promise<string[]> {
  return await callTauriFunction<string[]>('get_env_args')
}

export async function closeApplication (): Promise<void> {
  await callTauriFunction('close_application')
}
// create_ms_link
// target: String, link: String, args: Vec<String>
export async function createMsLink (link: string, args: string[]): Promise<void> {
  await callTauriFunction('create_ms_link', { link, args })
}

export async function runScrcpyCommand (args: string[]): Promise<boolean> {
  return await callTauriFunction<boolean>('run_scrcpy_command', { args })
}

export async function exit (): Promise<void> {
  await callTauriFunction('exit')
}
