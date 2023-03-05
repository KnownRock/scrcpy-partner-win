import { writable } from 'svelte/store'
import { generalDialogForm } from '.'
import { getDevices, lanuchSelf } from '../utils/devices'
import { saveConfig, type DeviceConfigExt } from '../utils/configs'
const store = writable<{
  show: boolean
  deviceId: string
} | {
  show: boolean
  deviceConfigId: string
} | {
  show: boolean
}>({
  show: false,
  deviceId: ''
})

async function getForm (
  currentDeviceId: string
): Promise<FormItem[]> {
  const devices = await getDevices('only saved')
  const currentDevice = devices.find((device) => device.id === currentDeviceId)
  // debugger
  const form: FormItem[] = [
    {
      type: 'header',
      label: 'Info',
      name: 'info'
    },
    {
      type: 'option',
      label: 'Device',
      name: 'device',
      options: devices.map((device) => {
        return { label: device.name, value: device.id }
      }),
      value: currentDeviceId,
      disabled: currentDevice !== undefined
    },
    {
      type: 'text',
      label: 'Name',
      name: 'name',
      value: 'New Config'
    },
    {
      type: 'header',
      label: 'General',
      name: 'general'
    },
    // {
    //   type: 'text',
    //   label: 'ADB Id',
    //   name: 'serial',
    //   value: currentDevice?.adbId ?? '',
    //   disabled: currentDevice?.adbId !== undefined
    // },
    {
      type: 'auto',
      label: 'Bit Rate',
      name: 'bit-rate',
      options: ['32M', '16M', '8M', '4M', '2M', '1M', '512K', '256K'],
      value: '8M'
    },
    {
      type: 'auto',
      label: 'Display Buffer',
      name: 'display-buffer',
      options: ['100', '50', '30', '20', '10', '5', '0'],
      value: '0'
    },
    {
      type: 'optional-auto',
      label: 'FPS',
      name: 'max-fps',
      options: ['144', '120', '75', '60', '30', '20', '15', '10', '5'],
      value: '60',
      enable: false
    },
    {
      type: 'optional-number',
      label: 'Max Size',
      name: 'max-size',
      value: 1080,
      enable: false
    },
    {
      type: 'optional-option',
      label: 'Orientation',
      name: 'lock-video-orientation',
      options: [
        { label: 'Natural orientation', value: '0' },
        { label: '90° counterclockwise', value: '1' },
        { label: '180° counterclockwise', value: '2' },
        { label: '90° clockwise', value: '3' }
      ],
      value: '0',
      enable: false
    },
    {
      type: 'header',
      label: 'Screen',
      name: 'screen'
    },
    {
      type: 'switch',
      label: 'Always on top',
      name: 'always-on-top',
      value: false
    },
    {
      type: 'switch',
      label: 'Fullscreen',
      name: 'fullscreen',
      value: false
    },
    {
      type: 'switch',
      label: 'Window Borderless',
      name: 'window-borderless',
      value: false
    },
    {
      type: 'optional-text',
      label: 'Window Title',
      name: 'title',
      value: '',
      enable: false
    },

    {
      type: 'header',
      label: 'Window',
      name: 'window'
    },
    {
      type: 'optional-number',
      label: 'Position X',
      name: 'window-x',
      value: 0,
      enable: false
    },
    {
      type: 'optional-number',
      label: 'Position Y',
      name: 'window-y',
      value: 0,
      enable: false
    },
    {
      type: 'optional-number',
      label: 'Width',
      name: 'width',
      value: 0,
      enable: false
    }, {
      type: 'header',
      label: 'Advanced',
      name: 'advanced'
    }, {
      type: 'switch',
      label: 'Autosave Location',
      name: 'spw-autosave-location',
      value: false
    }
  ]

  form.forEach((item) => {
    if (item.type === 'option') {
      item.defaultValue = item.value
    }
  })

  return form
}

function formEntityToArgs (entity: Parameters<DialogFormButton['callback']>[0]): string[] {
  const args: string[] = []
  Object.keys(entity).forEach((key) => {
    const value = entity[key]
    if (value === undefined) {
      return
    }
    if (value === true) {
      args.push(`--${key}`)
    } else if (value !== false) {
      args.push(`--${key}=${value as string | number}`)
    }
  })
  return args
}

store.subscribe(value => {
  (async () => {
    console.log('config-form.ts: store.subscribe: value:', value)

    const formItems = await getForm(value.deviceId)

    generalDialogForm.set({
      title: 'Config',
      show: value.show,
      formItems,
      buttons: [
        {
          label: 'Cancel',
          action: 'cancel',
          callback: (entity, formItems) => {
            return true
          }
        },
        {
          label: 'Save',
          action: 'save',
          defaultAction: true,
          callback: async (entity, formItems) => {
            const deviceConfigItems = Object.entries(entity)
              .filter(([key, value]) => {
                return !['device', 'name'].includes(key)
              })

            const deviceConfig: DeviceConfigExt = {
              id: '',
              deviceId: entity.device,
              name: entity.name,

              createdAt: null,
              updatedAt: null,
              lastSeenAt: null

            }

            await saveConfig(deviceConfig)

            return true
          }
        }
      ],
      cancelCallback: (response, formItems) => {
        return true
      }
    })
  })().then(() => {
    console.log('config-form.ts: store.subscribe: then')
  }).catch(error => {
    console.log('config-form.ts: store.subscribe: catch: error:', error)
  })
})

export default store
