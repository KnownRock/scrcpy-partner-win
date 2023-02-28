import { invoke } from '@tauri-apps/api/tauri'
import { callTauriFunction } from './tauri'

import type { Device } from '@prisma/client/index.d'
import prismaClientLike from './prisma-like-client'
import { getNoAdditionalPropertiesSchema } from './prisma-field-filter'
import { v4 as uuidv4 } from 'uuid'
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
  isSaved: boolean
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
      id: uuidv4(),
      model,
      product,
      createdAt: null,
      updatedAt: null,
      lastSeenAt: null
    }
  })
}

export async function deleteDevice (deviceId: string): Promise<void> {
  await prismaClientLike.device.delete({
    where: {
      id: deviceId
    }
  })
}

export async function saveDevice (device: Device): Promise<Device> {
  const clearDevice = getNoAdditionalPropertiesSchema<Device>('Device', device)

  const deviceInDb = await prismaClientLike.device.findUnique({
    where: {
      id: device.id
    }
  })

  let newOrUpdatedDevice: Device
  if (deviceInDb == null) {
    newOrUpdatedDevice = await prismaClientLike.device.create({
      data: {
        ...clearDevice,
        updatedAt: undefined,
        id: undefined
      }
    })
  } else {
    newOrUpdatedDevice = await prismaClientLike.device.update({
      where: {
        id: device.id
      },
      data: clearDevice
    })
  }

  return newOrUpdatedDevice
}

export async function getDevices (
  queryMode: 'all' | 'only adb' | 'only saved' | 'only unsaved' = 'all'
): Promise<DeviceExt[]> {
  const adbDevices = await getAdbDevices()
  const savedDevices = await prismaClientLike.device.findMany()
  const unsavedAdbDevices = adbDevices.filter(adbDevice => {
    return savedDevices.find(savedDevice => savedDevice.adbId === adbDevice.adbId) == null
  }).map(adbDevice => {
    return {
      ...adbDevice,
      isConnected: true,
      isSaved: false
    }
  })

  const savedDevicesExt = savedDevices.map(savedDevice => {
    const adbDevice = adbDevices.find(adbDevice => adbDevice.adbId === savedDevice.adbId)
    return {
      ...savedDevice,
      isConnected: adbDevice != null,
      isSaved: true
    }
  })

  // return [
  //   ...(isHaveUnsavedDevices ? unsavedAdbDevices : []),
  //   ...savedDevicesExt
  // ]

  if (queryMode === 'all') {
    return [
      ...unsavedAdbDevices,
      ...savedDevicesExt
    ]
  } else if (queryMode === 'only unsaved') {
    return unsavedAdbDevices
  } else if (queryMode === 'only saved') {
    return savedDevicesExt
  } else if (queryMode === 'only adb') {
    return adbDevices.map(adbDevice => {
      return {
        ...adbDevice,
        isConnected: true,
        isSaved: false
      }
    })
  } else {
    return []
  }
}

export async function lanuchSelf (args: string[]): Promise<void> {
  await invoke('lanuch_self', { args })
}

export type { Device } from '@prisma/client/index.d'
export type { DeviceExt }
