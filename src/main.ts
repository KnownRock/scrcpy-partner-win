import './i18n/index'
import App from './lib/HomeWithI18n.svelte'

const app = new App({
  target: document.getElementById('app') as HTMLElement
})

export default app
