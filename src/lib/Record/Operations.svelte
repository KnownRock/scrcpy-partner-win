<div class="record-operations">
  {#each operations as operation}
    {#if operation.type === 'tap'}
      <div>{operation.type} {operation.x} {operation.y}</div>
    {:else if operation.type === 'keyevent'}
      <div>{operation.type} {operation.key} {operation.keyEventType}</div>
    {:else if operation.type === 'am_start'}
      <div>{operation.type} {operation.packageName}</div>

    {:else if operation.type === 'motion'}
      <div>{operation.type} {operation.motionType} {operation.x} {operation.y}</div>

    {:else}
      <div>Unknown operation</div>
    {/if}
  {/each}
</div>

<script lang="ts">
  import type { RecordOperationWithTime } from '../../types'
  export let operations: RecordOperationWithTime[] = []

</script>