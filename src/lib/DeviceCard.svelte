<script lang="ts">
  import Button, { Label } from '@smui/button'
  import Card, {
    Content,
    PrimaryAction,
    Actions,
    ActionButtons,
    ActionIcons
  } from '@smui/card'
  import IconButton from '@smui/icon-button'
  // TODO: move all type to global.d.ts
  import { lanuchSelf, type Device } from '../utils/devices'
  import { configForm } from '../store/index'
  export let device: Device

  async function lanuchScrcpy () {
    lanuchSelf([`-s${device.adbId}`])
  }

  async function saveDevice (device) {
    console.log('saveDevice', device)
  }

  function showConfig () {
    configForm.set({
      show: true,
      deviceId: device.id
    })
  }
</script>

<Card>
  <PrimaryAction on:click={() => 1}>
    <!-- <Media class="card-media-16x9" aspectRatio="16x9" /> -->
    <Content class="mdc-typography--body2">
      <h2 class="mdc-typography--headline6" style="margin: 0;">
        <!-- A card with media. -->
        {device.name}
      </h2>
      <h3 class="mdc-typography--subtitle2" style="margin: 0; color: #888;">
        {device.model}
      </h3>
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
      <IconButton
        class="material-icons"
        on:click={() => saveDevice(device)}
        title="Save"
      >
        save
      </IconButton>
      <IconButton
        class="material-icons"
        on:click={() => showConfig()}
        title="More options"
        >more_vert
      </IconButton>
    </ActionIcons>
  </Actions>
</Card>
