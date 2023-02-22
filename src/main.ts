// import "./style.css";
import App from "./lib/Home.svelte";

const app = new App({
  target: document.getElementById("app") || document.body,
});

export default app;
