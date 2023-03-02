import { writable } from 'svelte/store'
import { generalDialogForm } from '.'
import { lanuchSelf } from '../utils/devices'
const store = writable({
  show: false,
  deviceAdbId: ''
})

function getForm (
  currentDeviceId: string
): FormItem[] {
  const form: FormItem[] = [
    {
      type: 'header',
      label: 'General',
      name: 'general'
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

    const formItems = getForm(value.deviceAdbId)

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
          label: 'Start',
          action: 'start',
          defaultAction: true,
          callback: async (entity, formItems) => {
            const args = formEntityToArgs(entity)
            await lanuchSelf(args)
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
