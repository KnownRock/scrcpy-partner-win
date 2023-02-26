<script lang="ts">
  import Devices from './Devices.svelte'
  import 'svelte-material-ui/bare.css'
  import Tab from '@smui/tab'
  import TabBar from '@smui/tab-bar'
  import { onMount } from 'svelte'
  import { showMainWindow } from '../utils/app'

  type TabEntry = {
    k: string;
    label: string;
  };

  const key = (tab: TabEntry) => tab.k

  const tabs: TabEntry[] = [
    // {
    //   k: 'config',
    //   label: 'Config',
    // },
    {
      k: 'devices',
      label: 'Devices'
    }
    // {
    //   k: 'components',
    //   label: 'Components',
    // },
  ]
  let active = tabs[0]

  onMount(() => {
    showMainWindow()
  })

</script>

<main class="container">
  <TabBar {tabs} let:tab {key} bind:active> 
    <Tab
    
      {tab}
      stacked={true}
      indicatorSpanOnlyContent={true}
      tabIndicator$transition="fade"
    >
      <h1 style="
        font-size:calc(min(3.5vw, 2rem ));
      ">{tab.label}</h1>
    </Tab>
  </TabBar>

  <div class="row">
    {#if active.k === 'config'}
      <h1>Config</h1>
    {/if}
    {#if active.k === 'devices'}
      <Devices />
    {/if}
    {#if active.k === 'components'}
      <h1>Components</h1>
    {/if} 
  </div>
</main>

<style>

</style>                                                                        