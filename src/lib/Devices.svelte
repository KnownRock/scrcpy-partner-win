<script lang="ts">
  import LayoutGrid, { Cell } from '@smui/layout-grid'

  import { getDevices, type Device } from '../utils/devices'

  import Config from './Config.svelte'
  import DeviceCard from './DeviceCard.svelte'

  let devices : Device[] = []

  let showConfig = false

  let currentDeviceAdbId : string = ''

  async function setDevices () {
    devices = await getDevices()
  }

  setDevices()

</script>

<div>

  <!-- <div style="display: flex; justify-content: space-between; align-items: flex-end;">
    <h1 style="margin-bottom:0.3em;">Device list</h1>
    <Button on:click={setDevices}>
      refresh
    </Button>

  </div> -->

  <Config bind:open={showConfig} currentDeviceId={currentDeviceAdbId} />
  
  <LayoutGrid>
    {#each devices as device}
      <Cell>
        <DeviceCard 
          bind:showConfig={showConfig} bind:currentDeviceId={currentDeviceAdbId}
          device={device}
         />
        
      </Cell>
    {/each}

  </LayoutGrid>

</div>