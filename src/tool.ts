// import "./style.css";
import './i18n/index'
import App from './lib/ToolWithI18n.svelte'
const App2 = App as unknown as new (options: { target: HTMLElement }) => App

const app = new App2({
  target: document.getElementById('app') as HTMLElement
})

export default app
