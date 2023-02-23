<div style="
display: flex; justify-content: center;
height: calc(100vh - 2px) ; align-items: center;
width: calc(100vw - 2px); flex-direction: column;
border: 1px solid #ccc;
">
  <h1>Scrcpy Partner</h1>
  <!-- <h1>Loading...</h1> -->
  <CircularProgress style="height: 32px; width: 32px;" indeterminate />
  
</div>

<script lang="ts">
  import 'svelte-material-ui/bare.css'
  import CircularProgress from '@smui/circular-progress'

  import { onMount } from 'svelte'
  import { init } from '../utils/app'

  onMount(async () => {
    setTimeout(() => {
      init()
    }, 2000)
})

</script>