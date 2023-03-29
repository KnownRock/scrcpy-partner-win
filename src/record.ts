import './style.css'
import './i18n/index'
import Record from './lib/RecordWithI18n.svelte'

const app = new Record({
  target: document.getElementById('app') as HTMLElement
})

export default app
