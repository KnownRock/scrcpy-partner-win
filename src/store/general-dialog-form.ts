import { writable } from 'svelte/store'

const store = writable<{
  show: boolean
  formItems: FormItem[]
  buttons: DialogFormButton[]
  title?: string
  cancelCallback: (response: string, formItems: FormItem[])
  => Promise<boolean> | boolean
}>({
      show: false,
      formItems: [],
      buttons: [],
      cancelCallback: () => true
    })

export default store
