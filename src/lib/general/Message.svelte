<Snackbar 
  bind:this={snackbar}
  class={`type-${type}`}
  >
  <Label>{ msgLabel }</Label>
  <Actions>
    {#each buttons as button}
      {#if button.icon}
        <IconButton
          class="material-icons"
          on:click={() => button.callback()}
          title={button.label}
        >
          {button.icon}
        </IconButton>
      {:else}
        <Button
          on:click={() => button.callback()}
          title={button.label}
        >
          <Label>{button.label}</Label>
        </Button>
      {/if}

    {/each}

    {#if noDismiss}
      <IconButton class="material-icons" title="Dismiss">close</IconButton>
    {/if}
  </Actions>
</Snackbar>


<script lang="ts">
  import './Message/Colors.scss'
  import Snackbar, { Actions, Label } from '@smui/snackbar'
  import IconButton from '@smui/icon-button'
  import { generalMsg } from '../../store'
  import Button from '@smui/button'

  let snackbar: Snackbar

  let msgLabel = ''
  let type = 'info'
  let noDismiss = false
  let buttons: MsgButton[] = []
  
  generalMsg.subscribe((value) => {
    // debugger
    if (value.show) {
      if (snackbar) {
        msgLabel = value.msg
        type = value.type ?? 'info'
        noDismiss = value.noDismiss ?? false
        buttons = value.buttons ?? []
  

        if (typeof snackbar.open === 'function') {
          snackbar.open()
        }
      }
    }
  })

</script>
