import './style.css'
import './i18n/index'
import Loading from './lib/LoadingWithI18n.svelte'

const app = new Loading({
  target: document.getElementById('app') as HTMLElement
})

export default app

// import { invoke } from "@tauri-apps/api";
// invoke("init_main_window");
