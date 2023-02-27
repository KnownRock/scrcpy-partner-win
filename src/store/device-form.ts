import { writable } from 'svelte/store'
import { generalDialogForm } from '.'
import { getDevices, type Device, saveDevice } from '../utils/devices'
const store = writable({
  show: false,
  deviceAdbId: ''
})

function getForm (
  device: Device
): FormItem[] {
  const form: FormItem[] = [
    {
      type: 'text',
      label: 'ADB Id',
      name: 'adbId',
      value: device.adbId,
      disabled: true
    },
    {
      type: 'text',
      label: 'name',
      name: 'name',
      value: device.name
    },
    {
      type: 'text',
      label: 'model',
      name: 'model',
      value: device.model,
      disabled: true
    },
    {
      type: 'text',
      label: 'product',
      name: 'product',
      value: device.product,
      disabled: true
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

    const devices = await getDevices()
    const currentDevice = devices.find(d => d.adbId === value.deviceAdbId)

    if (currentDevice == null) {
      const formItems: FormItem[] = [{
        type: 'header',
        label: 'Error: Device not found',
        name: 'error'
      }]

      generalDialogForm.set({
        show: value.show,
        formItems,
        buttons: [
          {
            label: 'Cancel',
            action: 'cancel',
            callback: (entity, formItems) => {
              return true
            }
          }
        ],
        cancelCallback: (response, formItems) => {
          return true
        }
      })
      return
    }
    const formItems = getForm(currentDevice)

    generalDialogForm.set({
      title: 'Device',
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
            await saveDevice(entity as Device)

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
