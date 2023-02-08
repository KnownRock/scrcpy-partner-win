<script lang="ts">
  import Devices from './lib/Devices.svelte'
  import "svelte-material-ui/bare.css"
  import "./style.css"


  import Tab, { Icon, Label } from '@smui/tab';
  import TabBar from '@smui/tab-bar';

  type TabEntry = {
    k: string;
    label: string;
  };

  const key = (tab: TabEntry) => tab.k;

  const tabs: TabEntry[] = [
    {
      k: 'config',
      label: 'Config',
    },
    {
      k: 'devices',
      label: 'Devices',
    },
    {
      k: 'components',
      label: 'Components',
    },
  ];
  let active = tabs[1];

  
</script>

<main class="container">
  <!-- <h1>Main</h1> -->

  <TabBar {tabs} let:tab {key} bind:active> 
    <Tab
    
      {tab}
      stacked={true}
      indicatorSpanOnlyContent={true}
      tabIndicator$transition="fade"
    >
      <h1>{tab.label}</h1>
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
  <!-- <link rel="stylesheet" href="node_modules/svelte-material-ui/bare.css" /> -->
</main>

<style>

</style>                                                                        