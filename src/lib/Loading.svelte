<div style="
display: flex; justify-content: center;
height: 100vh; align-items: center;
width: 100vw; flex-direction: column;
">
  <h1>Scrcpy Partner</h1>
  <!-- <h1>Loading...</h1> -->
  <CircularProgress style="height: 32px; width: 32px;" indeterminate />
  
</div>

<script lang="ts">
  import 'svelte-material-ui/bare.css'
  import CircularProgress from '@smui/circular-progress'
  import { invoke } from '@tauri-apps/api'

  import { onMount } from 'svelte'

  // const loading = true

  onMount(async () => {
    setTimeout(() => {
      invoke('init_main_window')
    }, 3000)
})

</script>