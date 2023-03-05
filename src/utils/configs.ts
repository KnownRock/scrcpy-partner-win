import type { DeviceConfig } from '@prisma/client'
import prismaClientLike from './prisma-like-client'
import { getNoAdditionalPropertiesSchema } from './prisma-field-filter'

export async function getConfigs (
  where: {
    deviceId: string
  }
): Promise<DeviceConfigExt[]> {
  return await prismaClientLike.deviceConfig.findMany({
    where
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

export async function getConfig (
  where: {
    deviceId: string
    id: string
  }
): Promise<DeviceConfigExt | null> {
  return await prismaClientLike.deviceConfig.findFirst({
    where
  })
}

export type DeviceConfigExt = DeviceConfig & {
}
