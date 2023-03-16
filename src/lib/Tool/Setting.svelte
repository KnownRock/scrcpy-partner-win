<script lang="ts">
  import IconButton from '@smui/icon-button'
  import Textfield from '@smui/textfield'

  import gridHelp from 'svelte-grid/build/helper/index.mjs'
  import { v4 as uuidv4 } from 'uuid'

  import prismaClientLike from '../../utils/prisma-like-client'
  import Button from '@smui/button'
  import Select, { Option } from '@smui/select'
  import ExecDialog from './ExecDialog.svelte'
  import { addableItems, addableItems2 } from './addable-items'

  const fullAddableItems = addableItems.concat(addableItems2)

  export let sidebarConfig
  export let activeLayer
  export let gridSize
  export let items
  export let maxWidth
  export let getDefaultSidebarConfig
  export let layerChanged
  export let getConfigWithSidebarConfig
  export let currentConfigId

  let showAddDialogVisible = false

  async function showAddDialog () {
    showAddDialogVisible = true
  }

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
              name: sidebarConfig.name,
              value: JSON.stringify(sidebarConfig)
            },
            create: {
              name: sidebarConfig.name,
  
              value: JSON.stringify(sidebarConfig)
            }
          }
        }
      })
    }
  }

</script>

<ExecDialog
  bind:show={showAddDialogVisible}
  onSubmit={(model) => {
     const execButtonInfo = {
    icon: model.icon,
    uiType: 'icon-button',

    cmdType: 'app-cmd',
    cmdName: 'open',
    opts: {
      exec: model.exec,
      args: model.args,
      cwd: model.cwd
    }

  }

    addAbleItems(execButtonInfo)
    showAddDialogVisible = false
  }}
  
  />

<div
  style="
    padding:0 10px 0 50px; min-height:600px;
    display: flex; flex-direction: column; justify-content: space-between;
    "
>
  <div>
    <h1>Settings</h1>

    <!-- name -->
    <Textfield
      label="Sidebar Config Name"
      bind:value={sidebarConfig.name}
      type="text"
    />

    <br />

    <Select bind:value={activeLayer}>
      {#each sidebarConfig.layers as layer, index}
        <Option value={index}>
          {`Layer ${index + 1}`}
        </Option>
      {/each}
    </Select>

    <div style="display:flex">
      <!-- <Textfield 
          label="Grid Size X"
          bind:value={gridSize[0]} 
          min={1}
          type="number" />
        <Textfield 
          style="margin-left: 10px;"
          label="Grid Size Y"
          min={1}
          bind:value={gridSize[1]} 
          type="number" /> -->

      <Select
        bind:value={gridSize[0]}
        label="Grid Size X"
        style="width: 150px;"
      >
        {#each [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] as size}
          <Option value={size}>
            {`${size}`}
          </Option>
        {/each}
      </Select>

      <Select
        label="Grid Size Y"
        style="margin-left: 10px;width: 150px;"
        bind:value={gridSize[1]}
      >
        {#each [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20] as size}
          <Option value={size}>
            {`${size}`}
          </Option>
        {/each}
      </Select>

      <!-- <Slider
          style="width: 100px;"
          bind:value={gridSize[0]}
          min={1}
          max={10}
          step={1}
          discrete
          input$aria-label="Grid Size X"
        />
        
        <Slider
          style="width: 100px; margin-left: 10px;"
          bind:value={gridSize[1]}
          min={1}
          max={20}
          step={1}
          discrete
          input$aria-label="Grid Size Y"
        /> -->
    </div>

    <div style="display:flex;    width: 350px;    flex-flow: row wrap;">
      {#each fullAddableItems as item}
        {#if item.uiType === 'icon-button'}
          <IconButton
            class="material-icons"
            on:click={() => addAbleItems(item)}
          >
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
    <Button
      on:click={() => {
        resetSidebarConfig()
      }}
      style="margin-right: 10px;"
    >
      Reset
    </Button>
    <!-- save -->
    <Button
      on:click={() => {
        saveSidebarConfig()
      }}
    >
      Save
    </Button>
  </div>
</div>
