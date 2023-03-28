<Form />
<div class="record-container">
  <div class="record-panel">
    <div class="record-info">  
      <div style="display:flex; align-items: center; justify-content: space-between;">
        <h1>
          {$t('Record Script')}
        </h1>
        
        <IconButton class="material-icons" on:click={close}>
          close
        </IconButton>
      </div>  
   
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

      <div>
        <Textfield
          style="width: 100%;"
          label={$t('Script Description')}
          bind:value={description}
        />
      </div>

      <div style="display: flex; align-items: center; justify-content: space-between;">
        <!-- TODO: make this better -->
        <Slider style="width: 60%;" 
          label={$t('Speed')}
          min={0.1} max={10} step={0.1}
        bind:value={scale} />
        
        <Label>
          {$t('Speed')} {scale}x
        </Label>

      </div>
      
      
    </div>

    <TabBar bind:active={opActive} />
    {#if opActive === 'keyEvents'}
      <KeyEvents  execute={execute}/>
    {:else if opActive === 'apps'}
      <Apps execute={execute} adbId={adbId}/>
    {:else if opActive === 'operations'}
      <div class="record-operations">
        <Operations bind:selection={selection} operations={operations}  execute={executeWithoutAdd} />
      </div>
    {:else if opActive === 'scripts'}
      <div>
        <Button on:click={() => openSelecter('select')}>
          Open
        </Button>

      </div>
      
    {/if}

    <!-- to end -->
    <div class="record-save">
      

      <!-- <Button>
        {$t('Reset')}
      </Button> -->
      <Button  on:click={play}>
        {$t('Play')} 
      </Button>
      <Button on:click={createNew}>
        {$t('New')}
      </Button>

      <Button  on:click={() => openSelecter()}>
        {$t('Open')}
      </Button>
      <Button variant="raised" on:click={save}>
        {$t('Save')}
      </Button>
      
    </div>
    
  </div>
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <RecordInterface execute={execute} adbId={adbId} />
</div>


<script lang="ts">
  import { t } from 'svelte-i18n'
  import 'svelte-material-ui/bare.css'

  import { generalDialogForm } from '../store'
  import Slider from '@smui/slider'

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
  import Button, { Label } from '@smui/button'
  import FormField from '@smui/form-field'
  import Checkbox from '@smui/checkbox'
  import Apps from './Record/Apps.svelte'
  import RecordInterface from './Record/RecordInterface.svelte'
  import { v4 as uuidv4 } from 'uuid'
  import IconButton from '@smui/icon-button'
  import Form from './general/Form.svelte'

  let withMotion = false
  let isRecording = true

  let adbId = ''
  let selection: [number, number] = [-1, -1]

  let name = ''
  let recordId = ''
  let description = ''
  let operations : RecordOperation[] = []
  let scale = 1.0
  // id = id

  function close () {
    // generalDialogForm.set(null)
  }

  function createNew () {
    operations = []
    name = ''
    description = ''
    recordId = ''
    scale = 1.0

    selection = [-1, -1]
  }

  async function openSelecter (mode: 'open' | 'select' = 'open') {
    const buttonsDict = {
      open: [{
        label: 'Open',
        callback: async (record) => {
          await load(record.id)
          generalDialogForm.set({
            show: false,
            formItems: [],
            buttons: [],
            cancelCallback: () => {
              return true
            }
          })
        }
      }, {
        label: 'Delete',
        callback: async (record) => {
          await prismaClientLike.recordScript.delete({
            where: {
              id: record.id
            }
          })
          setDialog()
        }
      }],

      select: [{
        label: 'Add',
        callback: async (record) => {
          operations.push({
            type: 'exec_script',
            scriptId: record.id,
            name: record.name
          })
        }
      }]
    }

    const buttons = buttonsDict[mode]

    async function setDialog () {
      const records = await prismaClientLike.recordScript.findMany({})
      generalDialogForm.set({
        title: 'Record',
        show: true,
        buttons: [{
          label: 'Cancel',
          action: 'cancel',
          callback: () => {
            return true
          }
        }],
        formItems: [
          {
            type: 'table',
            columns: [
              {
                label: 'Name',
                name: 'name'
              },
              {
                label: 'Description',
                name: 'description'
              },
              {
                label: 'Actions',
                name: 'actions',
                buttons
              }
            ],
            label: 'Record Scripts',
            name: 'recordScripts',

            value: records as Array<Record<string, any>>
          }
        ],
        cancelCallback: () => {
          return true
        }
      })
    }

    setDialog()
  }


  async function load (id) {
    createNew()

    const recordScript = await prismaClientLike.recordScript.findUnique({
      where: {
        id
      },
      include: {
        recordScript: true
      }
    })

    if (recordScript == null) {
      return
    }

    name = recordScript.name
    description = recordScript.description
    recordId = recordScript.id
    operations = JSON.parse(recordScript.recordScript.data)
    scale = recordScript.scale

    const scriptIds = operations
      .map((op) => op.type === 'exec_script' ? op.scriptId : null)
      .filter((id) => id != null)

    const scripts = await prismaClientLike.recordScript.findMany({
      where: {
        id: {
          in: scriptIds as string[]
        }
      }
    })


    // TODO: error handling
    const scriptDict = scripts.reduce((dict, script) => {
      dict[script.id] = {
        name: script.name,
        id: script.id
      }
      return dict
    }, {} as Record<string, any>)

    operations = operations.map((op) => {
      if (op.type === 'exec_script') {
        return {
          ...op,
          name: scriptDict[op.scriptId].name
        }
      }
      return op
    })
  }

  async function save () {
    await prismaClientLike.recordScript.upsert({
      where: {
        id: recordId || uuidv4()
      },
      update: {
        name,
        description,
        scale,
        recordScript: {
          create: {
            data: JSON.stringify(operations)
          }
        }
      },
      create: {
        name,
        description,
        scale,
        recordScript: {
          create: {
            data: JSON.stringify(operations)
          }
        }
      }
    })
  }


  let opActive

  $: setRecordPanelWithMotionRecord(withMotion)

  let scrcpyControlClient: ScrcpyControlClient | null = null

  
  let lastTime = null as null | number
  function addOperation (operation: RecordOperation) {
    if (lastTime == null) {
      lastTime = Date.now()
    } else {
      if (selection[1] === -1) {
        operations.push({
          type: 'delay',
          ms: Date.now() - lastTime
        })
        lastTime = Date.now()
      } else {
        operations.splice(selection[1], 0, {
          type: 'delay',
          ms: Date.now() - lastTime
        })
        lastTime = Date.now()
        selection = [selection[0], selection[1] + 1]
      }
    }

    if (selection[1] === -1) {
      operations.push(operation)
    } else {
      operations.splice(selection[1], 0, operation)
      selection = [selection[0], selection[1] + 1]
    }

  
    operations = operations
  }
  
  function executeWithoutAdd (operation: RecordOperation) {
    scrcpyControlClient?.execute(operation, {
      scale
    })
  }

  function play () {
    for (let i = 0; i < operations.length; i++) {
      const operation = operations[i]
      scrcpyControlClient?.execute(operation, {
        scale
      })
    }
  }


  function execute (operation: RecordOperation) {
    scrcpyControlClient?.execute(operation, {
      scale
    })

    if (isRecording) {
      addOperation(operation)
    }
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
    // await scrcpyControlClient.init()

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

