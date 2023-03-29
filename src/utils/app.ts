import { callTauriFunction } from './tauri'

export async function getExecMode (): Promise<string> {
  return await callTauriFunction<string>('get_exec_mode')
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

export async function exit (): Promise<void> {
  await callTauriFunction('exit')
}
// create_ms_link
// target: String, link: String, args: Vec<String>
export async function createMsLink (link: string, args: string[]): Promise<void> {
  await callTauriFunction('create_ms_link', { link, args })
}

export async function runScrcpyCommand (args: string[]): Promise<boolean> {
  return await callTauriFunction<boolean>('run_scrcpy_command', { args })
}

// get_config_id
export async function getConfigId (): Promise<string> {
  return await callTauriFunction<string>('get_config_id')
}

// open
// async fn open(exec: String, args: Vec<String>, cwd: String) {
export async function open (exec: string, args: string[], cwd: string): Promise<void> {
  await callTauriFunction('open', { exec, args, cwd })
}

// start
// async fn open(exec: String, cwd: String) {
export async function start (exec: string, cwd: string): Promise<void> {
  await callTauriFunction('start', { exec, cwd })
}

// async fn get_current_exe_path() -> String {
export async function getCurrentExePath (): Promise<string> {
  return await callTauriFunction<string>('get_current_exe_path')
}

// async fn get_current_exe_dir() -> String {
export async function getCurrentExeDir (): Promise<string> {
  return await callTauriFunction<string>('get_current_exe_dir')
}

// fn set_record_panel_with_motion_record(record_panel_with_motion_record: bool) {
export async function setRecordPanelWithMotionRecord (recordPanelWithMotionRecord: boolean): Promise<void> {
  await callTauriFunction('set_record_panel_with_motion_record', { recordPanelWithMotionRecord })
}

// async fn close_record_window() {
export async function closeRecordWindow (): Promise<void> {
  await callTauriFunction('close_record_window')
}

// async fn open_record_window() {
export async function openRecordWindow (): Promise<void> {
  await callTauriFunction('open_record_window')
}
