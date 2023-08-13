<script lang="ts">
  import { onDestroy, setContext } from 'svelte'
  import 'svelte-material-ui/bare.css'
  import LayoutGrid, { Cell } from '@smui/layout-grid'
  import Fab, { Icon } from '@smui/fab'
  
  import DeviceCard from './DeviceCard.svelte'
  import { configForm } from '../store/index'
  import { getConfigs, type DeviceConfigExt, saveConfig } from '../utils/configs'
  import { getDevices, type DeviceExt } from '../utils/devices'
  import { v4 as uuidv4 } from 'uuid'
  
  let configs: DeviceConfigExt[] = []
  let devices: DeviceExt[] = []
  let virtualDevices: {
    device: DeviceExt,
    config: DeviceConfigExt
  }[] = []
  
  const allDeviceId = uuidv4()
  const currentDeviceId = allDeviceId

  async function freshConfigs () {
    let queryDeviceId : undefined | string = currentDeviceId
    if (currentDeviceId === allDeviceId) {
      queryDeviceId = undefined
    }

    devices = await getDevices('only saved')
    configs = await getConfigs(queryDeviceId)

    const deviceDict = {}
    devices.forEach(device => {
      deviceDict[device.id] = device
    })


    virtualDevices = configs.map(config => {
      return {
        device: deviceDict[config.deviceId],
        config
      }
    }).sort((a, b) => {
      const aValue = +(a.config.order || a.config.createdAt || 0)
      const bValue = +(b.config.order || b.config.createdAt || 1)

      if (isNaN(aValue)) return 1
      if (isNaN(bValue)) return -1

      return aValue - bValue
    })

    console.log(virtualDevices)
  }

  async function handleMoveUp (device: DeviceExt) {
    // alert(123)
    const index = virtualDevices.findIndex(virtualDevice => virtualDevice.device.id === device.id)
    if (index === 0) return
    const virtualDevice = virtualDevices[index]
    const prevVirtualDevice = virtualDevices[index - 1]

    let nowOrder = +(virtualDevice.config.order || virtualDevice.config.createdAt || 0)
    let prevOrder = +(prevVirtualDevice.config.order || prevVirtualDevice.config.createdAt || 1)

    if (isNaN(nowOrder)) nowOrder = 0
    if (isNaN(prevOrder)) prevOrder = nowOrder + 1

    if (prevOrder === nowOrder) {
      prevOrder = nowOrder + 1
    }
  
    virtualDevice.config.order = prevOrder
    prevVirtualDevice.config.order = nowOrder
  
    await Promise.all([
      saveConfig(virtualDevice.config),
      saveConfig(prevVirtualDevice.config)
    ])

    await freshConfigs()
  }

  async function handleMoveDown (device: DeviceExt) {
    const index = virtualDevices.findIndex(virtualDevice => virtualDevice.device.id === device.id)
    if (index === virtualDevices.length - 1) return
    const virtualDevice = virtualDevices[index]
    const nextVirtualDevice = virtualDevices[index + 1]

    let nowOrder = +(virtualDevice.config.order || virtualDevice.config.createdAt || 0)
    let nextOrder = +(nextVirtualDevice.config.order || nextVirtualDevice.config.createdAt || 1)

    if (isNaN(nowOrder)) nowOrder = 0
    if (isNaN(nextOrder)) nextOrder = nowOrder + 1

    if (nextOrder === nowOrder) {
      nextOrder = nowOrder + 1
    }
  
    virtualDevice.config.order = nextOrder
    nextVirtualDevice.config.order = nowOrder
  
    await Promise.all([
      saveConfig(virtualDevice.config),
      saveConfig(nextVirtualDevice.config)
    ])

    await freshConfigs()
  }


  const timer = setInterval(() => {
    freshConfigs()
  }, 1000 * 5)

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
  <div style="
    position: fixed;
    bottom: 20px;
    right: 20px;
  ">
    <Fab  
      style="min-width: 56px;"
      color="primary" on:click={
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
            

  </div>

</div>