import { invoke } from '@tauri-apps/api/tauri'
import { callTauriFunction } from './tauri'

// export interface Device {
//   name: string
//   id: string
//   model: string
//   device: string
//   deviceProduct: string
//   transportId: string
// }

import type { Device } from '@prisma/client/index.d'

async function getAdbDevices (): Promise<Device[]> {
  const rawOutput = (await callTauriFunction<string>('adb_devices_l'))

  const lines = rawOutput.split('\n')
  const deviceLines = lines.slice(1, lines.length).filter(line => /\S/.exec(line))

  return deviceLines.map(line => {
    const matched = (/(\S+)\s+device product:(\S+) model:(\S+) device:(\S+) transport_id:(\S+)/.exec(line))
    if (matched == null) {
      throw new Error(`Failed to parse device line: ${line}`)
    }
    const [,adbId, product, model] = matched
    const name = adbId
    return {
      name,
      adbId,
      id: '',
      model,
      product
      // transportId
    }
  })
}

export async function getDevices (): Promise<Device[]> {
  const adbDevices = await getAdbDevices()

  return adbDevices
}

export async function lanuchSelf (args: string[]): Promise<void> {
  await invoke('lanuch_self', { args })
}

export type { Device } from '@prisma/client/index.d'
