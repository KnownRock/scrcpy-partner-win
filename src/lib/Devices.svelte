<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import List, { Item, Separator, Text } from '@smui/list';
  // import Button from "@smui/button";
  import LayoutGrid, { Cell } from '@smui/layout-grid';
  import Button, { Label } from '@smui/button';
  import Card, {
    Content,
    PrimaryAction,
    Media,
    MediaContent,
    Actions,
    ActionButtons,
    ActionIcons,
  } from '@smui/card';
  import IconButton, { Icon } from '@smui/icon-button';
  type Device = {
    name: string,
    id: string,
    model: string,
    device: string
    deviceProduct: string
    transportId: string
  }

  import Config from './Config.svelte';

  let devices : Device[] = [];

  let showConfig = false

  async function setDevices() {
    const rawOutput = await invoke("adb_devices_l") as string;  
    const lines = rawOutput.split("\n");
    const deviceLines = lines.slice(1, lines.length).filter(line => line.match(/\S/));

    devices = deviceLines.map(line => {
      const [,id, deviceProduct, model, device, transportId] = 
        line.match(/(\S+)\s+device product:(\S+) model:(\S+) device:(\S+) transport_id:(\S+)/);
      const name = id;
      return { name, id, model, device, deviceProduct, transportId };
    }); 


    devices = devices.concat(devices)
  }

  async function lanuchScrcpy() {
    await invoke("lanuch_scrcpy", { id: "emulator-5554" });
  }

  setDevices();

  
</script>

<div>

  <div style="display: flex; justify-content: space-between; align-items: flex-end;">
    <h1 style="margin-bottom:0.3em;">Device list</h1>
    <Button on:click={setDevices}>
      refresh
    </Button>

  </div>

  <Config bind:open={showConfig} />
  
  <LayoutGrid>
    {#each devices as device}
      <Cell>
        <Card>
          <PrimaryAction on:click={() =>1}>
            <!-- <Media class="card-media-16x9" aspectRatio="16x9" /> -->
            <Content class="mdc-typography--body2">
              <h2 class="mdc-typography--headline6" style="margin: 0;">
                <!-- A card with media. -->
                {device.name}
              </h2>
              <h3
                class="mdc-typography--subtitle2"
                style="margin: 0 0 10px; color: #888;"
              >
                {device.model}
              </h3>
            </Content>
          </PrimaryAction>
          <Actions>
            <ActionButtons>
              <Button on:click={() => lanuchScrcpy()}>
                <Label>Start</Label>
              </Button>
            </ActionButtons>
            <ActionIcons>
              <IconButton
                class="material-icons"
                on:click={() => { showConfig = true } }
                title="More options">more_vert
              </IconButton>
            </ActionIcons>
          </Actions>
        </Card>
      </Cell>
    {/each}

  </LayoutGrid>

</div>