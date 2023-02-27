import { invoke } from '@tauri-apps/api/tauri'
import { callTauriFunction } from './tauri'

import type { Device } from '@prisma/client/index.d'
import prismaClientLike from './prisma-like-client'

// export interface Device {
//   name: string
//   id: string
//   model: string
//   device: string
//   deviceProduct: string
//   transportId: string
// }
type DeviceExt = Device & {
  isConnected: boolean
}

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

export async function saveDevice (device: Device): Promise<void> {
  const deviceInDb = await prismaClientLike.device.findUnique({
    where: {
      adbId: device.adbId
    }
  })

  if (deviceInDb == null) {
    await prismaClientLike.device.create({
      data: device
    })
  } else {
    await prismaClientLike.device.update({
      where: {
        adbId: device.adbId
      },
      data: device
    })
  }
}

export async function getDevices (): Promise<DeviceExt[]> {
  const adbDevices = await getAdbDevices()
  const savedDevices = await prismaClientLike.device.findMany()

  const unsavedAdbDevices = adbDevices.filter(adbDevice => {
    return savedDevices.find(savedDevice => savedDevice.adbId === adbDevice.adbId) == null
  }).map(adbDevice => {
    return {
      ...adbDevice,
      isConnected: true
    }
  })

  const savedDevicesExt = savedDevices.map(savedDevice => {
    const adbDevice = adbDevices.find(adbDevice => adbDevice.adbId === savedDevice.adbId)
    return {
      ...savedDevice,
      isConnected: adbDevice != null
    }
  })

  return [...unsavedAdbDevices, ...savedDevicesExt]
}

export async function lanuchSelf (args: string[]): Promise<void> {
  await invoke('lanuch_self', { args })
}

export type { Device } from '@prisma/client/index.d'
export type { DeviceExt }
