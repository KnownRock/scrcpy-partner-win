<script lang="ts">
  import { t } from 'svelte-i18n'
  import Devices from './Devices.svelte'
  import Configs from './Configs.svelte'
  import 'svelte-material-ui/bare.css'
  import Tab from '@smui/tab'
  import TabBar from '@smui/tab-bar'
  import { onMount } from 'svelte'
  import Form from './general/Form.svelte'
  import Message from './general/Message.svelte'
  import Loading from './general/Loading.svelte'
  import { appWindow } from '@tauri-apps/api/window'

  type TabEntry = {
    k: string;
    label: string;
  };

  const key = (tab: TabEntry) => tab.k

  const tabs: TabEntry[] = [

    {
      k: 'config',
      // label: 'Config'
      label: $t('Config')
    },
    {
      k: 'devices',
      // label: 'Devices'
      label: $t('Devices')
    },
    {
      k: 'adb',
      // label: 'Adb Devices'
      label: $t('ADB Devices')
    }
  ]
  let active = tabs[0]

  onMount(() => {
    appWindow.show()
    appWindow.setTitle($t('Scrcpy Partner Home'))
  })

</script>


<main class="container">
  <Form />
  <Message />
  <Loading />
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
      <Configs />
    {/if}
    {#if active.k === 'devices'}
      <Devices />
    {/if}
    {#if active.k === 'adb'}
      <Devices queryType="only adb" />
    {/if}
  </div>
</main>


<style>

</style>                                                                        