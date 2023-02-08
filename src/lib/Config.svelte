<script lang="ts">
  import Dialog, { Header, Title, Content, Actions } from '@smui/dialog';
  import IconButton from '@smui/icon-button';
  import Button, { Label } from '@smui/button';
  // import LoremIpsum from '$lib/LoremIpsum.svelte';
  import Autocomplete from '@smui-extra/autocomplete';

  import type { Device } from '../utils/devices';
  import { getDevices } from '../utils/devices';
  let devices : Device[] = [];

  export let open = false;
  export let currentDeviceId = "";
  let response = 'Nothing yet.';

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
  <Content id="fullscreen-content">
    <div style="height:25em;">
      <Autocomplete
        options={devices.map((d) => d.id)}
        bind:value={currentDeviceId}
        label="Device"
      />
    </div>

    
  </Content>
  <Actions>
    <Button action="Save">
      <Label>Save</Label>
    </Button>
    <Button action="cancel">
      <Label>Cancel</Label>
    </Button>
    <Button action="start" defaultAction>
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
