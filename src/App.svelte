<script lang="ts">
  import { getExecMode } from './utils/app'

  import './style.css'
  
  import Home from './lib/Home.svelte'
  import Tool from './lib/Tool.svelte'

</script>

<main class="container">
  <!-- <Home /> -->

  {#await getExecMode()}
    <p>loading</p>
  {:then execMode}
    {#if execMode === 'tool'}
      <Tool />
    {/if}
    {#if execMode === 'home'}
      <Home />
    {/if}
  {/await}

 
    

</main>

<style>

</style>                                                                        