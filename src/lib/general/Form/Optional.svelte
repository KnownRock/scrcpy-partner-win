<script>
  import Checkbox from '@smui/checkbox'
  import FormField from '@smui/form-field'
  import NoPopDiv from '../NoPopDiv.svelte'

  export let disabled = false
  export let enable = false

</script>

<FormField disabled={disabled}>
  <Checkbox bind:checked={enable} />
  <NoPopDiv  slot="label">
    <slot />
  </NoPopDiv>
  
</FormField>