import { writable } from 'svelte/store'

const store = writable<{
  show: boolean
  formItems: FormItem[]
  buttons: DialogFormButton[]
  cancelCallback: (
  response: string,
  formItems: FormItem[]
  ) => boolean
}>({
      show: false,
      formItems: [],
      buttons: [],
      cancelCallback: () => true
    })

export default store
