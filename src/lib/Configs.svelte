<script lang="ts">
  import { onDestroy, setContext } from 'svelte'
  import 'svelte-material-ui/bare.css'
  import LayoutGrid, { Cell } from '@smui/layout-grid'
  import Fab, { Icon } from '@smui/fab'
  
  import DeviceCard from './DeviceCard.svelte'
  import { configForm } from '../store/index'
  import { getConfigs, type DeviceConfigExt, saveConfig } from '../utils/configs'
  import { getDevices, type DeviceExt, saveDevice } from '../utils/devices'
  import { v4 as uuidv4 } from 'uuid'
  import FabBox from './components/FabBox.svelte'

  export let queryType: Parameters<typeof getDevices>[0] = 'only saved'
  export let pageType:'config' | 'device' = 'config'
  
  let virtualDevices: {
    device: DeviceExt,
    config?: DeviceConfigExt
  }[] = []
  
  const allDeviceId = uuidv4()
  const currentDeviceId = allDeviceId

  type CustomSortable = {
    order: number | null,
    createdAt: Date | null
  }

  function sortOrder (a: CustomSortable, b: CustomSortable) {
    const aValue = +(a.order || a.createdAt || 0)
    const bValue = +(b.order || b.createdAt || 1)

    if (isNaN(aValue)) return 1
    if (isNaN(bValue)) return -1

    return aValue - bValue
  }


  function sortVirtualDevices () {
    if (pageType === 'config') {
      virtualDevices = virtualDevices.sort((a, b) => {
        if (a.config && b.config) {
          return sortOrder(a.config, b.config)
        } else {
          return 0
        }
      })
    } else {
      virtualDevices = virtualDevices.sort((a, b) => {
        return sortOrder(a.device, b.device)
      })
    }
  }

  async function freshConfigs () {
    let queryDeviceId : undefined | string = currentDeviceId
    if (currentDeviceId === allDeviceId) {
      queryDeviceId = undefined
    }

    const devices = await getDevices(queryType)
    if (pageType === 'config') {
      const configs = await getConfigs(queryDeviceId)
      const deviceDict = {}
      devices.forEach(device => {
        deviceDict[device.id] = device
      })

      virtualDevices = configs.map(config => {
        const device = deviceDict[config.deviceId]
        return {
          device,
          config
        }
      })
    } else {
      virtualDevices = devices.map(device => {
        return {
          device
        }
      })
    }
  
  
    sortVirtualDevices()
  }

  // TODO: only update order
  async function saveOrder (virtualDevice: typeof virtualDevices[0], prevVirtualDevice: typeof virtualDevices[0]) {
    if (pageType === 'config') {
      if (virtualDevice.config && prevVirtualDevice.config) {
        await Promise.all([
          saveConfig(virtualDevice.config),
          saveConfig(prevVirtualDevice.config)
        ])
      }
    } else {
      await Promise.all([
        saveDevice(virtualDevice.device),
        saveDevice(prevVirtualDevice.device)
      ])
    }
  }

  async function handleMoveUp (id: string) {
    const index = virtualDevices.findIndex(virtualDevice => (virtualDevice?.config?.id ?? virtualDevice.device.id) === id)
    if (index === 0) return
    const virtualDevice = virtualDevices[index]
    const prevVirtualDevice = virtualDevices[index - 1]

    const item = virtualDevice.config ?? virtualDevice.device
    const prevItem = prevVirtualDevice.config ?? prevVirtualDevice.device

    let nowOrder = +(item.order || item.createdAt || 0)
    let prevOrder = +(prevItem.order || prevItem.createdAt || 1)

    if (isNaN(nowOrder)) nowOrder = 0
    if (isNaN(prevOrder)) prevOrder = nowOrder + 1

    if (prevOrder === nowOrder) {
      prevOrder = nowOrder + 1
    }
  
    item.order = prevOrder
    prevItem.order = nowOrder

    sortVirtualDevices()
    saveOrder(virtualDevice, prevVirtualDevice)
    await freshConfigs()
  }

  async function handleMoveDown (id: string) {
    const index = virtualDevices.findIndex(virtualDevice => (virtualDevice?.config?.id ?? virtualDevice.device.id) === id)
    if (index === virtualDevices.length - 1) return
    const virtualDevice = virtualDevices[index]
    const nextVirtualDevice = virtualDevices[index + 1]

    const item = virtualDevice.config ?? virtualDevice.device
    const nextItem = nextVirtualDevice.config ?? nextVirtualDevice.device

    let nowOrder = +(item.order || item.createdAt || 0)
    let nextOrder = +(nextItem.order || nextItem.createdAt || 1)

    if (isNaN(nowOrder)) nowOrder = 0
    if (isNaN(nextOrder)) nextOrder = nowOrder + 1

    if (nextOrder === nowOrder) {
      nextOrder = nowOrder + 1
    }
  
    item.order = nextOrder
    nextItem.order = nowOrder

    sortVirtualDevices()

    saveOrder(virtualDevice, nextVirtualDevice)

    await freshConfigs()
  }


  const timer = setInterval(() => {
    freshConfigs()
  }, 1000 * 10)

  onDestroy(() => {
    clearInterval(timer)
  })


  $: (currentDeviceId) && (function () {
    freshConfigs()
  })()

  setContext('freshDevices', freshConfigs)

</script>

<div style="height:100%">
  <LayoutGrid style="height:calc(100%); overflow-y: auto;">
    {#each virtualDevices as vd, index}
      <Cell>
        <DeviceCard 
          config={vd.config} 
          device={vd.device}

          onMoveUp={handleMoveUp}
          onMoveDown={handleMoveDown}

          canMoveUp={index !== 0}
          canMoveDown={index !== virtualDevices.length - 1}
        />
      </Cell>
    {/each}
  </LayoutGrid>
  <!-- add fab button -->
  <FabBox>
    <Fab  
      style="min-width: 56px;"
      color="primary" 
      on:click={
        () => {
        configForm.set({
          show: true,
          type: 'new',
          callback: () => {
            // FIXME: this is a hack
            setTimeout(() => {
              freshConfigs()
            }, 500)
          }
        })
      }
      } ripple={false}>
      <Icon  class="material-icons">
        add
      </Icon>
    </Fab>
  </FabBox>

</div>