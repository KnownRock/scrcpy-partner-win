<script lang="ts">
  import Dialog, { Header, Title, Content, Actions } from '@smui/dialog';
  import IconButton from '@smui/icon-button';
  import Button, { Label } from '@smui/button';
  import Switch from '@smui/switch';
  // import LoremIpsum from '$lib/LoremIpsum.svelte';
  import Autocomplete from '@smui-extra/autocomplete';
  import FormField from '@smui/form-field';
  import type { Device } from '../utils/devices';
  import { getDevices, lanuchSelf } from '../utils/devices';
  let devices : Device[] = [];

  export let open = false;
  export let currentDeviceId = "";

  let form = getForm(devices);

  type FormItem = {
    type: 'option',
    label: string,
    name: string,
    options: string[],
    value: string
  } | {
    type: 'switch',
    label: string,
    name: string,
    value: boolean
  } | {
    type: 'br'
  }

  function getForm(devices){
    const form : FormItem[] = [{
      type: 'option',
      label: 'Device',
      name: 'serial',
      options: devices.map((d) => d.id),
      value: currentDeviceId
    },{
      type: 'option',
      label: 'Max Size',
      name: 'max-size',
      options: ['2160', '1440', '1080', '720', '480', '360', '240'],
      value: '1080',
    },{
      type: 'option',
      label: 'Bit Rate',
      name: 'bit-rate',
      options: ['32M', '16M', '8M', '4M', '2M', '1M', '512K', '256K'],
      value: '8M'
    },{
      type: 'option',
      label: 'FPS',
      name: 'max-fps',
      options: ['144','120','75', '60', '30', '20', '15', '10', '5'],
      value: '60'
    },{
      type: 'option',
      label: 'Display Buffer',
      name: 'display-buffer',
      options: ['100', '50', '30', '20', '10', '5', '0'],
      value: '0'
    }, {
      type:'br'
    }, {
      type: 'switch',
      label: 'Always on top',
      name: 'always-on-top',
      value: false
    }, {
      type: 'switch',
      label: 'Fullscreen',
      name: 'fullscreen',
      value: false
    }, {
      type: 'switch',
      label: 'Window Borderless',
      name: 'window-borderless',
      value: false
    }]
    return form;
  }

  let response = ''

  function closeHandler(e: CustomEvent<{ action: string }>) {
    switch (e.detail.action) {
      case 'close':
        response = 'Closed without response.';
        break;
      case 'reject':
        response = 'Rejected.';
        break;
      case 'accept':
        response = 'Accepted.';
        break;
    }
  }

  async function setDevices() {
    devices = await getDevices();
    form = getForm(devices);
  }

  async function start(){
    function formToArgs(form: FormItem[]){
      const args = [];
      for (const item of form) {
        if(item.type === 'option'){
          args.push(`--${item.name}=${item.value}`);
        } else if(item.type === 'switch'){
          if(item.value){
            args.push(`--${item.name}`);
          }
        }
      }

      return args
    }

    const args = formToArgs(form);
    lanuchSelf(args);
  }

  // $: open && setDevices();

  $: open && (()=>{
    setDevices();
  })()

</script>


<Dialog
  bind:open
  fullscreen
  aria-labelledby="fullscreen-title"
  aria-describedby="fullscreen-content"
  on:SMUIDialog:closed={closeHandler}
>
  <Header>
    <Title id="fullscreen-title">Config</Title>
    <IconButton action="close" class="material-icons">close</IconButton>
  </Header>
  <Content id="fullscreen-content" >
    {#each form as formItem}
      {#if formItem.type === 'option'}
        <Autocomplete
          options={formItem.options}
          bind:value={formItem.value}
          label={formItem.label}
        />
        <br>
      {/if}
      {#if formItem.type === 'br'}
        <br/>
      {/if}
      {#if formItem.type === 'switch'}
        <FormField align="end">
          <Switch bind:checked={formItem.value} />
          <span slot="label">{formItem.label}</span>
        </FormField>
      {/if}
    
      
    {/each}

    <!-- <div style="height:25em;">
      <Autocomplete
        options={devices.map((d) => d.id)}
        bind:value={currentDeviceId}
        label="Device"
      />
    </div> -->

    
  </Content>
  <Actions>
    <!-- <Button action="Save">
      <Label>Save</Label>
    </Button> -->
    <Button action="cancel">
      <Label>Cancel</Label>
    </Button>
    <Button action="start" defaultAction  on:click={() => start()}>
      <Label>Start</Label>
    </Button>
  </Actions>
</Dialog>

<!-- 
<Button on:click={() => (open = true)}>
  <Label>Open Dialog</Label>
</Button>

<IconButton
  class="material-icons"
  on:click={() => (open = true)}
  title="More options">more_vert
</IconButton>

<pre class="status">Response: {response}</pre>
 -->
