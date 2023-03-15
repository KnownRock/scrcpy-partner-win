
<script lang="ts">
  import 'svelte-material-ui/bare.css'
  import IconButton from '@smui/icon-button'

  import { invoke } from '@tauri-apps/api/tauri'
  import { onMount } from 'svelte'
  import { exit, getConfigId, setToolWindowSize, showToolWindow, open } from '../utils/app'
  import Grid from 'svelte-grid'
  import Textfield from '@smui/textfield'
  import { commandKeyDict } from './Tool/command-key-dict'

  import gridHelp from 'svelte-grid/build/helper/index.mjs'
  import { v4 as uuidv4 } from 'uuid'
  import { addableItems, addableItems2 } from './Tool/addable-items'
  import prismaClientLike from '../utils/prisma-like-client'
  import Button from '@smui/button'
  import Select, { Option } from '@smui/select'
  import { lanuchSelf } from '../utils/devices'

  import { open as openFileDialog } from '@tauri-apps/api/dialog'
  import Dialog, { Actions, Header, Title } from '@smui/dialog'
  import { Content } from '@smui/card'


  const fullAddableItems = addableItems.concat(addableItems2)
  
  let sidebarName = ''
  
  function getDefaultSidebarSingleLayer () {
    return {
      gridSize: [1, addableItems.length],
      items: addableItems.map((item, index) => {
        const insertItem = {
          id: uuidv4(),
          item
        }
        const gridPosAndSize = gridHelp.item({
          x: 0,
          y: index,
          w: 1,
          h: 1
        })

        Array.from(Array(maxWidth + 1)).forEach((_, i) => {
          insertItem[i] = gridPosAndSize
        })
  
        return insertItem
      })

    }
  }

  function getDefaultSidebarConfig () {
    const sidebarConfig = {
      activeLayer: 0,
      layers: Array.from(Array(9)).map((_, i) => {
        if (i === 0) {
          return getDefaultSidebarSingleLayer()
        } else {
          return {
            gridSize: [1, 1],
            items: []
          }
        }
      })
    }

    return sidebarConfig
  }


  let sidebarConfig = {
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

  let mode: 'normal' | 'setting' = 'normal'
  async function toggleMode () {
    if (mode === 'normal') {
      mode = 'setting'
    } else {
      mode = 'normal'
    }
  }

  let gridSize = [1, addableItems.length]

  const maxWidth = 10

  // $: sidebarConfig && (() => {
  //   gridSize = sidebarConfig.layers[sidebarConfig.activeLayer].gridSize
  //   items = sidebarConfig.layers[sidebarConfig.activeLayer].items
  // })()
  let activeLayer = 0
  $: activeLayer !== undefined && (() => {
    sidebarConfig.activeLayer = +activeLayer

    layerChanged()
  })()

  function layerChanged () {
    activeLayer = sidebarConfig.activeLayer
    gridSize = sidebarConfig.layers[sidebarConfig.activeLayer].gridSize
    items = sidebarConfig.layers[sidebarConfig.activeLayer].items
  }

  $: mode && gridSize && (() => {
    // debugger

    gridSize[0] = Math.max(1, gridSize[0])
    gridSize[1] = Math.max(1, gridSize[1])
    gridSize[0] = Math.min(maxWidth, gridSize[0])
    gridSize[1] = Math.min(20, gridSize[1])

    const barSize = [
      gridSize[0] * 50,
      gridSize[1] * 50
    ]
  
    gridSize = gridSize

    if (mode === 'normal') {
      setToolWindowSize(barSize[0] + 2, barSize[1] + 2)
      disableItems()
    } else {
      const newHeight = Math.max(600, barSize[1])

      setToolWindowSize(barSize[0] + 400 + 2, newHeight + 2)
      enableItems()
    }
  })()

  let items = [] as Array<any>


  function addAbleItems (item: any) {
    const insertItem = {
      id: uuidv4(),
      item
    }

    const gridPosAndSize = gridHelp.item({
      x: 0,
      y: 0,
      w: 1,
      h: 1
    })


    Array.from(Array(maxWidth + 1)).forEach((_, i) => {
      insertItem[i] = gridPosAndSize
    })
  
    items.push(insertItem)

    items = items
  }

  $: items && (() => {
    items.forEach((item) => {
      Array.from(Array(maxWidth + 1)).forEach((_, i) => {
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
    })

    sidebarConfig.layers[sidebarConfig.activeLayer].items = items
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
        sidebarName = queriedSidebarConfig.name
        const config = JSON.parse(queriedSidebarConfig.value)
        sidebarConfig = config
      } else {
        sidebarConfig = getDefaultSidebarConfig()
      }
    } else {
      sidebarConfig = getDefaultSidebarConfig()
    }

    layerChanged()
  })

  function resetSidebarConfig () {
    const defaultConfig = getDefaultSidebarConfig()
    sidebarConfig = defaultConfig

    layerChanged()
  }

  async function saveSidebarConfig () {
    const config = await getConfigWithSidebarConfig()

    if (config) {
      // const sidebarConfig = config.sideBarConfig

      // debugger
  
      prismaClientLike.deviceConfig.update({
        where: {
          id: currentConfigId
        },
        data: {
          sideBarConfig: {
            update: {
              name: sidebarName,
              value: JSON.stringify(sidebarConfig)
            },
            create: {
              name: sidebarName,
  
              value: JSON.stringify(sidebarConfig)
            }
          }
        }
      })
    }
  }

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
    }
  }

  function delItem (item) {
    items = items.filter((i) => i.id !== item.id)
    sidebarConfig.layers[sidebarConfig.activeLayer].items = items
  }

  function enableItems () {
    items.forEach((item) => {
      Array.from(Array(maxWidth + 1)).forEach((_, i) => {
        item[i] = {
          ...item[i],
          fixed: false,
          resizable: true,
          draggable: true
        }
      })
    })

    items = [...items]
  }

  function disableItems () {
    items.forEach((item) => {
      Array.from(Array(maxWidth + 1)).forEach((_, i) => {
        item[i] = {
          ...item[i],
          fixed: true,
          resizable: false,
          draggable: false
        }
      })
    })

    items = [...items]
  }


  async function showAddDialog () {
    // alert(1234)

    showAddDialogVisible = true

    // const selected = await open({
    //   filters: [{
    //     name: 'Executables',
    //     extensions: ['*']
    //   }]
    // })

    // console.log('selected', selected)
  }

  let showAddDialogVisible = false

  const execButtonInfo = {
    icon: 'web_asset',
    uiType: 'icon-button',

    cmdType: 'app-cmd',
    cmdName: 'open',
    opts: {
      exec: '',
      args: '',
      cwd: ''
    }

  }

  async function setExecPath () {
    const selected = await openFileDialog({
      filters: [{
        name: 'Executables',
        extensions: ['*']
      }]
    })

    const str = (typeof selected === 'string' ? selected : selected?.[0]) ?? ''

    execButtonInfo.opts.exec = str

    if (!execButtonInfo.opts.cwd) {
      execButtonInfo.opts.cwd = str.replace(/\\[^\\]+$/, '')
    }
  }

</script>
<Dialog
  bind:open={showAddDialogVisible}
  title="Add Item"
  width="400"
  height="300"
  on:close={() => { showAddDialogVisible = false }}
>
<Header>
  <Title id="fullscreen-title">{ 'Run Application' }</Title>
</Header>
<Content>
  <div style="display:flex;flex-direction:column;">
  <Textfield
    label="Name"
    bind:value={execButtonInfo.opts.exec}
  />
  <Button on:click={setExecPath}>Select Executable</Button>
  <Textfield
    label="Args"
    bind:value={execButtonInfo.opts.args}
  />
  <Textfield
    label="Cwd"
    bind:value={execButtonInfo.opts.cwd}
  />
  <Textfield
    label="Icon"
    bind:value={execButtonInfo.icon}
  />
  <IconButton class="material-icons">
    {execButtonInfo.icon}
  </IconButton>
  <a href="https://fonts.google.com/icons" target="_blank" rel="noreferrer">Google Material Icons</a>

</div>
</Content>
  <Actions>
    <Button
      on:click={() => { showAddDialogVisible = false }}
    >
      Cancel
    </Button>
    <Button
      on:click={() => {
        showAddDialogVisible = false
        addAbleItems(execButtonInfo)
      }}
    >
      OK
    </Button>
  </Actions>
</Dialog>



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
      
      gap={[0, 0]}
      rowHeight={50}
      let:item
      let:dataItem
      cols={[[gridSize[1], gridSize[0]]]}
    >
      <!-- <div class="item" style="background: {dataItem.color}">
        {dataItem.item}
      </div> -->
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
  <div style="
    padding:0 10px 0 50px; min-height:600px;
    display: flex; flex-direction: column; justify-content: space-between;
    ">
    <div>
      <h1>
        Settings
      </h1>

      <!-- name -->
      <Textfield 
        label="Sidebar Config Name"
        bind:value={sidebarName} 
        type="text" />

      <Select bind:value={activeLayer} >
        {#each sidebarConfig.layers as layer, index}
          <Option value={index}>
            {`Layer ${index + 1}`}
          </Option>
        {/each}
      </Select>
  
      <div style="display:flex">
        <Textfield 
          label="Grid Size X"
          bind:value={gridSize[0]} 
          min={1}
          type="number" />
        <Textfield 
          style="margin-left: 10px;"
          label="Grid Size Y"
          min={1}
          bind:value={gridSize[1]} 
          type="number" />
      </div>
  
      <div style="display:flex;    width: 350px;    flex-flow: row wrap;">
  
        {#each fullAddableItems as item}
          {#if item.uiType === 'icon-button'}
          <IconButton class="material-icons" on:click={() => addAbleItems(item)}>
            {item.icon}
          </IconButton> 
          {/if}
        {/each}

        <IconButton class="material-icons" on:click={() => showAddDialog()}>
          open_in_new
        </IconButton> 
      </div>
  
    </div>
    

    <div
      style="
      display: flex; justify-content: flex-end;
      "  
    >
      <!-- resett -->
      <Button on:click={() => { resetSidebarConfig() }} style="margin-right: 10px;">
        Reset
      </Button>
      <!-- save -->
      <Button on:click={() => { saveSidebarConfig() }}>
        Save
      </Button>
    </div>



  </div>
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


