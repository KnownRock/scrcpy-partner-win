import type { DeviceConfig } from '@prisma/client'
import prismaClientLike from './prisma-like-client'

export async function getConfigs (
  deviceId = 'default'
): Promise<DeviceConfig[]> {
  return await prismaClientLike.deviceConfig.findMany({
    where: {
      deviceId
    }
  })
}
