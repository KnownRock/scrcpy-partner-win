{#key uuid}
<div class="record-operations">
  {#each operations as operation, index}
      {#if operation.type === 'group'}
        <!-- <div style="border: 1px solid black; padding: 5px;"> -->
          <Paper variant="outlined" style="padding-top:10px;padding-bottom:10px">
            <div style="margin-top:16px;margin-bottom:-16px">
              <OperationFucPanel 
                operation={operation} 
                operations={operations}
                execute={execute}
                fresh={fresh}
                type={'group'}
              />
            </div>
            

            <Title>
              <Textfield
                variant="outlined" label="Group Name"
               bind:value={operation.name} />
              <!-- 123 -->
            </Title>
            <Content>
              <div style="
              border: 1px solid black; 
              padding: 5px;
              border-radius: 4px;
              ">
                <svelte:self operations={operation.operations} execute={execute} />
              </div>
              
            </Content>
          </Paper>
      {:else}
        <div style="display: flex; align-items: center; justify-content: space-between; padding: 5px;">
          <div  class="mdc-typography--headline6">
            {#if operation.type === 'tap'}
              <span>[{translateOperationType(operation.type)}] X:{operation.x} X:{operation.y}</span>
            {:else if operation.type === 'keyevent'}
              <span>[{translateOperationType(operation.type)}] Code:{operation.key} Type:{translateKeyEventType(operation.keyEventType)}</span>
            {:else if operation.type === 'am_start'}
              <span>[{translateOperationType(operation.type)}] Name:{operation.packageName}</span>
            {:else if operation.type === 'motion'}
              <span>[{translateOperationType(operation.type)}] Type:{translateMotionType(operation.motionType)} x:{operation.x} y:{operation.y}</span>
              {:else if operation.type === 'delay'}
                <span>[{translateOperationType(operation.type)}] 
                <input type="number" bind:value={operation.ms} style="width: 50px; text-align: center;" /> ms
              </span>
              {:else}
              <span>Unknown operation</span>
            {/if}
          </div>
     

          <OperationFucPanel 
            type={'item'}
            operation={operation} 
            operations={operations}
            execute={execute}
            fresh={fresh}
          />
          
        </div>
      {/if}
  
  {/each}
</div>
{/key}

<script lang="ts">
  import { KeyEventType, MotionType, type RecordOperation } from '../../types'
  // import { t } from 'svelte-i18n'
  import Textfield from '@smui/textfield'
  import OperationFucPanel from './OperationFucPanel.svelte'
  import Paper, { Title, Content } from '@smui/paper'
  export let operations: RecordOperation[] = []
  export let execute: (operation: RecordOperation) => void
  import { v4 as uuidv4 } from 'uuid'
  let uuid = uuidv4()


  function fresh () {
    uuid = uuidv4()
  }

  const translateMotionType = (motionType:MotionType) => {
    switch (motionType) {
      case MotionType.Down:
        return 'Down'
      case MotionType.Up:
        return 'Up'
      case MotionType.Move:
        return 'Move'
    }
  }

  const translateKeyEventType = (keyEventType:KeyEventType) => {
    switch (keyEventType) {
      case KeyEventType.Down:
        return 'Down'
      case KeyEventType.Up:
        return 'Up'
    }
  }

  const translateOperationType = (type: string) => {
    switch (type) {
      case 'tap':
        return 'Tap'
      case 'keyevent':
        return 'Key Event'
      case 'am_start':
        return 'Start App'
      case 'motion':
        return 'Motion'
      case 'delay':
        return 'Delay'
      default:
        return 'Unknown'
    }
  }

</script>

<style>
  
</style>