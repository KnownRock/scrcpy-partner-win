
import gridHelp from 'svelte-grid/build/helper/index.mjs'
import 'svelte-material-ui/bare.css'

import { v4 as uuidv4 } from 'uuid'
import { addableItems } from './addable-items'
export const maxWidth = 10
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
  [key: number]: {
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
          item
        }
        const gridPosAndSize = gridHelp.item({
          x: 0,
          y: index,
          w: 1,
          h: 1
        })

        Array.from(Array(maxWidth + 1)).forEach((_, i) => {
          insertItem[i] = gridPosAndSize
        })

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
