import { writable } from 'svelte/store'

const store = writable<{
  show: boolean
  noDismiss?: boolean
  buttons: MsgButton[]
  msg: string
  type?: 'error' | 'success' | 'info' | 'warning'
  cancelCallback?: () => Promise<boolean> | boolean
}>({
      show: false,
      buttons: [],
      msg: ''
    })

export default store
