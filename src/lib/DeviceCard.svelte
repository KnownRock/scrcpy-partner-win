<script lang="ts">
  import Button, { Icon, Label } from '@smui/button'
  import Badge from '@smui-extra/badge'
  import Card, {
    Content,
    PrimaryAction,
    Actions,
    ActionButtons,
    ActionIcons
  } from '@smui/card'
  import IconButton from '@smui/icon-button'
  // TODO: move all type to global.d.ts
  import { deleteDevice, lanuchSelf, type DeviceExt } from '../utils/devices'
  import { configForm, confirmDialog, deviceForm } from '../store/index'
  import { getContext } from 'svelte'
  
  const freshDevices = getContext('freshDevices') as () => void

  export let device: DeviceExt

  async function lanuchScrcpy () {
    lanuchSelf([`-s${device.adbId}`])
  }

  async function saveDevice (device) {
    // console.log('saveDevice', device)
    deviceForm.set({
      show: true,
      device,
      callback: async (device) => {
        await freshDevices()
      }
    })
  }

  async function handleDeleteDevice (device) {
    confirmDialog.set({
      show: true,
      title: 'Delete device',
      message: `Are you sure to delete device [${device.name}]?`,
      okCallback: async () => {
        // console.log('delete device', device)
        await deleteDevice(device.id)

        await freshDevices()
      }
    })
  }

  function showConfig () {
    configForm.set({
      show: true,
      deviceAdbId: device.adbId
    })
  }
</script>

<Card>
  <PrimaryAction on:click={() => 1}>
    <!-- <Media class="card-media-16x9" aspectRatio="16x9" /> -->
    <Content class="mdc-typography--body2" >
      <div style="display: flex; justify-content: space-between; align-items: center;">
        <div >
          <h2 class="mdc-typography--headline6" style="margin: 0;">
            <!-- A card with media. -->
            {device.name}&nbsp;
          </h2>
          <h3 class="mdc-typography--subtitle2" style="margin: 0; color: #888;">
            {device.model}&nbsp;
          </h3>
          
          {#if !device.isSaved}
          <Badge 
            style="padding-left: 0.5em;"
            position="inset"
            
             aria-label="notification count">
            New device
            </Badge>
          {/if}
        </div>
        <div>
          <!-- <h5>{ device.isConnected ? 'USB Connected' : 'USB Disconnected'}</h5> -->
          <Icon class="material-icons" style="font-size: 2em; color: {device.isConnected ? 'green' : 'red'};">
            {device.isConnected ? 'usb' : 'usb_off'}
          </Icon>
        </div>
      </div>
      
  
    </Content>
  </PrimaryAction>
  <Actions>
    <ActionButtons>
      <Button
        variant="raised"
        on:click={() => lanuchScrcpy()}
        title="Start scrcpy"
      >
        <Label>Start</Label>
      </Button>
    </ActionButtons>
    <ActionIcons>
      {#if !device.isSaved}
      <IconButton
        class="material-icons"
        on:click={() => saveDevice(device)}
        title="Save"
      >
        save
      </IconButton>
      {:else}
      <IconButton
        class="material-icons"
        on:click={() => saveDevice(device)}
        title="Edit"
      >
        edit
      </IconButton>
      <IconButton
        class="material-icons"
        on:click={() => handleDeleteDevice(device)}
        title="Delete"
      >
        delete
      </IconButton>
      {/if}
      <IconButton
        class="material-icons"
        on:click={() => showConfig()}
        title="More options"
        >more_vert
      </IconButton>
    </ActionIcons>
  </Actions>
</Card>
