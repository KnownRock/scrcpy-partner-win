<Dialog
  bind:open={show}
  title="Add Item"
  width="400"
  height="300"
  on:close={() => { show = false }}
>
<Header>
  <Title id="fullscreen-title">{ 'Run Application' }</Title>
</Header>
<Content>
  <div style="display:flex;flex-direction:column;">
  <Textfield
    label="Name"
    bind:value={model.exec}
  />
  <Button on:click={setExecPath}>Select Executable</Button>


  <Textfield
    disabled={execMode === 'start'}
    label="Args"
    bind:value={model.args}
  />

  <Textfield
    label="Cwd"
    bind:value={model.cwd}
  />
  <Textfield
    label="Icon"
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
      Cancel
    </Button>
    <Button
      on:click={() => {
        show = false
        onSubmit(model)
      }}
    >
      OK
    </Button>
  </Actions>
</Dialog>

<script lang="ts">
  import 'svelte-material-ui/bare.css'
  import IconButton from '@smui/icon-button'

  import Textfield from '@smui/textfield'

  import Button from '@smui/button'
  import { open as openFileDialog } from '@tauri-apps/api/dialog'
  import Dialog, { Actions, Header, Title } from '@smui/dialog'
  import { Content } from '@smui/card'
  const model = {
    exec: '',
    args: '',
    cwd: '',
    icon: 'web_asset'
  }

  type Model = typeof model

  export let execMode: 'exec' | 'start' = 'exec'


  export let onSubmit = (model: Model) => {}
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
