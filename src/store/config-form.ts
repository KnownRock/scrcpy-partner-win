import { writable } from 'svelte/store'
import { dialogForm } from '.'
const store = writable({
  show: false,
  deviceId: ''
})

store.subscribe(value => {
  (async () => {
    console.log('config-form.ts: store.subscribe: value:', value)

    dialogForm.set({
      show: value.show,
      formItems: [],
      buttons: [
        {
          label: 'Save',
          action: 'submit',
          defaultAction: true,
          callback: formItems => {
            console.log('config-form.ts: store.subscribe: callback: formItems:', formItems)
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
