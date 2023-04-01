{#key uuid}
<div class="record-operations">
  <VirtualList
    height={400}
    width="auto"
    itemCount={operations.length}
    itemSize={50}>
    <div slot="item" let:index let:style {style} class="row">
      {#if operations[index].type !== 'group'}
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <div 
          on:click={(e) => {
            e.stopPropagation()
            handleSelect(e, index)
          }}
          class={index >= selection[0] && index <= selection[1] ? 'operation-item-selected' : 'operation-item'}
          style="display: flex; align-items: center; justify-content: space-between; padding: 5px;">
            <Item operation={operations[index]} />
     

          <OperationFucPanel 
            type={'item'}
            bind:operation={operations[index]} 
            bind:operations={operations}
            bind:selection={selection}
            execute={execute}
            fresh={fresh}
          />
          
        </div>
      {/if}
    </div>
  </VirtualList>
</div>
{/key}

<script lang="ts">
  import type { RecordOperation } from '../../types'
  import OperationFucPanel from './OperationFucPanel.svelte'
  export let operations: RecordOperation[] = []
  export let execute: (operation: RecordOperation) => void
  import { v4 as uuidv4 } from 'uuid'
  let uuid = uuidv4()
  import VirtualList from 'svelte-tiny-virtual-list'
  import Item from './Operations/Item.svelte'
  
  export let selection: [number, number]

  function handleSelect (e: MouseEvent, index) {
    e.stopPropagation()

    if (e.shiftKey) {
      if (selection[0] === -1) {
        selection[0] = index
        selection[1] = index
      } else {
        selection[1] = index
      }
    } else {
      if (index === selection[0] && index === selection[1]) {
        selection[0] = -1
        selection[1] = -1
        return
      }

      selection[0] = index
      selection[1] = index
    }
  }

  function fresh () {
    uuid = uuidv4()
  }

</script>

<style>
  .operation-item-selected {
    background-color: #afafaf;

    user-select: none;
  }

  .operation-item {
    user-select: none;
  }

  .row {
    /* height: 100px; */
  }
  
</style>