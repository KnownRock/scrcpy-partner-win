<div class={`fuc-panel-${type}`}>
  <IconButton 
    size="mini"
    on:click={() => {
      const index = operations.indexOf(operation)
      if (index > 0) {
        const temp = operations[index - 1]
        operations[index - 1] = operation
        operations[index] = temp
        fresh()
      }
    }}
    class="material-icons"
    title="Up"
    disabled={operations.indexOf(operation) === 0}
  >
    arrow_upward
  </IconButton>

  <IconButton 
    size="mini"
    on:click={() => {
      const index = operations.indexOf(operation)
      if (index < operations.length - 1) {
        const temp = operations[index + 1]
        operations[index + 1] = operation
        operations[index] = temp
        fresh()
      }
    }}
    disabled={operations.indexOf(operation) === operations.length - 1}
    class="material-icons"
    title="Down"
  >
    arrow_downward
  </IconButton>

  <IconButton 
    size="mini"
    on:click={() => {
      removeOperation(operations, operation)
    }} 
    class="material-icons" 
    title="Delete"
  >
    delete
  </IconButton>

  <IconButton 
    size="mini"
    on:click={() => execute(operation)} 
    class="material-icons" 
    title="Play"
  >
    play_arrow
  </IconButton>
</div>

<script lang="ts">
  import IconButton from '@smui/icon-button'
  // import { t } from 'svelte-i18n'
  import type { RecordOperation } from '../../types'


  export let operations: RecordOperation[] = []
  export let execute: (operation: RecordOperation) => void
  export let operation: RecordOperation
  export let fresh: () => void
  export let type: 'item' | 'group'

  function removeOperation (operations: RecordOperation[], operation: RecordOperation) {
    operations.splice(operations.indexOf(operation), 1)
    // operations = [...operations]
    fresh()
  }

</script>

<style>
  .fuc-panel-group {
    display: flex;
    justify-content: flex-end;
    align-items: center;

    position: relative;
    /* top: -10px; */
    height: 0;
  }

  .fuc-panel-item {
    display: flex;
    justify-content: flex-end;
    align-items: center;
  }
</style>