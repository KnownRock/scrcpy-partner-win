<script lang="ts">
  import IconButton from '@smui/icon-button'

  import { invoke } from '@tauri-apps/api/tauri'

  const commandKeyDict = {
    volume_up: {
      keyCode: 0x26,
      scanCode: 0x48,
      extendKeyFlag: 1,
      isAlt: true,
      isShift: false
    },
    volume_down: {
      keyCode: 0x28,
      scanCode: 0x50,
      extendKeyFlag: 1,
      isAlt: true,
      isShift: false
    },
    rotate_left: {
      keyCode: 0x25,
      scanCode: 0x4B,
      extendKeyFlag: 1,
      isAlt: true,
      isShift: false
    },
    rotate_right: {
      keyCode: 0x27,
      scanCode: 0x4D,
      extendKeyFlag: 1,
      isAlt: true,
      isShift: false
    },
    home: {
      // alt h
      keyCode: 0x48,
      scanCode: 0x23,
      extendKeyFlag: 1,
      isAlt: true,
      isShift: false
  
    },
    app_switch: {
      // alt s
      keyCode: 0x53,
      scanCode: 0x1F,
      extendKeyFlag: 1,
      isAlt: true,
      isShift: false
    },
    back: {
      // alt b
      keyCode: 0x42,
      scanCode: 0x30,
      extendKeyFlag: 0,
      isAlt: true,
      isShift: false
    },
    screen_on: {
      // shift alt o
      keyCode: 0x4F,
      scanCode: 0x18,
      extendKeyFlag: 0,
      isAlt: true,
      isShift: true
    },
    screen_off: {
      // alt o
      keyCode: 0x4F,
      scanCode: 0x18,
      extendKeyFlag: 0,
      isAlt: true,
      isShift: false
    },
    expend_notification: {
      // alt n
      keyCode: 0x4E,
      scanCode: 0x31,
      extendKeyFlag: 0,
      isAlt: true,
      isShift: false
    },
    aspect_ratio: {
      // alt w
      keyCode: 0x57,
      scanCode: 0x11,
      extendKeyFlag: 0,
      isAlt: true,
      isShift: false
    },
    power: {
      // alt p
      keyCode: 0x50,
      scanCode: 0x19,
      extendKeyFlag: 0,
      isAlt: true,
      isShift: false
    }
  
  }

  async function sendKey (command: string) {
    const opt = commandKeyDict[command]
    if (opt) {
      await invoke('sendkey', opt)
    }
  }

</script>
<div class="tool-container">


  <IconButton class="material-icons" on:click={() => sendKey('power')}>
    power_settings_new
  </IconButton>
  <IconButton class="material-icons" on:click={() => sendKey('volume_up')}>
    volume_up
  </IconButton>
  <IconButton class="material-icons" on:click={() => sendKey('volume_down')}>
    volume_down
  </IconButton>
  <IconButton class="material-icons" on:click={() => sendKey('rotate_left')}>
    rotate_left
  </IconButton>
  <IconButton class="material-icons" on:click={() => sendKey('rotate_right')}>
    rotate_right
  </IconButton>
  <IconButton class="material-icons" on:click={() => sendKey('aspect_ratio')}>
    aspect_ratio
  </IconButton>
  <IconButton class="material-icons" on:click={() => sendKey('expend_notification')}>
    menu_open
  </IconButton>
  <IconButton class="material-icons" on:click={() => sendKey('screen_off')}>
    mobile_off
  </IconButton>
  <IconButton class="material-icons" on:click={() => sendKey('screen_on')}>
    smartphone
  </IconButton>
  <IconButton class="material-icons" on:click={() => sendKey('back')}>
    arrow_back_ios_new
  </IconButton>
  <IconButton class="material-icons" on:click={() => sendKey('home')}>
      radio_button_unchecked
  </IconButton>
  <IconButton class="material-icons" on:click={() => sendKey('app_switch')}>
    crop_square
  </IconButton>

</div>