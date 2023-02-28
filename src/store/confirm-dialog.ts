import { writable } from 'svelte/store'
import { generalDialogForm } from '.'

const store = writable<{
  title: string
  message: string
  show: boolean
  okCallback: () => void
  cancelCallback?: () => void
}>({
      title: '',
      message: '',
      show: false,
      okCallback: () => {},
      cancelCallback: () => {}
    })

store.subscribe(value => {
  generalDialogForm.set({
    title: value.title,
    show: value.show,
    formItems: [
      {
        type: 'message',
        label: 'Message',
        name: 'message',
        value: value.message
      }
    ],
    buttons: [
      {
        label: 'Cancel',
        action: 'cancel',
        callback: (entity, formItems) => {
          value?.cancelCallback?.()
          return true
        }
      },
      {
        label: 'OK',
        action: 'ok',
        defaultAction: true,
        callback: (entity, formItems) => {
          value.okCallback()
          return true
        }
      }
    ],
    cancelCallback: (response, formItems) => {
      value?.cancelCallback?.()
      return true
    }
  })
})

export default store
