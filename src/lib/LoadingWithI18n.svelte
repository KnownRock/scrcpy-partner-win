<script>
  import { isLoading } from 'svelte-i18n'
  import Loading from './Loading.svelte'
</script>

{#if $isLoading}
  <div></div>
{:else}
  <Loading />
{/if}
