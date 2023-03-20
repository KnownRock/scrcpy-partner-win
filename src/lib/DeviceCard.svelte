<script lang="ts">
  import Button, { Icon, Label } from '@smui/button'
  import Badge from '@smui-extra/badge'
  import { t } from 'svelte-i18n'
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
  import { createMsLink } from '../utils/app'
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

  async function createLink () {
    if (config) {
      createMsLink(`spw-[${config.name}]`,
        [`--spw-config-id=${config.id}`])
    } else {
      createMsLink(`spw-[${device.name}]`,
        [`--serial=${device.adbId}`])
    }
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
        title: $t('Delete config'),
        message: `${$t('Are you sure to delete config')} [${config.name}]?`,
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
      title: $t('Delete device'),
      message: `${$t('Are you sure to delete device')} [${device.name}]?`,
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
            {$t('New device')}
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
        <Label>{$t('Start')}</Label>
      </Button>
    </ActionButtons>
    {/if}
    {#if device.isTcpipDevice && !device.isConnected}
      <ActionButtons>
        <Button
          disabled={device.isConnected}
          variant="raised"
          color="secondary"
          on:click={() => connectAdb()}
          title="Start scrcpy"
        >
          <Label>
            <!-- Connect -->
            {$t('Connect')}
          </Label>
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
          <Label>
            <!-- Disconnect -->
            {$t('Disconnect')}
          </Label>
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
              <!-- createMsLink -->
              

              <Item on:SMUI:action={() => saveDevice()}>
                <Text>
                  <!-- Edit -->
                  {$t('Edit')}
                </Text>
              </Item>
              <Item on:SMUI:action={() => saveDevice('copy')}>
                <Text>
                  <!-- Duplicate -->
                  {$t('Duplicate')}
                </Text>
              </Item>
              <Item on:SMUI:action={() => handleDeleteDevice()}>
                <Text style="color: red;">
                  <!-- Delete -->
                  {$t('Delete')}
                </Text>
              </Item>

              <Item on:SMUI:action={() => createLink()}>
                <Text style="color: blue;">
                <!-- Create Link -->
                {$t('Create Link')}
              </Text>
              </Item>

              {#if !config}
            
              <Separator />
              <Item on:SMUI:action={() => showConfig()}>
                <Text>
                  <!-- Config -->
                  {$t('Config')}
                </Text>
              </Item>

              {/if}
            </List>
          </Menu>
        </div>

      {/if}
      
    </ActionIcons>
  </Actions>
</Card>

<style>
</style>
