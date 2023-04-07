import { KeyEventType } from '../../types'
interface cmd {
  cmdType: 'scrcpy-cmd' | 'app-cmd' | 'scrcpy-control-cmd'
  cmdName: string
  opts?: Record<string, any>
}

type AddableItem = ({
  uiType: 'icon-button'
  icon: string
} & cmd) | {
  uiType: 'icon-button-2'
  icon: string

  cmds: [cmd, cmd]
}
export const addableItems: AddableItem[] = [{
  uiType: 'icon-button-2',
  icon: 'power_settings_new',

  cmds: [{
    cmdType: 'scrcpy-control-cmd',
    cmdName: 'keyevent',
    opts: {
      key: 26,
      keyEventType: KeyEventType.Down
    }
  }, {
    cmdType: 'scrcpy-control-cmd',
    cmdName: 'keyevent',
    opts: {
      key: 26,
      keyEventType: KeyEventType.Up
    }
  }]
}, {
  uiType: 'icon-button',
  icon: 'volume_up',

  cmdType: 'scrcpy-cmd',
  cmdName: 'volume_up'
}, {
  uiType: 'icon-button',
  icon: 'volume_down',

  cmdType: 'scrcpy-cmd',
  cmdName: 'volume_down'
}, {
  uiType: 'icon-button',
  icon: 'rotate_left',

  cmdType: 'scrcpy-cmd',
  cmdName: 'rotate_left'
}, {
  uiType: 'icon-button',
  icon: 'rotate_right',

  cmdType: 'scrcpy-cmd',
  cmdName: 'rotate_right'
}, {
  uiType: 'icon-button',
  icon: 'aspect_ratio',

  cmdType: 'scrcpy-cmd',
  cmdName: 'aspect_ratio'
}, {
  uiType: 'icon-button',
  icon: 'menu_open',

  cmdType: 'scrcpy-cmd',
  cmdName: 'expend_notification'
}, {
  uiType: 'icon-button',
  icon: 'mobile_off',

  cmdType: 'scrcpy-cmd',
  cmdName: 'screen_off'
}, {
  uiType: 'icon-button',
  icon: 'smartphone',

  cmdType: 'scrcpy-cmd',
  cmdName: 'screen_on'
}, {
  uiType: 'icon-button',
  icon: 'arrow_back_ios_new',

  cmdType: 'scrcpy-cmd',
  cmdName: 'back'
}, {
  uiType: 'icon-button',
  icon: 'radio_button_unchecked',

  cmdType: 'scrcpy-cmd',
  cmdName: 'home'
}, {
  uiType: 'icon-button',
  icon: 'crop_square',

  cmdType: 'scrcpy-cmd',
  cmdName: 'app_switch'
}, {
  uiType: 'icon-button',
  icon: 'close',

  cmdType: 'app-cmd',
  cmdName: 'exit'
}, {
  uiType: 'icon-button',
  icon: 'settings',

  cmdType: 'app-cmd',
  cmdName: 'open_settings'
}]

export const addableItems2: AddableItem[] = Array.from({ length: 9 }, (v, i) => {
  const layerButton: AddableItem = {
    uiType: 'icon-button',
    icon: `filter_${i + 1}`,

    cmdType: 'app-cmd',
    cmdName: 'set_layer',

    opts: {
      layer: i
    }
  }

  return layerButton
})

export const addableItems3: AddableItem[] = [{
  uiType: 'icon-button',
  icon: 'screenshot',

  cmdType: 'app-cmd',
  cmdName: 'capture'
}]

export const fullAddableItems = addableItems.concat(addableItems3).concat(addableItems2)
