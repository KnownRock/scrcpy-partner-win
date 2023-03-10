import type { DeviceConfig, DeviceConfigValue } from '@prisma/client'
import prismaClientLike from './prisma-like-client'

export async function getConfigs (deviceId?: string): Promise<DeviceConfigExt[]> {
  return await prismaClientLike.deviceConfig.findMany({
    where: {
      deviceId
    }
  })
}

export async function saveConfig (
  data: DeviceConfig
): Promise<DeviceConfigExt> {
  // const data = getNoAdditionalPropertiesSchema<DeviceConfig>('DeviceConfig', rawData)

  const currentConfig = await getConfig({
    deviceId: data.deviceId,
    id: data.id
  })

  if (currentConfig != null) {
    return await prismaClientLike.deviceConfig.update({
      where: {
        id: data.id
      },
      data
    })
  } else {
    return await prismaClientLike.deviceConfig.create({
      data: {
        name: data.name,
        device: {
          connect: {
            id: data.deviceId
          }
        }
      }
    })
  }
}

export async function saveConfigItems (
  deviceConfigId: string,
  items: DeviceConfigValue[]
): Promise<void> {
  // delete all items
  await prismaClientLike.deviceConfigValue.deleteMany({
    where: {
      deviceConfigId
    }
  })

  // create new items
  // await Promise.all(items.map(async (item) => {
  //   await prismaClientLike.deviceConfigValue.create({
  //     data: {
  //       deviceConfigId,
  //       key: item.key,
  //       value: item.value
  //     }
  //   })
  // }))

  for (const item of items) {
    await prismaClientLike.deviceConfigValue.create({
      data: {
        deviceConfigId,
        key: item.key,
        value: item.value
      }
    })
  }
}

export async function getConfigItems (
  deviceConfigId: string
): Promise<DeviceConfigValue[]> {
  return await prismaClientLike.deviceConfigValue.findMany({
    where: {
      deviceConfigId
    }
  })
}

export async function getConfig (
  where: {
    deviceId?: string
    id?: string
  }
): Promise<DeviceConfigExt | null> {
  return await prismaClientLike.deviceConfig.findFirst({
    where
  })
}

export async function getConfigById (
  id: string
): Promise<DeviceConfigExtC | null> {
  return await prismaClientLike.deviceConfig.findUnique({
    where: {
      id
    },
    include: {
      deviceConfigValue: true
    }
  })
}

export async function deleteConfigById (
  id: string
): Promise<void> {
  await prismaClientLike.deviceConfig.delete({
    where: {
      id
    }
  })
}

export async function updateConfigLastSeenAt (
  id: string
): Promise<void> {
  await prismaClientLike.deviceConfig.update({
    where: {
      id
    },
    data: {
      lastSeenAt: new Date()
    }
  })
}

export type DeviceConfigExt = DeviceConfig

export type DeviceConfigValueExt = DeviceConfigValue

export type DeviceConfigExtC = DeviceConfigExt & {
  deviceConfigValue: DeviceConfigValueExt[]
}
