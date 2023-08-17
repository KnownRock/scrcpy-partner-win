
<script lang="ts">
  import 'svelte-material-ui/bare.css'
  import IconButton, { Icon } from '@smui/icon-button'
  import { onMount } from 'svelte'
  import { exit, getConfigId, open, start } from '../utils/app'
  import Grid from 'svelte-grid'
  import { commandKeyDict } from './Tool/command-key-dict'
  import { lanuchSelf } from '../utils/devices'

  import Setting from './Tool/Setting.svelte'
  import { getConfigWithSidebarConfig, getDefaultSidebarConfig, getInitConfig } from './Tool/config'
  import gridHelp from 'svelte-grid/build/helper/index.mjs'
  import TabBar from '@smui/tab-bar'
  import Tab from '@smui/tab'
  import { callTauriFunction } from '../utils/tauri'
  import { t } from 'svelte-i18n'
  import { appWindow, LogicalSize } from '@tauri-apps/api/window'
  import ScrcpyControlClient from '../utils/ScrcpyControlClient'
  import prismaClientLike from '../utils/prisma-like-client'
  import Loading from './general/Loading.svelte'
  import Form from './general/Form.svelte'

  import { window } from '@tauri-apps/api'
  import { TauriEvent } from '@tauri-apps/api/event'

  window.getCurrent().listen(TauriEvent.WINDOW_CLOSE_REQUESTED, () => {
    exit()
  })

  const componentDict = {
    setting: Setting
  }

  type Application = {
    label: string
    id: keyof typeof componentDict
  }

  let adbId = ''

  function setToolWindowSize (width: number, height: number) {
    appWindow.setSize(new LogicalSize(width, height))
  }


  let sidebarConfig = getInitConfig()
  
  
  async function sendKey (command: string) {
    const opt = commandKeyDict[command]
    if (opt) {
      await callTauriFunction('sendkey', opt)
    }
  }


  const applications = [] as Application[]
  let currentApplication = null as Application | null
  function addApplication (
    application: Application
  ) {
    const index = applications.findIndex((a) => a.id === application.id)
    if (index === -1) {
      applications.push(application)
    }

    currentApplication = application
  }

  function removeApplication (
    applicationId: Application['id'] | null
  ) {
    if (applicationId === null) {
      return
    }

    const index = applications.findIndex((a) => a.id === applicationId)
    if (index !== -1) {
      applications.splice(index, 1)
    }

    if (currentApplication?.id === applicationId) {
      currentApplication = applications[index + 1] ?? applications[index - 1] ?? null
    }

    if (applicationId === 'setting') {
      mode = 'normal'
    }
  }

  let mode: 'normal' | 'setting' = 'normal'
  async function toggleMode () {
    if (mode === 'normal') {
      mode = 'setting'
      addApplication({
        label: 'Setting',
        id: 'setting'
      })
    } else {
      mode = 'normal'
      removeApplication('setting')
    }
  }

  let gridSize = [1, 1]

  let activeLayer = 0
  $: activeLayer !== undefined && (() => {
    sidebarConfig.activeLayer = +activeLayer
    layerChanged()
  })()

  $: sidebarConfig && (() => {
    layerChanged()
  })()

  function layerChanged () {
    activeLayer = sidebarConfig.activeLayer
    gridSize = sidebarConfig.layers[sidebarConfig.activeLayer].gridSize
    setItemsBySidebarConfig()
  }

  // eslint-disable-next-line
  $: mode, currentApplication, gridSize && (() => {
    gridSize[0] = Math.max(1, gridSize[0])
    gridSize[1] = Math.max(1, gridSize[1])

    const barSize = [
      gridSize[0] * 50,
      gridSize[1] * 50
    ]
  
    gridSize = gridSize

    let newWidth = barSize[0]
    let newHeight = barSize[1]
    if (currentApplication) {
      newHeight = Math.max(600, barSize[1])
      newWidth = barSize[0] + 400
    }


    setToolWindowSize(newWidth + 2, newHeight + 2)
  

    setItemsBySidebarConfig()
  })()

  let items = [] as Array<any>

  function setSidebarConfigItems (items) {
    sidebarConfig.layers[sidebarConfig.activeLayer].items = items.map((i) => {
      return {
        id: i.id,
        coord: i[gridSize[0]],
        item: i.item
      }
    })
  }

  function setItemsBySidebarConfig () {
    items = sidebarConfig.layers[sidebarConfig.activeLayer].items.map((i) => {
      const coord = i.coord ?? gridHelp.item({
        x: 0,
        y: 0,
        w: 1,
        h: 1
      })

      const coordWithEnableState = {
        ...coord,
  
        ...(mode === 'normal'
          ? {
              fixed: true,
              resizable: false,
              draggable: false
            }
          : {
              fixed: false,
              resizable: true,
              draggable: true
            })
      }

      return {
        id: i.id,
        [gridSize[0]]: coordWithEnableState,
        item: i.item
      }
    })
  }

  $: items && (() => {
    items.forEach((item) => {
      const i = gridSize[0]
      const bottom = item[i].y + item[i].h
      if (bottom > gridSize[1]) {
        item[i].y = gridSize[1] - item[i].h

        if (item[i].y < 0) {
          item[i].y = 0
        }

        if (item[i].h > gridSize[1]) {
          item[i].h = gridSize[1]
        }
      }
    })


    setSidebarConfigItems(items)
  })()


  let currentConfigId = ''
  let currentSideBarConfigId = '' as string// uuidv4()
  onMount(async () => {
    appWindow.show()
    const configId = await getConfigId()
    console.log('configId', configId)

    currentConfigId = configId

    const config = await getConfigWithSidebarConfig(currentConfigId)
    console.log('config', config)

    if (config) {
      const device = config.device
      adbId = device.adbId

      currentSideBarConfigId = config.sideBarConfigId ?? ''
      const queriedSidebarConfig = config.sideBarConfig


      if (queriedSidebarConfig) {
        const config = JSON.parse(queriedSidebarConfig.value)
        sidebarConfig = config
      } else {
        sidebarConfig = getDefaultSidebarConfig()
      }
    } else {
      sidebarConfig = getDefaultSidebarConfig()
    }

    if (!scrcpyControlClient) {
      scrcpyControlClient = new ScrcpyControlClient({ adbId })
      scrcpyControlClient.init()
    }

    return () => {
      console.log('unmount')
      scrcpyControlClient?.close()
      scrcpyControlClient = null
    }
  })


  async function execute (item) {
    if (item.cmdType === 'scrcpy-cmd') {
      sendKey(item.cmdName)
    }
    if (item.cmdType === 'scrcpy-control-cmd') {
      scrcpyControlClient?.execute({
        type: item.cmdName,
        key: item.opts.key,
        keyEventType: item.opts.keyEventType
      })
    }
    if (item.cmdType === 'app-cmd') {
      if (item.cmdName === 'exit') {
        exit()
      }
      if (item.cmdName === 'set_layer') {
        activeLayer = item.opts.layer
      }
      if (item.cmdName === 'open_settings') {
        lanuchSelf([])
      }
      if (item.cmdName === 'open') {
        open(item.opts.exec, item.opts.args.split(' ').filter((i) => i), item.opts.cwd)
      }
      if (item.cmdName === 'start') {
        start(item.opts.exec, item.opts.cwd)
      }
      if (item.cmdName === 'exec_script') {
        execScript(item.opts.scriptId)
      }
  

      if (item.cmdName === 'capture') {
        if (scrcpyControlClient) {
          await scrcpyControlClient.execute({
            type: 'adb_cmd',
            cmd: 'mkdir -p /sdcard/ScreenCapture'
          })

          await scrcpyControlClient.execute({
            type: 'adb_cmd',
            cmd: 'screencap -p /sdcard/ScreenCapture/$(date +%Y%m%d%H%M%S).png'
          })
        }
      }
    }
  }
  

  let loadingPromise: Promise<void> | null = null
  let scrcpyControlClient: ScrcpyControlClient | null = null
  async function execScript (scriptId) {
    if (loadingPromise) {
      await loadingPromise
    }

    if (!scrcpyControlClient) {
      loadingPromise = new Promise((resolve) => {
        scrcpyControlClient = new ScrcpyControlClient({ adbId })
        scrcpyControlClient
          .init()
          .then(() => {
            loadingPromise = null
            resolve()
          })
      })

      await loadingPromise
    }

    const script = await prismaClientLike.recordScript.findUnique({
      where: {
        id: scriptId
      },
      include: {
        recordScript: true
      }
    })

    if (script) {
      const operations = JSON.parse(script.recordScript.data)
      for (const operation of operations) {
        await scrcpyControlClient?.execute(operation, {
          scale: script.scale
        })
      }
    }
  }

  function delItem (item) {
    items = items.filter((i) => i.id !== item.id)
    setSidebarConfigItems(items)
  }

</script>



<div class="main-container">
  <div 
  class="tool-container"
  style={
  `width: ${gridSize[0] * 50}px; height: ${gridSize[1] * 50}px; min-width: ${gridSize[0] * 50}px; min-height: ${gridSize[1] * 50}px;`}
  >
    {#if currentConfigId && mode === 'normal'}
      <div 
        style="
        position: absolute; 
        bottom: 0; left: 0; 
        width: 16px; 
        height: 12px; 
         z-index: 1;
        overflow: hidden;
        padding: -5px;
        display: flex; justify-content: center; align-items: center;
        ">
      
        <Icon class="material-icons" 
        on:click={() => { toggleMode() }}
        style="
          color: rgb(96, 125, 139); font-size: 10px;
          width: 15px; height: 15px; padding: 0; margin: 0;
          cursor: pointer;
        ">
          build_circle
        </Icon>
      </div>

    {/if}


    <Grid
      style={`width: ${gridSize[0] * 50}px; height: ${gridSize[1] * 50}px;`}
      bind:items
      fastStart={true}
      gap={[0, 0]}
      rowHeight={50}
      let:item
      let:dataItem
      cols={[[gridSize[1], gridSize[0]]]}
    >
      <div style="
      display: flex; justify-content: center; align-items: center;
      height: 100%; width: 100%;
      "
      on:dblclick={() => {
        if (mode === 'setting') {
          delItem(dataItem)
        }
        }} >

        {#if dataItem.item.uiType === 'icon-button'}
        <IconButton
          disabled={mode === 'setting'}
          class="material-icons" on:click={() => execute(dataItem.item)}>
          {dataItem.item.icon}
        </IconButton> 
        {/if}
        {#if dataItem.item.uiType === 'icon-button-2'}
        <IconButton
          disabled={mode === 'setting'}
          class="material-icons" 
          on:mousedown={() => execute(dataItem.item.cmds[0])}
          on:mouseup={() => execute(dataItem.item.cmds[1])}
        >
          {dataItem.item.icon}
        </IconButton>
        {/if}

      </div>
      
    </Grid>
  
  </div>
  {#if currentApplication}
  <div style="display: flex; height:100%;min-height:600px;
    flex-direction: column;
  ">
    <TabBar 
    tabs={applications}
    let:tab
    key={(tab) => tab?.id ?? ''}
    bind:active={currentApplication}
  >
    {#each applications as app}
    <Tab
    
      {tab}
      stacked={true}
      indicatorSpanOnlyContent={true}
      tabIndicator$transition="fade"
    >
      {$t(app.label)}
      </Tab>
    {/each}


  </TabBar>

  <div style="height: 0"
  >
    <IconButton
      class="material-icons"
      on:click={() => { removeApplication(currentApplication?.id ?? null) }}
    >
      close
    </IconButton>
  </div>

  <svelte:component this={componentDict[currentApplication.id]}
    bind:sidebarConfig={sidebarConfig}
    bind:items={items}
    bind:gridSize={gridSize}
    bind:activeLayer={activeLayer}
    bind:currentConfigId={currentConfigId}
    bind:currentSideBarConfigId={currentSideBarConfigId}
  />

  </div> 
    
  {/if}
</div>
<Form />
<Loading />


<style>
.tool-container{
  display: flex;
  flex-direction: column; 
  height: calc(100vh - 2px);
  width: calc(100vw - 2px);
  overflow: hidden;

  color: rgb(96, 125, 139);

  border: 1px solid rgb(96, 125, 139);
}

.main-container{
  height: 100vh;
  width: 100vw;

  display: flex;
  overflow: hidden;
}
</style>


