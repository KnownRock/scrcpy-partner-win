import "./style.css";
import Loading from "./lib/Loading.svelte";


const app = new Loading({
  target: document.getElementById("app") || document.body,
});

export default app;


// import { invoke } from "@tauri-apps/api";
// invoke("init_main_window");