import { writable } from 'svelte/store'

const store = writable<{
  show: boolean
}>({
  show: false
})

export default store
