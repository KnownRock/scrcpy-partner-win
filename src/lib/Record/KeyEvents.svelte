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
    icon: 'home',
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
  }, {
    // https://stackoverflow.com/questions/7585105/turn-on-screen-on-device/41198273#41198273
    label: 'Wake Up',
    command: 'wakeUp',
    icon: 'brightness_5',
    adbKey: 224
  }, {
    label: 'App Switch',
    command: 'appSwitch',
    icon: 'swap_horiz',
    adbKey: 187
  }, {
    label: 'Calulator',
    command: 'calculator',
    icon: 'calculate',
    adbKey: 210
  }, {
    label: 'Calendar',
    command: 'calendar',
    icon: 'event',
    adbKey: 208
  }, {
    label: 'Call',
    command: 'call',
    icon: 'call',
    adbKey: 5
  }, {
    label: 'Camera',
    command: 'camera',
    icon: 'camera_alt',
    adbKey: 27
  }, {
    label: 'Explorer',
    command: 'explorer',
    // browser
    icon: 'explore',
    adbKey: 64
  },
  // KEYCODE_HEADSETHOOK
  {
    label: 'Headset Hook',
    command: 'headsetHook',
    icon: 'headset',
    adbKey: 79
  },
  // KEYCODE_HELP
  {
    label: 'Help',
    command: 'help',
    icon: 'help',
    adbKey: 259
  }, {
    label: 'Mail',
    command: 'mail',
    icon: 'mail',
    adbKey: 33
  },

  // KEYCODE_MUSIC
  {
    label: 'Music',
    command: 'music',
    icon: 'music_note',
    adbKey: 209
  },

  // KEYCODE_MEDIA_PAUSE
  {
    label: 'Media Pause',
    command: 'mediaPause',
    icon: 'pause',
    adbKey: 127
  },
  // KEYCODE_MEDIA_PLAY
  {
    label: 'Media Play',
    command: 'mediaPlay',
    icon: 'play_arrow',
    adbKey: 126
  },
  // KEYCODE_MEDIA_PLAY_PAUSE
  {
    label: 'Media Play/Pause',
    command: 'mediaPlayPause',
    icon: 'play_arrow',
    adbKey: 85
  },
  // KEYCODE_MEDIA_RECORD
  {
    label: 'Media Record',
    command: 'mediaRecord',
    icon: 'fiber_manual_record',
    adbKey: 130
  },
  // KEYCODE_MEDIA_STOP
  {
    label: 'Media Stop',
    command: 'mediaStop',
    icon: 'stop',
    adbKey: 86
  },
  // KEYCODE_MEDIA_FAST_FORWARD
  {
    label: 'Media Fast Forward',
    command: 'mediaFastForward',
    icon: 'fast_forward',
    adbKey: 90
  },
  // KEYCODE_MEDIA_NEXT
  {
    label: 'Media Next',
    command: 'mediaNext',
    icon: 'skip_next',
    adbKey: 87
  },
  // KEYCODE_MEDIA_PREVIOUS
  {
    label: 'Media Previous',
    command: 'mediaPrevious',
    icon: 'skip_previous',
    adbKey: 88
  },
  // KEYCODE_MEDIA_REWIND
  {
    label: 'Media Rewind',
    command: 'mediaRewind',
    icon: 'fast_rewind',
    adbKey: 89
  },
  // KEYCODE_MEDIA_SKIP_BACKWARD
  {
    label: 'Media Skip Backward',
    command: 'mediaSkipBackward',
    icon: 'fast_rewind',
    adbKey: 273
  },
  // KEYCODE_MEDIA_SKIP_FORWARD
  {
    label: 'Media Skip Forward',
    command: 'mediaSkipForward',
    icon: 'fast_forward',
    adbKey: 272
  },
  // KEYCODE_MEDIA_STEP_BACKWARD
  {
    label: 'Media Step Backward',
    command: 'mediaStepBackward',
    icon: 'fast_rewind',
    adbKey: 275
  },
  // KEYCODE_MEDIA_STEP_FORWARD
  {
    label: 'Media Step Forward',
    command: 'mediaStepForward',
    icon: 'fast_forward',
    adbKey: 274
  },

  // KEYCODE_NOTIFICATION
  {
    label: 'Notification',
    command: 'notification',
    icon: 'notifications',
    adbKey: 83
  },

  // KEYCODE_PASTE
  {
    label: 'Paste',
    command: 'paste',
    icon: 'content_paste',
    adbKey: 279
  },
  
  {
    label: 'Mute',
    command: 'mute',
    icon: 'volume_off',
    adbKey: 91
  }, {
    label: 'Music',
    command: 'music',
    icon: 'music_note',
    adbKey: 209
  }, {
    label: 'Notification',
    command: 'notification',
    icon: 'notifications',
    adbKey: 83
  }, {
    label: 'Search',
    command: 'search',
    icon: 'search',
    adbKey: 84
  }, {
    label: 'Speakerphone',
    command: 'speakerphone',
    icon: 'phone_in_talk',
    adbKey: 8
  }, {
    label: 'Stop',
    command: 'stop',
    icon: 'stop',
    adbKey: 86
  }, {
    label: 'Voice Dial',
    command: 'voiceDial',
    icon: 'mic',
    adbKey: 85
  }
  ]
</script>