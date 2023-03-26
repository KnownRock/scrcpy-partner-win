<div>
  {#each keyButtons as button}
    <IconButton 
      class="material-icons"
      on:mousedown={() => {
        execute({
          type: 'keyevent',
          key: button.adbKey,
          keyEventType: KeyEventType.Down
        })
      }}
      
      on:mouseup={() => {
        execute({
          type: 'keyevent',
          key: button.adbKey,
          keyEventType: KeyEventType.Up
        })
      }}
      >


      {button.icon}
    </IconButton>
  {/each}
</div>

<script lang="ts">
  import IconButton from '@smui/icon-button'
  import { KeyEventType, type RecordOperation } from '../../types'

  export let execute: (operation: RecordOperation) => void

  const keyButtons = [{
    label: 'Home',
    command: 'home',
    icon: 'radio_button_unchecked',
    adbKey: 3
  }, {
    label: 'Back',
    command: 'back',
    icon: 'arrow_back',
    adbKey: 4
  }, {
    label: 'Menu',
    command: 'menu',
    icon: 'menu',
    adbKey: 82
  }, {
    label: 'Power',
    command: 'power',
    icon: 'power_settings_new',
    adbKey: 26
  }, {
    label: 'Volume Up',
    command: 'volumeUp',
    icon: 'volume_up',
    adbKey: 24
  }, {
    label: 'Volume Down',
    command: 'volumeDown',
    icon: 'volume_down',
    adbKey: 25
  }]
</script>