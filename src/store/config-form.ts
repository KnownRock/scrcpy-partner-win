
import { writable } from 'svelte/store'
import { generalDialogForm } from '.'
import { getDevices } from '../utils/devices'
import { saveConfig, type DeviceConfigValueExt, type DeviceConfigExt, getConfig, saveConfigItems, getConfigItems } from '../utils/configs'
import { getFormItems } from '../utils/scrcpy'
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
  const form = getFormItems(formValue)

  const deviceFormItems: FormItem[] = [{
    type: 'header',
    label: 'Main',
    name: 'main'
  }, {
    type: 'text',
    label: 'Name',
    name: 'name',
    value: currentDeviceConfig?.name ?? 'New Config'
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
  }]

  const advancedFormItems: FormItem[] = [{
    type: 'header',
    label: 'Advanced',
    name: 'advanced'
  },
  {
    type: 'optional-switch',
    label: 'Autosave Location & Size',
    name: '--spw-autosave-location-size',
    value: formValue['--spw-autosave-location-size'] ?? false,
    enable: formValue['--spw-autosave-location-size'] !== undefined
  }]

  return [...deviceFormItems, ...form, ...advancedFormItems]
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
