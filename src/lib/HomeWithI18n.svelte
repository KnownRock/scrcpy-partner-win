<script>
  import { isLoading } from 'svelte-i18n'
  import Home from './Home.svelte'
</script>

{#if $isLoading}
  <div></div>
{:else}
  <Home />
{/if}
