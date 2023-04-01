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
  {:else if operation.type === 'exec_script'}
    <span>[{translateOperationType(operation.type)}] Name:{operation.name}</span>
  
  {:else if operation.type === 'adb_cmd'}
    <span>[{translateOperationType(operation.type)}] Command:{operation.cmd}</span>

  {:else}
    <span>Unknown operation</span>
  {/if}
</div>

<script lang="ts">
  import { KeyEventType, MotionType, type RecordOperation } from '../../../types'


  export let operation: RecordOperation

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
      case 'exec_script':
        return 'Execute Script'
      case 'adb_cmd':
        return 'ADB Command'
      default:
        return 'Unknown'
    }
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

</script>