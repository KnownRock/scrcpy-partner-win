<script lang="ts">
  import { setContext, onDestroy } from 'svelte'
  import { t } from 'svelte-i18n'
  import 'svelte-material-ui/bare.css'
  import LayoutGrid, { Cell } from '@smui/layout-grid'
  import Fab, { Label, Icon } from '@smui/fab'
  import { getDevices, type DeviceExt } from '../utils/devices'
  
  import DeviceCard from './DeviceCard.svelte'
  import { deviceForm } from '../store/index'
  import Select, { Option } from '@smui/select'
  import FabBox from './components/FabBox.svelte'
  
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

      return (aValue - bValue) * currentSortOrder
    })
  }

  const timer = setInterval(() => {
    freshDevices()
  }, 1000 * 5)

  onDestroy(() => {
    clearInterval(timer)
  })


  const currentSort = 'createdAt'

  $: currentSort && (function () {
    freshDevices()
  })()

  setContext('freshDevices', freshDevices)

  freshDevices()

</script>

<div>
  <LayoutGrid>
    {#each devices as device}
      <Cell>
        <DeviceCard 
          device={device}
         />
        
      </Cell>
    {/each}

  </LayoutGrid>
  <FabBox>
    <Fab  
    style="min-width: 56px;"
    color="primary" 
    on:click={() => {
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
          lastSeenAt: null,
          order: null
        },
        callback: async () => {
          await freshDevices()
        }
      })
    }}  ripple={false}>
      <Icon  class="material-icons">add</Icon>
    </Fab>
  </FabBox>

</div>