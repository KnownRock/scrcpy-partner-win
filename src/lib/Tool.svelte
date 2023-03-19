
<script lang="ts">
  import 'svelte-material-ui/bare.css'
  import IconButton from '@smui/icon-button'
  import { invoke } from '@tauri-apps/api/tauri'
  import { onMount } from 'svelte'
  import { exit, getConfigId, setToolWindowSize, showToolWindow, open, start } from '../utils/app'
  import Grid from 'svelte-grid'
  import { commandKeyDict } from './Tool/command-key-dict'
  import { Command } from '@tauri-apps/api/shell'
  import prismaClientLike from '../utils/prisma-like-client'
  import { lanuchSelf } from '../utils/devices'

  import Setting from './Tool/Setting.svelte'
  import { getDefaultSidebarConfig } from './Tool/config'
  import gridHelp from 'svelte-grid/build/helper/index.mjs'

  let sidebarConfig = {
    name: 'default',
    activeLayer: 0,
    layers: Array.from(Array(9)).map((_, i) => {
      return {
        gridSize: [1, 1],
        items: [] as any[]
      }
    })
  }
  
  
  async function sendKey (command: string) {
    const opt = commandKeyDict[command]
    if (opt) {
      await invoke('sendkey', opt)
    }
  }

  type Application = {
    label: 'recorder'
    id: 'recorder'
  }

  const applications = [] as Application[]

  function addApplication (
    application: Application
  ) {
    const index = applications.findIndex((a) => a.id === application.id)
    if (index === -1) {
      applications.push(application)
    }
  }

  let mode: 'normal' | 'setting' = 'normal'
  async function toggleMode () {
    if (mode === 'normal') {
      mode = 'setting'
    } else {
      mode = 'normal'
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
    // items = sidebarConfig.layers[sidebarConfig.activeLayer].items
    setItemsBySidebarConfig()
  }

  $: mode && applications && gridSize && (() => {
    gridSize[0] = Math.max(1, gridSize[0])
    gridSize[1] = Math.max(1, gridSize[1])
    // gridSize[0] = Math.min(maxWidth, gridSize[0])
    // gridSize[1] = Math.min(20, gridSize[1])

    const barSize = [
      gridSize[0] * 50,
      gridSize[1] * 50
    ]
  
    gridSize = gridSize

    if (mode === 'normal') {
      setToolWindowSize(barSize[0] + 2, barSize[1] + 2)
    } else {
      // keep setting visible
      const newHeight = Math.max(600, barSize[1])
      setToolWindowSize(barSize[0] + 400 + 2, newHeight + 2)
    }

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
      // Array.from(Array(maxWidth + 1)).forEach((_, i) => {
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
      // })
    })

    // sidebarConfig.layers[sidebarConfig.activeLayer].items = items.map((i) => {
    //   return {
    //     id: i.id,
    //     size: i[gridSize[0]],
    //     item: i.item
    //   }
    // })

    setSidebarConfigItems(items)
  })()


  async function getConfigWithSidebarConfig () {
    const config = await prismaClientLike.deviceConfig.findUnique({
      where: {
        id: currentConfigId
      },
      include: {
        sideBarConfig: true
      }
    })

    return config
  }

  let currentConfigId = ''
  onMount(async () => {
    showToolWindow()

    const configId = await getConfigId()
    console.log('configId', configId)

    currentConfigId = configId

    const config = await getConfigWithSidebarConfig()
    console.log('config', config)

    if (config) {
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
  })


  function execute (item) {
    if (item.cmdType === 'scrcpy-cmd') {
      sendKey(item.cmdName)
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
      if (item.cmdName === 'record') {
        // set mode to recording or show a record application?
      }
    }
  }

  function delItem (item) {
    items = items.filter((i) => i.id !== item.id)
    // sidebarConfig.layers[sidebarConfig.activeLayer].items = items
    setSidebarConfigItems(items)
  }

</script>



<div class="main-container">
  <div 
  class="tool-container"
  style={
  `width: ${gridSize[0] * 50}px; height: ${gridSize[1] * 50}px; min-width: ${gridSize[0] * 50}px; min-height: ${gridSize[1] * 50}px;`}
  >
    <!-- temp -->
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    {#if currentConfigId}
      <div 
        on:click={() => { toggleMode() }}
        style="position: absolute; bottom: 0; left: 0; width: 10px; height: 10px; background: #000; opacity: 0.2; z-index: 1;">
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
      on:dblclick={() => { delItem(dataItem) }} >
        <IconButton
        disabled={mode === 'setting'}
        
       class="material-icons" on:click={() => execute(dataItem.item)}>
        {dataItem.item.icon}
      </IconButton> 
      </div>
  
    
      
    </Grid>
  
  </div>
  {#if mode === 'setting'}
    <Setting
      bind:sidebarConfig={sidebarConfig}
      bind:items={items}
      bind:gridSize={gridSize}
      bind:activeLayer={activeLayer}
      bind:currentConfigId={currentConfigId}
      getConfigWithSidebarConfig={getConfigWithSidebarConfig}

    />
  {/if}
</div>



<style>
.tool-container{
  display: flex;
  flex-direction: column; 
  /* align-items: center;  */
  /* justify-content: center; */
  height: calc(100vh - 2px);
  width: calc(100vw - 2px);
  overflow: hidden;

  color: rgb(96, 125, 139);
  /* background-color: rgb(240, 240, 240); */

  border: 1px solid rgb(96, 125, 139);

  /* background-color: aqua; */
}

.main-container{
  height: 100vh;
  width: 100vw;

  display: flex;
  overflow: hidden;

  /* background-color: antiquewhite; */
}
</style>


