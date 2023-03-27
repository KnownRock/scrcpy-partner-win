
<div class="record-container">
  <div class="record-panel">
    <div class="record-info">    
      <h1>
        {$t('Record Script')}
      </h1>
      <Textfield
        label={$t('Script Name')}
        bind:value={name}
      />
      <!-- isRecording -->
      <FormField>
        <Checkbox bind:checked={isRecording} />
        <span slot="label">{$t('Recording')}</span>
      </FormField>


      <FormField>
        <Checkbox bind:checked={withMotion} />
        <span slot="label">{$t('With Motion Record')}</span>
      </FormField>
      
    </div>

    <TabBar bind:active={opActive} />
    {#if opActive === 'keyEvents'}
      <KeyEvents  execute={execute}/>
    {:else if opActive === 'Apps'}
      <Apps execute={execute} adbId={adbId}/>
    {:else if opActive === 'operations'}
    <div class="record-operations">
      <Operations operations={operations}  execute={executeWithoutAdd} />
    </div>
      
    {/if}

    <!-- to end -->
    <div class="record-save">
      <Button>
        Reset
      </Button>
      <Button variant="outlined" on:click={play}>
        Play
      </Button>
      <Button variant="raised">
        Save
      </Button>
    </div>
    
  </div>
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <RecordInterface execute={execute} adbId={adbId} />
</div>


<script lang="ts">
  import { t } from 'svelte-i18n'
  import 'svelte-material-ui/bare.css'


  import { getConfigId, setRecordPanelWithMotionRecord } from '../utils/app'
  import { onMount } from 'svelte'
  import { getConfigById } from '../utils/configs'
  import prismaClientLike from '../utils/prisma-like-client'
  import { appWindow } from '@tauri-apps/api/window'
  import TabBar from './Record/TabBar.svelte'
  import KeyEvents from './Record/KeyEvents.svelte'
  import type{ RecordOperation } from '../types'

  import ScrcpyControlClient from './Record/ScrcpyControlClient'
  import Operations from './Record/Operations.svelte'
  import Textfield from '@smui/textfield'
  import Button from '@smui/button'
  import FormField from '@smui/form-field'
  import Checkbox from '@smui/checkbox'
  import Apps from './Record/Apps.svelte'
  import RecordInterface from './Record/RecordInterface.svelte'
  let withMotion = false
  let isRecording = true

  let name = ''
  let adbId = ''


  let opActive

  $: setRecordPanelWithMotionRecord(withMotion)

  let scrcpyControlClient: ScrcpyControlClient | null = null


  const record : RecordOperation[] = []


  let operations = record
  let lastTime = null as null | number
  function addOperation (operation: RecordOperation) {
    if (lastTime == null) {
      lastTime = Date.now()
    } else {
      operations.push({
        type: 'delay',
        ms: Date.now() - lastTime
      })
    }

    operations.push(operation)
  
    operations = operations
  }
  
  function executeWithoutAdd (operation: RecordOperation) {
    scrcpyControlClient?.execute(operation)
  }

  function play () {
    for (let i = 0; i < operations.length; i++) {
      const operation = operations[i]
      scrcpyControlClient?.execute(operation)
    }
  }


  function execute (operation: RecordOperation) {
    scrcpyControlClient?.execute(operation)

    addOperation(operation)
  }

  onMount(async () => {
    const configId = await getConfigId()
    const config = await getConfigById(configId)
    if (config == null) {
      appWindow.close()

      return
    }

    const device = await prismaClientLike.device.findUnique({
      where: {
        id: config.deviceId
      }
    })

    console.log('config', config)
    console.log('device', device)

    if (device == null) {
      appWindow.close()

      return
    }

    adbId = device.adbId

    // listen tauri event
    scrcpyControlClient = new ScrcpyControlClient({ adbId })
    await scrcpyControlClient.init()

    return () => {
      scrcpyControlClient?.close()
    }
  })

</script>

<style>
  .record-container {
    
    margin: 0;
    background-color: transparent;
    overflow: hidden;
  }

  /* FIXME: temp */
  :global(#app, body, html){
    height: 100%;
    background-color: transparent;
  }
  
  .record-container{

    /* width: 100vw; */

    height: calc(100vh - 2px);
    border: 1px solid #ccc;

    display: flex;
  }

  .record-panel{
    height: 100vh;
    display: flex;
    flex-direction: column;
    width: 600px;
    background-color: rgb(240, 240, 240);
    overflow: hidden;
  }

  .record-operations{
    overflow: auto;
  }

  .record-info{
    padding:10px;
  }



  .record-save{
    display: flex;
    flex-direction: column;
    justify-items: center;
    padding: 10px;
    margin-top: auto;
  }
</style>

