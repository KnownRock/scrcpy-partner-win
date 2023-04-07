<script lang="ts">
  import IconButton from '@smui/icon-button'
  import Textfield from '@smui/textfield'

  import gridHelp from 'svelte-grid/build/helper/index.mjs'
  import { v4 as uuidv4 } from 'uuid'
  import { t } from 'svelte-i18n'
  import { generalLoading } from '../../store/index'

  import prismaClientLike from '../../utils/prisma-like-client'
  import Button from '@smui/button'
  import Select, { Option } from '@smui/select'
  import ExecDialog from './ExecDialog.svelte'
  import { fullAddableItems } from './addable-items'
  import { getConfigWithSidebarConfig, getDefaultSidebarConfig } from './config'
  
  import { openRecordWindow } from '../../utils/app'
  import { hide, setDialog } from '../../utils/sidebar-config'


  export let sidebarConfig
  export let activeLayer
  export let gridSize
  export let items
  export let currentConfigId
  export let currentSideBarConfigId

  let showAddDialogVisible = false
  let showAddStartDialogVisible = false
  let showAddScriptDialogVisible = false

  function openSidebarConfigWindow () {
    setDialog([{
      label: 'Open',
      callback: async (entity) => {
        console.log('open', entity)

        await prismaClientLike.sideBarConfig.findUnique({
          where: {
            id: entity.id
          }
        }).then((config) => {
          if (config) {
            sidebarConfig = JSON.parse(config.value)
          }
        })

        currentSideBarConfigId = entity.id

        hide()

        return true
      }
    }, {
      label: 'Delete',
      callback: async (entity) => {
        generalLoading.set({
          show: true
        })

        await prismaClientLike.sideBarConfig.delete({
          where: {
            id: entity.id
          }
        })

        openSidebarConfigWindow()

        generalLoading.set({
          show: false
        })

        return true
      }
    }])
  }

  async function showAddDialog () {
    showAddDialogVisible = true
  }

  async function showAddScriptDialog () {
    showAddScriptDialogVisible = true
  }

  async function showAddStartDialog () {
    showAddStartDialogVisible = true
  }

  function addAbleItems (item: any) {
    const insertItem = {
      id: uuidv4(),
      item
    }

    const gridMatrix = new Array(gridSize[1]).fill(0).map(() => new Array(gridSize[0]).fill(0))
    sidebarConfig.layers[activeLayer].items.forEach((item) => {
      for (let i = 0; i < item.coord.h; i++) {
        for (let j = 0; j < item.coord.w; j++) {
          gridMatrix[item.coord.y + i][item.coord.x + j] = 1
        }
      }
    })
  

    let x = 0
    let y = 0

    for (let i = 0; i < gridMatrix.length; i++) {
      for (let j = 0; j < gridMatrix[i].length; j++) {
        if (gridMatrix[i][j] === 0) {
          x = j
          y = i
          break
        }
      }
    }


    const gridPosAndSize = gridHelp.item({
      x,
      y,
      w: 1,
      h: 1
    })
    insertItem[gridSize[0]] = gridPosAndSize
    items.push(insertItem)
    items = items
  }


  function resetSidebarConfig () {
    const defaultConfig = getDefaultSidebarConfig()
    sidebarConfig = defaultConfig
  }

  async function saveSidebarConfig () {
    generalLoading.set({
      show: true
    })

    const config = await getConfigWithSidebarConfig(currentConfigId)

    if (config) {
      const sbc = await prismaClientLike.sideBarConfig.upsert({
        where: {
          id: currentSideBarConfigId
        },
        update: {
          name: sidebarConfig.name,
          value: JSON.stringify(sidebarConfig)
        },
        create: {
          name: sidebarConfig.name,
          value: JSON.stringify(sidebarConfig)
        }
      })

      currentSideBarConfigId = sbc.id

      prismaClientLike.deviceConfig.update({
        where: {
          id: currentConfigId
        },
        data: {
          sideBarConfig: {
            connect: {
              id: currentSideBarConfigId
            }
          }
        }
      })
    }

    generalLoading.set({
      show: false
    })
  }

  let innerGridSize = [...gridSize]

  function setInnerGridSize () {
    innerGridSize = gridSize
  }


  $: gridSize && (() => {
    setInnerGridSize()
  })()


  $: gridSize[0] = +innerGridSize[0] || 1
  $: gridSize[1] = +innerGridSize[1] || 1

</script>



<ExecDialog
  execMode="exec"
  bind:show={showAddDialogVisible}
  onSubmit={(model) => {
    if (model.type === '2') return

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

<ExecDialog
  execMode="exec_script"
  bind:show={showAddScriptDialogVisible}
  onSubmit={(model) => {
    if (model.type === '2') {
      const execButtonInfo = {
      icon: model.icon,
      uiType: 'icon-button',

      cmdType: 'app-cmd',
      cmdName: 'exec_script',
      opts: {
        scriptId: model.scriptId
      }
    }
    addAbleItems(execButtonInfo)
    showAddScriptDialogVisible = false
    }
  }}
  
/>

<ExecDialog
  bind:show={showAddStartDialogVisible}
  execMode="start"
  onSubmit={(model) => {
    if (model.type === '2') return

    const execButtonInfo = {
      icon: model.icon,
      uiType: 'icon-button',

      cmdType: 'app-cmd',
      cmdName: 'start',
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
    padding:0 10px 0 50px; min-height:500px;
    display: flex; flex-direction: column; justify-content: space-between;
    "
>
  <div>
    <h1>{$t('Setting')}</h1>

    <!-- name -->
    <Textfield
      label={$t('Sidebar Config Name')}
      bind:value={sidebarConfig.name}
      type="text"
    />

    <br />

    <Select bind:value={activeLayer}>
      {#each sidebarConfig.layers as layer, index}
        <Option value={index}>
          {`${$t('Layer')} ${index + 1}`}
        </Option>
      {/each}
    </Select>

    <div style="display:flex">
      <Textfield 
          label={$t('Grid Size X')}
          bind:value={innerGridSize[0]} 
          min={1}
          type="number" />
        <Textfield 
          style="margin-left: 10px;"
          label={$t('Grid Size Y')}
          min={1}
          bind:value={innerGridSize[1]} 
          type="number" />
    </div>

    <div style="display:flex;    width: 350px;    flex-flow: row wrap;">
      {#each fullAddableItems as item}
        {#if item.uiType === 'icon-button' || item.uiType === 'icon-button-2'}
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

      <IconButton class="material-icons" on:click={() => showAddStartDialog()}>
        terminal
      </IconButton>

      <IconButton class="material-icons" on:click={() => showAddScriptDialog()}>
        code
      </IconButton>
    </div>
  </div>

  <div
    style="
      display: flex; justify-content: space-between;
      "
  >
  <div>
    <Button
      on:click={() => {
        openRecordWindow()
      }}
      style="margin-right: 10px;"
    >
      {$t('Record')}
    </Button>

  </div>
    <div>
    <Button
      on:click={() => {
        openSidebarConfigWindow()
      }}
      style="margin-right: 10px;"
    >
      <!-- Open -->
      {$t('Open')}
    </Button>

    <Button
      on:click={() => {
        resetSidebarConfig()
      }}
      style="margin-right: 10px;"
    >
      <!-- Reset -->
      {$t('Reset')}
    </Button>
    <Button
      on:click={() => {
        saveSidebarConfig()
      }}
    >
      <!-- Save -->
      {$t('Save')}
    </Button>
    </div>
  </div>
</div>
