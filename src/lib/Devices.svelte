<script lang="ts">


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

  import {getDevices, lanuchSelf} from "../utils/devices";
  import type { Device  } from "../utils/devices";

  import Config from './Config.svelte';

  let devices : Device[] = [];

  let showConfig = false
  let currentDeviceId : string = ""

  async function setDevices() {
    devices = await getDevices()
  }

  async function lanuchScrcpy(deviceId: string) {
    lanuchSelf([ `-s${deviceId}` ])
  }

  setDevices();

  
</script>

<div>

  <!-- <div style="display: flex; justify-content: space-between; align-items: flex-end;">
    <h1 style="margin-bottom:0.3em;">Device list</h1>
    <Button on:click={setDevices}>
      refresh
    </Button>

  </div> -->

  <Config bind:open={showConfig} currentDeviceId={currentDeviceId} />
  
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
              <Button on:click={() => lanuchScrcpy(device.id)}>
                <Label>Start</Label>
              </Button>
            </ActionButtons>
            <ActionIcons>
              <IconButton
                class="material-icons"
                on:click={() => { 
                  showConfig = true;
                  currentDeviceId = device.id;
                }}
                title="More options">more_vert
              </IconButton>
            </ActionIcons>
          </Actions>
        </Card>
      </Cell>
    {/each}

  </LayoutGrid>

</div>