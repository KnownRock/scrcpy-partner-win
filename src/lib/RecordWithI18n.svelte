<script>
  import { isLoading } from 'svelte-i18n'
  import Record from './Record.svelte'
</script>

{#if $isLoading}
  <div></div>
{:else}
  <Record />
{/if}
