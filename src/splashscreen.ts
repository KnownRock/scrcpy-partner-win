import './style.css'
import './i18n/index'
import Loading from './lib/LoadingWithI18n.svelte'
const Loading2 = Loading as unknown as new (options: { target: HTMLElement }) => Loading

const app = new Loading2({
  target: document.getElementById('app') as HTMLElement
})

export default app

// import { invoke } from "@tauri-apps/api";
// invoke("init_main_window");
