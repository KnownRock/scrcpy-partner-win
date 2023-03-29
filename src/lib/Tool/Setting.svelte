<script lang="ts">
  import IconButton from '@smui/icon-button'
  import Textfield from '@smui/textfield'
  import Loading from '../general/Loading.svelte'
  import gridHelp from 'svelte-grid/build/helper/index.mjs'
  import { v4 as uuidv4 } from 'uuid'
  import { t } from 'svelte-i18n'
  import { generalLoading } from '../../store/index'

  import prismaClientLike from '../../utils/prisma-like-client'
  import Button from '@smui/button'
  import Select, { Option } from '@smui/select'
  import ExecDialog from './ExecDialog.svelte'
  import { addableItems, addableItems2 } from './addable-items'
  import { getConfigWithSidebarConfig, getDefaultSidebarConfig } from './config'
  import Form from '../general/Form.svelte'
  import { openRecordWindow } from '../../utils/app'

  const fullAddableItems = addableItems.concat(addableItems2)

  export let sidebarConfig
  export let activeLayer
  export let gridSize
  export let items
  export let currentConfigId

  let showAddDialogVisible = false
  let showAddStartDialogVisible = false
  let showAddScriptDialogVisible = false

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

    const gridPosAndSize = gridHelp.item({
      x: 0,
      y: 0,
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

<Form />

<Loading />
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
