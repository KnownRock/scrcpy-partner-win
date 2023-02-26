import { writable } from 'svelte/store'

const store = writable({
  show: false,
  deviceId: ''
})

export default store
