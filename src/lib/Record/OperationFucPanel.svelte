<!-- svelte-ignore a11y-click-events-have-key-events -->
<div class={`fuc-panel-${type}`}
  on:click={(e) => {
    e.stopPropagation()
    e.preventDefault()
  }}
>
  <IconButton 
    size="mini"
    on:click={up}
    class="material-icons"
    title="Up"
    disabled={operations.indexOf(operation) === 0}
  >
    arrow_upward
  </IconButton>

  <IconButton 
    size="mini"
    on:click={down}
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
  export let type: 'item' | 'group'
  export let selection: [number, number]
  // FIXME: This is a hack to force refresh
  export let fresh : ()=> void

  function up () {
    const index = operations.indexOf(operation)

    if (index >= selection[0] && index <= selection[1]) {
      if (selection[0] - 1 < 0) return
      const temp = operations[selection[0] - 1]
      operations[selection[0] - 1] = operations[selection[0]]
      operations[selection[0]] = temp

      selection = [selection[0] - 1, selection[1] - 1]
    } else {
      if (index > 0) {
        const temp = operations[index - 1]
        operations[index - 1] = operation
        operations[index] = temp
      }
    }
}

  function down () {
    const index = operations.indexOf(operation)

    if (index >= selection[0] && index <= selection[1]) {
      if (selection[1] + 1 >= operations.length) return
      const temp = operations[selection[1] + 1]
      operations[selection[1] + 1] = operations[selection[1]]
      operations[selection[1]] = temp

      selection = [selection[0] + 1, selection[1] + 1]
    } else {
      if (index < operations.length - 1) {
        const temp = operations[index + 1]
        operations[index + 1] = operation
        operations[index] = temp
      }
    }
  }

  function removeOperation (operations: RecordOperation[], operation: RecordOperation) {
    if (type === 'item') {
      const index = operations.indexOf(operation)
      if (index >= selection[0] && index <= selection[1]) {
        operations.splice(selection[0], selection[1] - selection[0] + 1)

        selection = [-1, -1]
      } else {
        operations.splice(operations.indexOf(operation), 1)
      }
    } else {
      operations.splice(operations.indexOf(operation), 1)
    }

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