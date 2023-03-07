<script lang="ts">
  import { setContext } from 'svelte'
  
  import 'svelte-material-ui/bare.css'
  import LayoutGrid, { Cell } from '@smui/layout-grid'
  import Fab, { Label, Icon } from '@smui/fab'
  
  import DeviceCard from './DeviceCard.svelte'
  import { configForm } from '../store/index'
  import Select, { Option } from '@smui/select'
  import { getConfigs, type DeviceConfigExt } from '../utils/configs'
  import { getDevices, type DeviceExt } from '../utils/devices'
  
  let configs: DeviceConfigExt[] = []
  let devices: DeviceExt[] = []
  let virtualDevices: {
    device: DeviceExt,
    config: DeviceConfigExt
  }[] = []
  
  let currentDeviceId = 'all'

  async function freshConfigs () {
    let queryDeviceId : undefined | string = currentDeviceId
    if (currentDeviceId === 'all') {
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
    })


    // const queriedDevices = await getConfigs(currentDeviceId)

    // const currentSortKey = currentSort.split('_')[0]
    // const currentSortOrder = currentSort.split('_')[1] === 'asc' ? 1 : -1

    // devices = queriedDevices.sort((a, b) => {
    //   if (a[currentSortKey] == null && b[currentSortKey] == null) return 0
    //   if (a[currentSortKey] == null) return 1 * currentSortOrder
    //   if (b[currentSortKey] == null) return -1 * currentSortOrder

    //   const aValue = new Date(a[currentSortKey]).getTime()
    //   const bValue = new Date(b[currentSortKey]).getTime()

    //   console.log(aValue - bValue)
    //   console.log((aValue - bValue) * currentSortOrder)

    //   return (aValue - bValue) * currentSortOrder
    // })
  }

  let currentSort = 'createdAt'

  $: (currentSort || currentDeviceId) && (function () {
    freshConfigs()
  })()

  setContext('freshDevices', freshConfigs)

  // freshDevices()

  // freshConfigs()

</script>

<div>
  <LayoutGrid style="
    margin: 0 ;
    padding-bottom: 0;

  ">
    <Cell align="middle"  spanDevices={{ desktop: 6, tablet: 2, phone: 1 }}>

      <Fab  color="primary" on:click={() => {
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
      }} extended  ripple={false}>
        <Icon  class="material-icons">add</Icon>
        <Label>Add</Label>
      </Fab>
      
    </Cell>

    <Cell align="middle"  spanDevices={{ desktop: 6, tablet: 6, phone: 4 }}
     style="display: flex; align-items: center; justify-content: flex-end;">
      
     <Select 
      bind:value={currentDeviceId}
      label="filter by device" variant="outlined" style="width: min(calc(50% - 56px) , 200px);">
        <Option value={'all'}>{'All'}</Option>
        {#each devices as device}
          <Option value={device.id}>{device.name}</Option>
        {/each}
      </Select>

      <Select
        style="margin: 0 0.5em; width: min(calc(50% - 56px) , 200px);"
        variant="outlined"
        label="Sort by"
        bind:value={currentSort}
      >
        <Option value="createdAt">Created at</Option>
        <Option value="createdAt_asc">Created at (asc)</Option>
        <Option value="updatedAt">Updated at</Option>
        <Option value="updatedAt_asc">Updated at (asc)</Option>
        <Option value="lastSeenAt">Last seen at</Option>
        <Option value="lastSeenAt_asc">Last seen at (asc)</Option>
      </Select>



      <!-- fix width -->
      <Fab  
      
      style="min-width: 56px;"
      color="primary" on:click={freshConfigs} ripple={false}>
        <Icon  class="material-icons">refresh</Icon>
      </Fab>
     

    </Cell>
    

    
  </LayoutGrid>
  <LayoutGrid>
    {#each virtualDevices as vd}
      <Cell>
        <DeviceCard config={vd.config} device={vd.device} />
      </Cell>
    {/each}


  </LayoutGrid>

</div>