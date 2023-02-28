import { writable } from 'svelte/store'
import { generalDialogForm } from '.'
import { type Device, saveDevice } from '../utils/devices'
const store = writable<{
  show: boolean
  device: Device
  callback: (device: Device) => void
}>({
      show: false,
      device: {
        id: '',
        adbId: '',
        name: '',
        model: '',
        product: '',
        createdAt: null,
        updatedAt: null,
        lastSeenAt: null
      },
      callback (device: Device) {}
    })

function getForm (
  device: Device
): FormItem[] {
  const form: FormItem[] = [
    {
      type: 'text',
      label: 'ADB Id',
      name: 'adbId',
      value: device.adbId
      // disabled: true
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
      value: device.model
    },
    {
      type: 'text',
      label: 'product',
      name: 'product',
      value: device.product
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
    const currentDevice = value.device

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
            const modifiedDevice = {
              ...currentDevice,
              ...entity
            }
            await saveDevice(modifiedDevice)

            value.callback(modifiedDevice)

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
