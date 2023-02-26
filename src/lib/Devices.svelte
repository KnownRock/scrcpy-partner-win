<script lang="ts">
  import LayoutGrid, { Cell } from '@smui/layout-grid'

  import { getDevices, type Device } from '../utils/devices'

  import DeviceCard from './DeviceCard.svelte'
  import Form from './general/Form.svelte'

  let devices : Device[] = []

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

  <!-- <Config bind:open={showConfig} currentDeviceId={currentDeviceAdbId} /> -->
  <Form />
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