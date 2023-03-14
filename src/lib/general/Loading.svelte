<script lang="ts">
  import './Loading/Color.scss'
  import Dialog, { Content } from '@smui/dialog'
  import { generalLoading } from '../../store'
  import CircularProgress from '@smui/circular-progress'
  let open = false

  generalLoading.subscribe((value) => {
    open = value.show
  })
</script>

<Dialog
  bind:open
  scrimClickAction=""
  escapeKeyAction=""
  aria-labelledby="mandatory-title"
  aria-describedby="mandatory-content"
>
  <Content>
    <div
      style="    
      display: flex; 
      justify-content: center; 
      align-items: center;height: 100px;"
    >
      <CircularProgress
        class="my-four-colors"
        style="height: 64px; width: 64px;"
        indeterminate
        fourColor
      />
    </div>
  </Content>
</Dialog>
