import { invoke } from '@tauri-apps/api/tauri'
import { callTauriFunction } from './tauri'

export interface Device {
  name: string
  id: string
  model: string
  device: string
  deviceProduct: string
  transportId: string
}

export async function getDevices (): Promise<Device[]> {
  const rawOutput = (await callTauriFunction<string>('adb_devices_l'))

  const lines = rawOutput.split('\n')
  const deviceLines = lines.slice(1, lines.length).filter(line => /\S/.exec(line))

  return deviceLines.map(line => {
    const matched = (/(\S+)\s+device product:(\S+) model:(\S+) device:(\S+) transport_id:(\S+)/.exec(line))
    if (matched == null) {
      throw new Error(`Failed to parse device line: ${line}`)
    }
    const [,id, deviceProduct, model, device, transportId] = matched

    const name = id
    return { name, id, model, device, deviceProduct, transportId }
  })
}

export async function lanuchSelf (args: string[]): Promise<void> {
  await invoke('lanuch_self', { args })
}
