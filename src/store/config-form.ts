
import { writable } from 'svelte/store'
import { generalDialogForm } from '.'
import { getDevices } from '../utils/devices'
import { saveConfig, type DeviceConfigValueExt, type DeviceConfigExt, getConfig, saveConfigItems, getConfigItems } from '../utils/configs'
const store = writable<{
  show: boolean
  deviceId: string
  type: 'new-by-device'
  callback?: () => void
} | {
  show: boolean
  deviceConfigId: string
  type: 'edit' | 'copy'
  callback?: () => void
} | {
  show: boolean
  type: 'new'
  callback?: () => void
}>({
      show: false,
      type: 'new',
      callback () {}
    })

async function getForm (
  currentDeviceId?: string,
  currentDeviceConfig?: DeviceConfigExt
): Promise<FormItem[]> {
  const devices = await getDevices('only saved')
  const currentDevice = devices.find((device) => device.id === currentDeviceId)
  const currentDeviceConfigValues = await getConfigItems(currentDeviceConfig?.id ?? '')
  const formValue = currentDeviceConfigValues.reduce<Record<string, any>>((acc, item) => {
    acc[item.key] = JSON.parse(item.value)
    return acc
  }, {})

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
      value: currentDeviceId ?? '',
      disabled: currentDevice !== undefined
    },
    {
      type: 'text',
      label: 'Name',
      name: 'name',
      value: currentDeviceConfig?.name ?? 'New Config'
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
      value: formValue['bit-rate'] ?? '8M'
    },
    {
      type: 'auto',
      label: 'Display Buffer',
      name: 'display-buffer',
      options: ['100', '50', '30', '20', '10', '5', '0'],
      value: formValue['display-buffer'] ?? '0'
    },
    {
      type: 'optional-auto',
      label: 'FPS',
      name: 'max-fps',
      options: ['144', '120', '75', '60', '30', '20', '15', '10', '5'],
      value: formValue['max-fps'] ?? '60',
      enable: formValue['max-fps'] !== undefined
    },
    {
      type: 'optional-number',
      label: 'Max Size',
      name: 'max-size',
      value: +formValue['max-size'] ?? 1080,
      enable: formValue['max-size'] !== undefined
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
      value: formValue['lock-video-orientation'] ?? '0',
      enable: formValue['lock-video-orientation'] !== undefined
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
      value: formValue['always-on-top'] ?? false
    },
    {
      type: 'switch',
      label: 'Fullscreen',
      name: 'fullscreen',
      value: formValue.fullscreen ?? false
    },
    {
      type: 'switch',
      label: 'Window Borderless',
      name: 'window-borderless',
      value: formValue['window-borderless'] ?? false
    },
    {
      type: 'optional-text',
      label: 'Window Title',
      name: 'title',
      value: formValue.title ?? '',
      enable: formValue.title !== undefined
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
      value: formValue['window-x'] ?? 0,
      enable: formValue['window-x'] !== undefined
    },
    {
      type: 'optional-number',
      label: 'Position Y',
      name: 'window-y',
      value: formValue['window-y'] ?? 0,
      enable: formValue['window-y'] !== undefined
    },
    {
      type: 'optional-number',
      label: 'Width',
      name: 'width',
      value: formValue.width ?? 0,
      enable: formValue.width !== undefined
    }, {
      type: 'header',
      label: 'Advanced',
      name: 'advanced'
    }, {
      type: 'switch',
      label: 'Autosave Location & Size',
      name: 'spw-autosave-location-size',
      value: formValue['spw-autosave-location-size'] ?? false
    }
  ]

  form.forEach((item) => {
    if (item.type === 'option') {
      item.defaultValue = item.value
    }
  })

  return form
}

store.subscribe(value => {
  (async () => {
    console.log('config-form.ts: store.subscribe: value:', value)

    async function getFormItems (): Promise<FormItem[]> {
      let formItems: FormItem[] = []
      if (value.type === 'new-by-device') {
        formItems = await getForm(value.deviceId)
      } else if (value.type === 'edit' || value.type === 'copy') {
        const config = await getConfig({ id: value.deviceConfigId })
        formItems = await getForm(config?.deviceId ?? '', config ?? undefined)
      } else if (value.type === 'new') {
        formItems = await getForm()
      }

      return formItems
    }

    const formItems = await getFormItems()

    async function setGeneralDialogForm (formItems: FormItem[]): Promise<void> {
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

              let deviceConfig: DeviceConfigExt = {
                id: '',
                deviceId: entity.device,
                name: entity.name,

                createdAt: null,
                updatedAt: null,
                lastSeenAt: null
              }

              if (value.type === 'edit' || value.type === 'copy') {
                deviceConfig = {
                  ...deviceConfig,
                  ...await getConfig({ id: value.deviceConfigId }),

                  deviceId: entity.device,
                  name: entity.name
                }

                if (value.type === 'copy') {
                  deviceConfig.id = ''
                }
              }

              const config = await saveConfig(deviceConfig)

              // const configItems = await getConfigItems(
              //   deviceConfig.id
              // ) ?? []

              const configItems: DeviceConfigValueExt[] = []
              deviceConfigItems.forEach(([key, value]) => {
                if (value === undefined) return
                configItems.push({
                  id: '',
                  deviceConfigId: config.id,
                  key,
                  value
                })
              })

              configItems.forEach((configItem) => {
                configItem.value = JSON.stringify(configItem.value)
              })

              await saveConfigItems(config.id, configItems)

              value.callback?.()

              return true
            }
          }
        ],
        cancelCallback: (response, formItems) => {
          return true
        }
      })
    }

    await setGeneralDialogForm(formItems)
  })().then(() => {
    console.log('config-form.ts: store.subscribe: then')
  }).catch(error => {
    console.log('config-form.ts: store.subscribe: catch: error:', error)
  })
})

export default store
