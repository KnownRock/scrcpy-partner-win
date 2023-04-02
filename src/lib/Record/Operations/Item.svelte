<div  class="operation-item">
  <div class="operation-icons">
    <Icon class="material-icons" style="vertical-align: middle;">
      {iconDict[operation.type]}
    </Icon>
    {#if operation.type === 'tap'}
      <Icon class="material-icons" style="vertical-align: middle;">
        location_searching
      </Icon>
    {:else if operation.type === 'keyevent'}
      <Icon 
        onclick={() => handleKeyEventTypeToggle()}
        class="material-icons" style="vertical-align: middle;">
        {keyIconDict[operation.keyEventType]}
      </Icon>
    {:else if operation.type === 'motion'}
      <Icon 
        onclick={() => handleMotionTypeToggle()}
        class="material-icons" style="vertical-align: middle;">
        {motionIconDict[operation.motionType]}
      </Icon>
      <Icon class="material-icons" style="vertical-align: middle;">
        location_searching
      </Icon>
    {/if}

  </div>
  <div class="operation-type">
    {#if operation.type === 'keyevent'}
      <Autocomplete
        {options}
        label={$t('Key Code')}
        getOptionLabel={(option) => keyCodeToString(option)}
        bind:value={operation.key}
      />

    {:else if operation.type === 'am_start'}
      <TextField
        label={$t('Package Name')}
        bind:value={operation.packageName}
        style="width: 100%"
      />

    {:else if operation.type === 'motion' || operation.type === 'tap'}
      <div style="display: flex;">
        <TextField
          label={$t('X')}
          bind:value={operation.x}
          style="width: 100%"
          type="number"
        />
        <TextField
          label={$t('Y')}
          bind:value={operation.y}
          style="width: 100%"
          type="number"
        />
      </div>

    {:else if operation.type === 'delay'}
      <TextField
        style="width: 100%"
        label={$t('Delay(ms)')}
        bind:value={operation.ms}
        type="number"
      />


    {:else if operation.type === 'exec_script'}
      <!-- <span>{operation.name}</span> -->
      <!-- <Label style="margin-right: 10px;">{operation.name}</Label> -->
      <TextField 
      style="width: 100%" 
      label={$t('Script Name')} 
      bind:value={operation.name} 
      disabled
      />
      <Button on:click={() => selectScript()}>
        {$t('Select')}
      </Button>
    {:else if operation.type === 'message'}
      <!-- <span>{operation.message}</span> -->
      <TextField
        style="width: 100%"
        label={$t('Message')}
        bind:value={operation.message}
        />

    {:else if operation.type === 'adb_cmd'}
      <!-- <span>{operation.cmd}</span> -->
      <TextField
        style="width: 100%"
        label={$t('ADB Command')}
        bind:value={operation.cmd}
        />

    {:else}
      <span>Unknown operation</span>
    {/if}
  </div>


</div>

<script lang="ts">
  import Autocomplete from '@smui-extra/autocomplete'
  import { Icon } from '@smui/fab'
  import { KeyEventType, MotionType, type RecordOperation } from '../../../types'
  import { keyCodeToString, getOptions } from '../../../utils/keycode'
  import { t } from 'svelte-i18n'
  import TextField from '@smui/textfield'
  import { generalDialogForm } from '../../../store'
  import { setDialog } from '../../../utils/record'
  import Button from '@smui/button'
  export let operation: RecordOperation

  const iconDict = {
    tap: 'touch_app',
    keyevent: 'keyboard',
    am_start: 'launch',
    motion: 'gesture',
    delay: 'timer',
    exec_script: 'code',
    adb_cmd: 'adb',
    message: 'message'
  }

  const options = getOptions().map((option) => option.id)

  const motionIconDict = {
    [MotionType.Down]: 'swipe_down',
    [MotionType.Up]: 'swipe_up',
    [MotionType.Move]: 'swipe'
  }

  const keyIconDict = {
    [KeyEventType.Down]: 'arrow_downward',
    [KeyEventType.Up]: 'arrow_upward'
  }

  function handleMotionTypeToggle () {
    if (operation.type !== 'motion') {
      return
    }

    if (operation.motionType === MotionType.Down) {
      operation.motionType = MotionType.Move
    } else if (operation.motionType === MotionType.Move) {
      operation.motionType = MotionType.Up
    } else if (operation.motionType === MotionType.Up) {
      operation.motionType = MotionType.Down
    }
  }

  function handleKeyEventTypeToggle () {
    if (operation.type !== 'keyevent') {
      return
    }

    if (operation.keyEventType === KeyEventType.Down) {
      operation.keyEventType = KeyEventType.Up
    } else if (operation.keyEventType === KeyEventType.Up) {
      operation.keyEventType = KeyEventType.Down
    }
  }

  function selectScript () {
    const buttons = [{
      label: 'Open',
      callback: async (record) => {
        if (operation.type === 'exec_script') {
          operation.name = record.name
          operation.scriptId = record.id
        }

        generalDialogForm.set({
          show: false,
          formItems: [],
          buttons: [],
          cancelCallback: () => {
            return true
          }
        })
      }
    }]
    setDialog(buttons)
  }
  
</script>

<style>
  .operation-item {

    display: flex;
    align-items: center;
    padding: 5px;
    border-radius: 4px;
    border: 1px solid #000000;
    margin: 5px;
    background-color: #ffffff;
    justify-content: space-between;
    padding: 0 10px;
    height: 60px;

    width: 100%;

    font-size: 1.2em;
  }

  .operation-icons {
    width: 100px;
  }

  .operation-type {
    width: 100%;
    flex: 1;
    display: flex;
    justify-content: flex-end;
    align-items: center;
  }

</style>