
import gridHelp from 'svelte-grid/build/helper/index.mjs'
import 'svelte-material-ui/bare.css'

import { v4 as uuidv4 } from 'uuid'
import { addableItems } from './addable-items'
import prismaClientLike from '../../utils/prisma-like-client'
import type { DeviceConfig, SideBarConfig as PrismaSidebarConfig } from '@prisma/client'
interface SidebarConfig {
  name: string
  activeLayer: number
  layers: SidebarLayer[]
}

interface SidebarLayer {
  gridSize: [number, number]
  items: SidebarItem[]
}

interface SidebarItem {
  id: string
  item: any
  // [key: number]: {
  //   x: number
  //   y: number
  //   w: number
  //   h: number
  // }
  coord: {
    x: number
    y: number
    w: number
    h: number
  }
}

export function getDefaultSidebarConfig (): SidebarConfig {
  function getDefaultSidebarSingleLayer (): SidebarLayer {
    return {
      gridSize: [1, addableItems.length],
      items: addableItems.map((item, index) => {
        const insertItem = {
          id: uuidv4(),
          item,
          coord: gridHelp.item({
            x: 0,
            y: index,
            w: 1,
            h: 1
          })
        }

        return insertItem
      })

    }
  }

  const sidebarConfig = {
    name: 'default',
    activeLayer: 0,
    layers: Array.from(Array(9)).map((_, i) => {
      if (i === 0) {
        return getDefaultSidebarSingleLayer()
      } else {
        return {
          gridSize: [1, 1] as [number, number],
          items: [] as SidebarItem[]
        }
      }
    })
  }

  return sidebarConfig
}

export async function getConfigWithSidebarConfig (currentConfigId): Promise<DeviceConfig & { sideBarConfig: PrismaSidebarConfig }
| null> {
  const config = await prismaClientLike.deviceConfig.findUnique({
    where: {
      id: currentConfigId
    },
    include: {
      sideBarConfig: true
    }
  })

  return config as DeviceConfig & { sideBarConfig: PrismaSidebarConfig } | null
}

export function getInitConfig (): SidebarConfig {
  return {
    name: 'default',
    activeLayer: 0,
    layers: Array.from(Array(9)).map((_, i) => {
      return {
        gridSize: [1, 1],
        items: [] as any[]
      }
    })
  }
}
