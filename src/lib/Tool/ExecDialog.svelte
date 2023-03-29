<Dialog
  bind:open={show}
  title={$t('Add Item')}
  width="400"
  height="300"
  on:close={() => { show = false }}
>
<Header>
  <Title id="fullscreen-title">{ ({
    exec_script: $t('Add Script'),
    start: $t('Add Start'),
    exec: $t('Add Exec')
  })[execMode]
   }</Title>
</Header>
<Content>
  <div style="display:flex;flex-direction:column;">

 

  

  {#if execMode === 'exec_script'}
    <Textfield
      label="Name"
      bind:value={modelScript.name}
    />
    <Textfield
      disabled={true}
      label="Script ID"
      bind:value={modelScript.scriptId}
    />

    <Button on:click={() => {
      setDialog([{
        label: 'Open',
        callback: async (record) => {
          modelScript.scriptId = record.id
          modelScript.name = record.name

          hide()
        }
      }])
    }}>
      <!-- Select Executable -->
      {$t('Select Script')}
    </Button>
  {:else}
  <Textfield
    label="Name"
    bind:value={model.exec}
  />

  <Button on:click={setExecPath}>
    <!-- Select Executable -->
    {$t('Select Executable')}
  </Button>


  <Textfield
    disabled={execMode === 'start'}
    label={$t('Args')}
    bind:value={model.args}
  />

  <Textfield
    label={$t('Working Directory')}
    bind:value={model.cwd}
  />

  {/if}

  <Textfield
    label={$t('Icon')}
    bind:value={model.icon}
  />
  <IconButton class="material-icons">
    {model.icon}
  </IconButton>
  <a href="https://fonts.google.com/icons" target="_blank" rel="noreferrer">Google Material Icons</a>

</div>
</Content>
  <Actions>
    <Button
      on:click={() => { show = false }}
    >
      <!-- Cancel -->
      { $t('Cancel') }
    </Button>
    <Button
      on:click={() => {
        show = false
        if (execMode === 'exec_script') {
          onSubmit(modelScript)
        } else {
          onSubmit(model)
        }
      }}
    >
      <!-- OK -->
      { $t('OK') }
    </Button>
  </Actions>
</Dialog>

<script lang="ts">
  import 'svelte-material-ui/bare.css'
  import IconButton from '@smui/icon-button'
  import { t } from 'svelte-i18n'
  import Textfield from '@smui/textfield'

  import Button from '@smui/button'
  import { open as openFileDialog } from '@tauri-apps/api/dialog'
  import Dialog, { Actions, Header, Title } from '@smui/dialog'
  import { Content } from '@smui/card'
  import { hide, setDialog } from '../../utils/record'

  export let execMode: 'exec' | 'start' | 'exec_script' = 'exec'

  const model = {
    type: '1' as '1',
    exec: '',
    args: '',
    cwd: '',
    icon: 'web_asset'
  }

  const modelScript = {
    type: '2' as '2',
    name: '',
    scriptId: '',
    icon: 'web_asset'
  }

  type Model = typeof model
  type ModelScript = typeof modelScript


  export let onSubmit : (model: Model | ModelScript) => void
  export let show = false

  $: show && (() => {
    model.exec = ''
    model.args = ''
    model.cwd = ''
    model.icon = 'web_asset'
  })()

  async function setExecPath () {
    const selected = await openFileDialog({
      filters: [{
        name: 'Executables',
        extensions: ['*']
      }]
    })

    const str = (typeof selected === 'string' ? selected : selected?.[0]) ?? ''

    model.exec = str

    if (!model.cwd) {
      model.cwd = str.replace(/\\[^\\]+$/, '')
    }
  }

  </script>
