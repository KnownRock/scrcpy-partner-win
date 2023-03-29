

<div class="container">
  <div class="item">
    <Textfield
      style="flex:1"
      label={$t('Delay(ms)')}
      bind:value={delay}
      type="number"
    />
    <Button on:click={() => addDelay()}>
      {$t('Add')}
    </Button>
  </div>

  <div class="item">
    <Textfield
      style="flex:1"
      label={$t('Key Event Code')}
      bind:value={keyCode}
      type="number"
    />
    {#key keyType}
    <Select
      label={$t('Key Event Type')}
      bind:value={keyType}
    >
      <Option value={KeyEventType.Down}>{$t('Down')}</Option>
      <Option value={KeyEventType.Up}>{$t('Up')}</Option>
    </Select>
    {/key}


    <Button on:click={addKeyEvent}>
      {$t('Add')}
    </Button>

  </div>  

  <!-- motion event -->
  <div class="item">
    <Textfield
      style="flex:1"
      label={$t('Motion Event X')}
      bind:value={motionX}
      type="number"
    />
    <Textfield
      style="flex:1"
      label={$t('Motion Event Y')}
      bind:value={motionY}
      type="number"
    />

    {#key keyType}
    <Select
      label={$t('Motion Event Type')}
      bind:value={motionType}
    >
      <Option value={MotionType.Down}>{$t('Down')}</Option>
      <Option value={MotionType.Up}>{$t('Up')}</Option>
      <Option value={MotionType.Move}>{$t('Move')}</Option>
      
    </Select>
    {/key}

    <Button on:click={() => addMotionEvent()}>
      {$t('Add')}
    </Button>
  </div>


  <!-- adb_cmd -->
  <div class="item">
    <Textfield
      style="flex:1"
      label={$t('ADB Command')}
      bind:value={adbCmd}
    />
    <Button on:click={() => addAdbCmd()}>
      {$t('Add')}
    </Button>
  </div>
</div>

<script lang="ts">
  import Button from '@smui/button'
  import Textfield from '@smui/textfield'
  import type { RecordOperation } from '../../types'
  import { KeyEventType, MotionType } from '../../types'
  import { t } from 'svelte-i18n'
  import Select, { Option } from '@smui/select'

  export let addOperation: (operation: RecordOperation) => void

  let delay = 1000
  let keyCode = 0
  let keyType = KeyEventType.Down
  let adbCmd = ''

  let motionX = 0
  let motionY = 0
  let motionType = MotionType.Down
  
  function addDelay () {
    addOperation({
      type: 'delay',
      ms: delay
    })
  }

  function addKeyEvent () {
    addOperation({
      type: 'keyevent',
      key: keyCode,
      keyEventType: keyType
    })
  }

  function addAdbCmd () {
    addOperation({
      type: 'adb_cmd',
      cmd: adbCmd
    })
  }

  function addMotionEvent () {
    addOperation({
      type: 'motion',
      x: motionX,
      y: motionY,
      motionType
    })
  }

  </script>

<style>
  .container {
    padding:10px;
  }

  .item{
    display:flex; align-items:center; justify-content:space-between
  }
</style>