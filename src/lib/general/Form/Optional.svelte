<script>
  import Checkbox from '@smui/checkbox'
  import FormField from '@smui/form-field'

  export let disabled = false
  export let enable = false

</script>

<FormField disabled={disabled}>
  <Checkbox bind:checked={enable} />
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <div slot="label"
    on:click={e => { e.stopPropagation() }}
    on:mousedown={e => e.stopPropagation()}
    on:mouseup={e => e.stopPropagation()}
  >
    <slot />
  </div>
  
</FormField>