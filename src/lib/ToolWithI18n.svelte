<script>
  import { isLoading } from 'svelte-i18n'
  import Tool from './Tool.svelte'
</script>

{#if $isLoading}
  <div></div>
{:else}
  <Tool />
{/if}
