<Snackbar 
  bind:this={snackbar}
  class={`type-${type}`}
  >
  <Label>{ $t(msgLabel) }</Label>
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
          <Label>
            {$t(button.label)}
          </Label>
        </Button>
      {/if}

    {/each}

    {#if noDismiss}
      <IconButton class="material-icons" title="Dismiss">close</IconButton>
    {/if}
  </Actions>
</Snackbar>


<script lang="ts">
  import { t } from 'svelte-i18n'
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
