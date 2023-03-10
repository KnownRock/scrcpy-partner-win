export const argsTemplate = {
  '--max-size': Number,
  '-m': '--max-size',
  '--bit-rate': String,
  '-b': '--bit-rate',
  '--max-fps': Number,
  '--print-fps': Boolean,
  '--crop': String,
  '--lock-video-orientation': Number,
  '--encoder': String,
  '--record': String,
  '-r': '--record',
  '--no-display': Boolean,
  '--display-buffer': Number,
  '--tcpip': String,
  '--serial': String,
  '-s': '--serial',

  '--window-title': String,
  '--window-x': Number,
  '--window-y': Number,
  '--window-width': Number,
  '--window-height': Number,
  '--window-borderless': Boolean,
  '--always-on-top': Boolean,
  '--fullscreen': Boolean,
  '-f': '--fullscreen',
  '--rotation': Number,

  '--no-control': Boolean,
  '--display': Number,
  '--stay-awake': Boolean,
  '-w': '--stay-awake',
  '--turn-screen-off': Boolean,
  '-S': '--turn-screen-off',
  '--power-off-on-close': Boolean,
  '--no-power-on': Boolean,
  '--show-touches': Boolean,
  '-t': '--show-touches',
  '--disable-screensaver': Boolean,

  '--hid-keyboard': Boolean,
  '-K': '--hid-keyboard',
  '--hid-mouse': Boolean,
  '-M': '--hid-mouse',
  '--otg': Boolean,

  '--prefer-text': Boolean,
  '--raw-key-events': Boolean,
  '--no-key-repeat': Boolean,
  '--forward-all-clicks': Boolean,

  '--spw-config-id': String

}

export const argsTags = {
  general: ['--max-size', '--bit-rate', '--max-fps', '--print-fps', '--crop', '--lock-video-orientation', '--encoder', '--record', '--no-display', '--display-buffer'],
  window: ['--window-title', '--window-x', '--window-y', '--window-width', '--window-height', '--window-borderless', '--always-on-top', '--fullscreen', '--rotation'],
  power: ['--no-control', '--display', '--stay-awake', '--turn-screen-off', '--power-off-on-close', '--no-power-on', '--show-touches', '--disable-screensaver'],
  input: ['--hid-keyboard', '--hid-mouse', '--otg', '--prefer-text', '--raw-key-events', '--no-key-repeat', '--forward-all-clicks']
}

export const formArgsTagsOrder = [
  'general',
  'window',
  'power',
  'input'
]

export const argsDescriptions = {
  '--max-size': 'Set window maximum size (e.g. 1024)',
  '-m': 'Set window maximum size (e.g. 1024)',
  '--bit-rate': 'Set video bit rate (e.g. 2M or 2000k)',
  '-b': 'Set video bit rate (e.g. 2M or 2000k)',
  '--max-fps': 'Set max frame rate (default 60)',
  '--print-fps': 'Show frame rate on stdout',
  '--crop': 'Crop the video to a given width:height:x:y (default: disabled)',
  '--lock-video-orientation': 'Lock video orientation (default: 0, unlocked)',
  '--encoder': 'Set video encoder (e.g. OMX.h264.encoder, OMX.qcom.video.encoder.avc, OMX.Exynos.avc.enc)',
  '--record': 'Record video to a file',
  '-r': 'Record video to a file',
  '--no-display': 'Disable display',
  '--display-buffer': 'Set display buffer size (default 8)',
  '--tcpip': 'Connect to device via TCP/IP (e.g. 5555)',
  '--serial': 'Connect to device via serial',
  '-s': 'Connect to device via serial',

  '--window-title': 'Set window title',
  '--window-x': 'Set window x position',
  '--window-y': 'Set window y position',
  '--window-width': 'Set window width',
  '--window-height': 'Set window height',
  '--window-borderless': 'Set window borderless',
  '--always-on-top': 'Set window always on top',
  '--fullscreen': 'Start in fullscreen',
  '-f': 'Start in fullscreen',
  '--rotation': 'Set window rotation (default: 0)',

  '--no-control': 'Disable device control',
  '--display': 'Set display number (default: 0)',
  '--stay-awake': 'Keep device awake',
  '-w': 'Keep device awake',
  '--turn-screen-off': 'Turn screen off',
  '-S': 'Turn screen off',
  '--power-off-on-close': 'Power off on close',
  '--no-power-on': 'Do not power on the device',
  '--show-touches': 'Show touches',
  '-t': 'Show touches',
  '--disable-screensaver': 'Disable screensaver',

  '--hid-keyboard': 'Enable HID keyboard',
  '-K': 'Enable HID keyboard',
  '--hid-mouse': 'Enable HID mouse',
  '-M': 'Enable HID mouse',
  '--otg': 'Enable OTG',
  '--prefer-text': 'Prefer text (e.g. copy/paste) to control device',
  '--raw-key-events': 'Send raw key events',
  '--no-key-repeat': 'Disable key repeat',
  '--forward-all-clicks': 'Forward all clicks (default: only forward clicks on the device screen)'
}

// FIXME: make type more strict
const formItemOverride: Record<string, FormItem> = {
  '--max-size': {
    type: 'optional-auto-number',
    label: 'Max Size',
    name: '--max-size',
    value: 1080,
    enable: false,
    options: [2160, 1440, 1080, 720, 480, 360, 240]
  },
  '--bit-rate': {
    type: 'optional-auto',
    label: 'Bit Rate',
    name: '--bit-rate',
    value: '8M',
    enable: false,
    options: ['64M', '32M', '16M', '8M', '4M', '2M', '1M', '512k', '256k', '128k', '64k']
  },
  '--max-fps': {
    type: 'optional-auto-number',
    label: 'Max FPS',
    name: '--max-fps',
    value: 60,
    enable: false,
    options: [144, 120, 90, 75, 60, 30, 15, 10, 5]
  },
  '--lock-video-orientation': {
    type: 'optional-option',
    label: 'Lock Video Orientation',
    name: '--lock-video-orientation',
    value: '0',
    enable: false,
    options: [
      { label: 'Natural orientation', value: '0' },
      { label: '90° counterclockwise', value: '1' },
      { label: '180° counterclockwise', value: '2' },
      { label: '90° clockwise', value: '3' }
    ]
  }
}

export function getFormItems (
  formValue: Record<string, any>
): FormItem[] {
  const formItems: FormItem[] = []

  formArgsTagsOrder.forEach((tag) => {
    formItems.push({
      type: 'header',
      label: tag.replace(/^[a-z]/, (match) => match.toUpperCase()),
      name: tag
    })

    argsTags[tag].forEach((arg: string) => {
      const template = argsTemplate[arg]

      const label = arg
        .replace(/--([a-z])/g, (match, p1) => p1.toUpperCase())
        .replace(/-([a-z])/g, (match, p1) => ' ' + ((p1 ?? '') as string).toUpperCase())

      if (formItemOverride[arg] !== undefined) {
        formItems.push({
          ...formItemOverride[arg],
          // FIXME: make type more strict
          enable: formValue[arg] !== undefined,
          value: formValue[arg] ?? formItemOverride[arg].value,
          description: argsDescriptions[arg]
        })
      } else if (template === Number) {
        formItems.push({
          type: 'optional-number',
          label,
          name: arg,
          // value: 0,
          // enable: false
          value: formValue[arg] ?? 0,
          enable: formValue[arg] !== undefined,
          description: argsDescriptions[arg]
        })
      } else if (template === String) {
        formItems.push({
          type: 'optional-text',
          label,
          name: arg,
          // value: '',
          // enable: false
          value: formValue[arg] ?? '',
          enable: formValue[arg] !== undefined,
          description: argsDescriptions[arg]
        })
      } else if (template === Boolean) {
        formItems.push({
          type: 'optional-switch',
          label,
          name: arg,
          // value: false,
          // enable: false
          value: formValue[arg] ?? false,
          enable: formValue[arg] !== undefined,
          description: argsDescriptions[arg]
        })
      }
    })
  })

  return formItems
}
