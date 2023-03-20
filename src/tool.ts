// import "./style.css";
import './i18n/index'
import App from './lib/ToolWithI18n.svelte'

const app = new App({
  target: document.getElementById('app') as HTMLElement
})

export default app
