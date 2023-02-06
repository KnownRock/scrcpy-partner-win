<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import List, { Item, Separator, Text } from '@smui/list';
  import Button from "@smui/button";

  type Device = {
    name: string,
    id: string
  }

  let name = "";
  let devices : Device[] = [];
  let clicked = ""

  async function greet(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    devices = await invoke("greet", { name })
  }

  async function listDevices() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    devices = await invoke("list_devices")
  }

  listDevices()
  
</script>

<div>
  <div class="row">
    <Button on:click={listDevices}>
      refresh
    </Button>
  </div>
  <p>Device list</p>
  <!-- <p>{devices}</p> -->
  <!-- svelte for loop -->
  <List class="demo-list">
    {#each devices as device}
      <Item on:SMUI:action={() => (clicked = 'Cut')}><Text>{device.name}</Text></Item>
    {/each}
  </List>

</div>