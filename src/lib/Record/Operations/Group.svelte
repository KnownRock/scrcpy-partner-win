{#if operation.type === 'group'}
  <Paper variant="outlined" style="padding-top:10px;padding-bottom:10px">
  <div style="margin-top:16px;margin-bottom:-16px">
    <OperationFucPanel 
      bind:operation={operation} 
      bind:operations={operations}
      execute={execute}
      fresh={fresh}
      type={'group'}
      bind:selection={selection}
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
{/if}
<script lang="ts">
  import type { RecordOperation } from '../../../types'
  // import { t } from 'svelte-i18n'
  import Textfield from '@smui/textfield'
  import OperationFucPanel from '../OperationFucPanel.svelte'
  import Paper, { Title, Content } from '@smui/paper'
  export let operation: RecordOperation
  export let operations: RecordOperation[]
  export let execute: (operation: RecordOperation) => void
  import { v4 as uuidv4 } from 'uuid'
  let uuid = uuidv4()
  export let selection: [number, number]
  function fresh () {
    uuid = uuidv4()
  }

</script>

  