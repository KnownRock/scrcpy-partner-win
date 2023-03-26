
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
      <Operations operations={operations} />
    </div>
      
    {/if}

    <!-- to end -->
    <div class="record-save">
      <Button>
        Reset
      </Button>
      <Button variant="raised">
        Save
      </Button>
    </div>
    
  </div>
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <div class="record-interface" 
    bind:clientHeight={height}
    bind:clientWidth={width}
    on:mousedown={handleMouseDown}
    on:mouseup={handleMouseUp}
    on:mousemove={throttleMouseMove}
    on:contextmenu={handleContextMenu}
    on:mouseleave={handleMouseLeave}
    >
  </div>
</div>


<script lang="ts">
  import { t } from 'svelte-i18n'
  import 'svelte-material-ui/bare.css'


  import { getConfigId, getDeviceSize, setRecordPanelWithMotionRecord } from '../utils/app'
  import { onMount } from 'svelte'
  import { getConfigById } from '../utils/configs'
  import prismaClientLike from '../utils/prisma-like-client'
  import { appWindow } from '@tauri-apps/api/window'
  import TabBar from './Record/TabBar.svelte'
  import KeyEvents from './Record/KeyEvents.svelte'
  import { type RecordOperationWithTime, type RecordOperation, KeyEventType } from '../types'
  import { MotionType } from '../types'

  import ScrcpyControlClient from './Record/ScrcpyControlClient'
  import Operations from './Record/Operations.svelte'
  import Textfield from '@smui/textfield'
  import Button from '@smui/button'
  import FormField from '@smui/form-field'
  import Checkbox from '@smui/checkbox'
  import Apps from './Record/Apps.svelte'

  let withMotion = false
  let isRecording = true

  let name = ''

  let height = 0
  let width = 0
  let adbId = ''
  let deviceSize = [0, 0]

  let opActive

  $: setRecordPanelWithMotionRecord(withMotion)

  let scrcpyControlClient: ScrcpyControlClient | null = null


  let operations : RecordOperationWithTime[] = []


  function handleContextMenu (e: MouseEvent) {
    execute({
      type: 'keyevent',
      key: 4,
      keyEventType: KeyEventType.Down
    })

    execute({
      type: 'keyevent',
      key: 4,
      keyEventType: KeyEventType.Up
    })

    e.preventDefault()
  }

  function addOperation (operation: RecordOperation) {
    operations.push({
      ...operation,
      time: new Date()
    })
    operations = operations
  }


  function execute (operation: RecordOperation) {
    scrcpyControlClient?.execute(operation)

    addOperation(operation)
  }


  function getXYFromEvent (e: MouseEvent) {
    const [deviceWidth, deviceHeight] = deviceSize
    const x = ~~(e.offsetX / width * deviceWidth)
    const y = ~~(e.offsetY / height * deviceHeight)
    return [x, y]
  }

  let isMouseDown = false
  async function handleMouseDown (e: MouseEvent) {
    if (e.button !== 0) return

    console.log('mousedown', e)
    isMouseDown = true
    const [x, y] = getXYFromEvent(e)
    execute({
      type: 'motion',
      motionType: MotionType.Down,
      x,
      y
    })
    // addMouseEvent(MotionType.Down, x, y)

    // if (motionAdbShell) motionAdbShell.write(`input motionevent down ${x} ${y}\n`)
  }

  async function handleMouseUp (e: MouseEvent) {
    if (e.button !== 0) return

    console.log('mouseup', e)
    isMouseDown = false
    const [x, y] = getXYFromEvent(e)
    // addMouseEvent(MotionType.Up, x, y)
    execute({
      type: 'motion',
      motionType: MotionType.Up,
      x,
      y
    })
    // if (motionAdbShell) motionAdbShell.write(`input motionevent up ${x} ${y}\n`)
  }

  async function handleMouseLeave (e: MouseEvent) {
    if (e.button !== 0) return

    console.log('mouseleave', e)
    isMouseDown = false
    const [x, y] = getXYFromEvent(e)
    execute({
      type: 'motion',
      motionType: MotionType.Up,
      x,
      y
    })
  }

  const throttle = (fn: Function, delay: number) => {
    let lastCall = 0
    return function (...args: any[]) {
      const now = (new Date()).getTime()
      if (now - lastCall < delay) {
        return
      }
      lastCall = now
      return fn(...args)
    }
  }


  async function handleMouseMove (e: MouseEvent) {
    if (!isMouseDown) return
    console.log('mousemove', e)
    const [x, y] = getXYFromEvent(e)
    // addMouseEvent(MotionType.Move, x, y)
    execute({
      type: 'motion',
      motionType: MotionType.Move,
      x,
      y
    })
  }

  const throttleMouseMove = throttle(handleMouseMove, 10)


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

    deviceSize = await getDeviceSize(device.adbId)
    console.log('deviceSize', deviceSize)

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

  .record-interface{
    flex: 1;
    background-color: transparent;
  }

  .record-save{
    display: flex;
    flex-direction: column;
    justify-items: center;
    padding: 10px;
    margin-top: auto;
  }
</style>

