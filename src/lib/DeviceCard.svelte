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
  import {
    deleteDevice, lanuchSelf, connectTcpipDevice, type DeviceExt
  } from '../utils/devices'
  import { configForm, configSelectForm, confirmDialog, deviceForm } from '../store/index'
  import { getContext } from 'svelte'
  import Menu from '@smui/menu'
  import List, { Item, Separator, Text } from '@smui/list'
  import { deleteConfigById, type DeviceConfigExt } from '../utils/configs'
  const freshDevices = getContext('freshDevices') as () => void

  export let device: DeviceExt

  // eslint-disable-next-line
  export let config: DeviceConfigExt | undefined = undefined

  async function sleep (ms) {
    return new Promise(resolve => setTimeout(resolve, ms))
  }

  async function lanuchScrcpy () {
    if (config) {
      lanuchSelf([
        `--spw-config-id=${config.id}`
      ])

      return
    }


    lanuchSelf([
      `--serial=${device.adbId}`,
      '--window-title',
      `${device.name}`
    ])
  }

  async function connectAdb () {
    await connectTcpipDevice(device.adbId)
    // wait adb devices refresh
    await sleep(1000)
    await freshDevices()
  }

  async function disconnectAdb () {
    await connectTcpipDevice(device.adbId, false)
    await sleep(1000)
    await freshDevices()
  }

  async function saveDevice (type? : 'copy') {
    // console.log('saveDevice', device)
    if (config !== undefined) {
      if (type === 'copy') {
        configForm.set({
          show: true,
          deviceConfigId: config.id,
          type: 'copy',
          callback: () => {
            freshDevices()
          }
        })
        return
      }

      configForm.set({
        show: true,
        deviceConfigId: config.id,
        type: 'edit',
        callback: () => {
          freshDevices()
        }
      })
      return
    }

    let editDevice = device
    if (type === 'copy') {
      editDevice = {
        ...device,
        id: '',
        name: `${device.name} (copy)`,
        updatedAt: null,
        createdAt: null
        // seenAt: null
      }
    }


    deviceForm.set({
      show: true,
      device: editDevice,
      callback: async (device) => {
        await freshDevices()
      }
    })
  }

  async function handleDeleteDevice () {
    if (config !== undefined) {
      confirmDialog.set({
        show: true,
        title: 'Delete device',
        message: `Are you sure to delete config [${config.name}]?`,
        okCallback: async () => {
          // console.log('delete device', device)
          await deleteConfigById(config?.id ?? '')

          await freshDevices()
        }
      })
      return
    }


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

  let clicked = ''
  let menu: Menu
  let actionButton

  function showConfig () {
    configSelectForm.set({
      show: true,
      deviceId: device.id
    })
  }
</script>

<Card>
  <PrimaryAction on:click={() => 1}>
    <!-- <Media class="card-media-16x9" aspectRatio="16x9" /> -->
    <Content class="mdc-typography--body2" >
      <div style="display: flex; justify-content: space-between; align-items: center;">
        {#if config !== undefined}
        <div >
          <h2 class="mdc-typography--headline6" style="margin: 0;">
            <!-- A card with media. -->
            {config.name}&nbsp;
          </h2>
          <h3 class="mdc-typography--subtitle2" style="margin: 0; color: #888;">
            {device.name}&nbsp;
          </h3>
        </div>
        {:else}

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

        {/if}

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
    {#if !device.isTcpipDevice || device.isConnected}
    <ActionButtons style="margin-right:10px">
      <Button
        disabled={!device.isConnected}
        variant="raised"
        on:click={() => lanuchScrcpy()}
        title="Start scrcpy"
      >
        <Label>Start</Label>
      </Button>
    </ActionButtons>
    {/if}
    {#if device.isTcpipDevice && !device.isConnected}
      <ActionButtons>
        <Button
          disabled={device.isConnected}
          variant="raised"
          on:click={() => connectAdb()}
          title="Start scrcpy"
        >
          <Label>Connect</Label>
        </Button>
      </ActionButtons>
    {/if}
    {#if device.isTcpipDevice && device.isConnected}
      <ActionButtons>
        <Button
          disabled={!device.isConnected}
          variant="raised"
          color="secondary"
          on:click={() => disconnectAdb()}
          title="Start scrcpy"
        >
          <Label>Disconnect</Label>
        </Button>
      </ActionButtons>
    {/if}
    <ActionIcons>
      {#if !device.isSaved}

        <IconButton
          class="material-icons"
          on:click={() => saveDevice()}
          title="Save"
        >
          save
        </IconButton>

      {:else}


        <div>
          <IconButton
            bind:this={actionButton}
            class="material-icons"
            on:click={() => menu.setOpen(true)}
            title="More options"
            >more_vert
          </IconButton>
          <Menu 
            bind:this={menu} 
            on:SMUI:closed={() => {
              clicked = ''
              menu.setOpen(false)
          }}>
            <List>
              <Item on:SMUI:action={() => saveDevice()}>
                <Text>Edit</Text>
              </Item>
              <Item on:SMUI:action={() => saveDevice('copy')}>
                <Text>
                  Duplicate
                </Text>
              </Item>
              <Item on:SMUI:action={() => handleDeleteDevice()}>
                <Text style="color: red;">
                  Delete
                </Text>
              </Item>
            
              <Separator />
              <Item on:SMUI:action={() => showConfig()}>
                <Text>Config</Text>
              </Item>
            </List>
          </Menu>
        </div>

      {/if}
      
    </ActionIcons>
  </Actions>
</Card>

<style>
  .device-actions div {
    margin: 10px;
  }
</style>
