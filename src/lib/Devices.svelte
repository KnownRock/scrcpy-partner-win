<script lang="ts">
  import { setContext } from 'svelte'
  
  import 'svelte-material-ui/bare.css'
  import LayoutGrid, { Cell } from '@smui/layout-grid'
  import Fab, { Label, Icon } from '@smui/fab'
  import { getDevices, type DeviceExt } from '../utils/devices'
  
  import DeviceCard from './DeviceCard.svelte'
  import { deviceForm } from '../store/index'
  import Select, { Option } from '@smui/select'
  
  export let queryType: Parameters<typeof getDevices>[0] = 'all'
  let devices : DeviceExt[] = []

  async function freshDevices () {
    const queriedDevices = await getDevices(queryType)

    const currentSortKey = currentSort.split('_')[0]
    const currentSortOrder = currentSort.split('_')[1] === 'asc' ? 1 : -1

    devices = queriedDevices.sort((a, b) => {
      if (a[currentSortKey] == null && b[currentSortKey] == null) return 0
      if (a[currentSortKey] == null) return 1 * currentSortOrder
      if (b[currentSortKey] == null) return -1 * currentSortOrder

      const aValue = new Date(a[currentSortKey]).getTime()
      const bValue = new Date(b[currentSortKey]).getTime()

      console.log(aValue - bValue)
      console.log((aValue - bValue) * currentSortOrder)

      return (aValue - bValue) * currentSortOrder
    })
  }

  let currentSort = 'createdAt'

  $: currentSort && (function () {
    freshDevices()
  })()

  setContext('freshDevices', freshDevices)

  freshDevices()

</script>

<div>
  <LayoutGrid style="
    margin: 0.5em;
  ">
    <Cell align="middle"  spanDevices={{ desktop: 6, tablet: 4, phone: 1 }}>

      <Fab  color="primary" on:click={() => {
        deviceForm.set({
          show: true,
          device: {
            id: '',
            name: '',
            adbId: '',
            model: '',
            product: '',
            createdAt: null,
            updatedAt: null,
            lastSeenAt: null
          },
          callback: async (device) => {
            await freshDevices()
          }
        })
      }} extended  ripple={false}>
        <Icon  class="material-icons">add</Icon>
        <Label>Add</Label>
      </Fab>
      
    </Cell>

    <Cell align="middle"  spanDevices={{ desktop: 6, tablet: 4, phone: 3 }}
     style="display: flex; align-items: center; justify-content: flex-end;">
      <!-- sort by createdAt or updatedAt or lastSeenAt -->
      
      <Select
        style="margin-right: 0.5em; width: min(calc(100% - 56px) , 200px);"
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
      color="primary" on:click={freshDevices} ripple={false}>
        <Icon  class="material-icons">refresh</Icon>
      </Fab>
     

    </Cell>
    

    
  </LayoutGrid>
  <LayoutGrid>
    {#each devices as device}
      <Cell>
        <DeviceCard 
          device={device}
         />
        
      </Cell>
    {/each}

  </LayoutGrid>

</div>