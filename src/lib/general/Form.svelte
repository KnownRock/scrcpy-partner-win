<script lang="ts">
  import Select, { Option } from '@smui/select'
  import Checkbox from '@smui/checkbox'
  import Dialog, { Header, Title, Content, Actions } from '@smui/dialog'
  import IconButton from '@smui/icon-button'
  import Button, { Label } from '@smui/button'
  import Switch from '@smui/switch'
  import Autocomplete from '@smui-extra/autocomplete'
  import FormField from '@smui/form-field'
  import Textfield from '@smui/textfield'
  import LayoutGrid, { Cell } from '@smui/layout-grid'
  
  import { generalDialogForm } from '../../store/index'

  let open = false
  let currentFormItems : FormItem[] = []
  let buttons: DialogFormButton[] = []
  let cancelCallback: {
    (response: string, formItems: FormItem[]): boolean | Promise<boolean>
  } = () => true

  let updateTime = 0
  let response = ''
  const title = 'Form'

  generalDialogForm.subscribe((value) => {
    if (value.show) {
      open = true
      currentFormItems = value.formItems
      buttons = value.buttons
      cancelCallback = value.cancelCallback
      updateTime++
    }
  })


  async function closeHandler (e: CustomEvent<{ action: string }>) {
    switch (e.detail.action) {
      case 'close':
        response = 'Closed without response.'
        break
      case 'reject':
        response = 'Rejected.'
        break
      case 'accept':
        response = 'Accepted.'
        break
    }

    // FIXME: fix will close dialog when click cancel button in dialog when return false
    const isClose = await cancelCallback(response, currentFormItems)
    if (isClose) {
      open = false
    }
  }

  async function buttonHandler (button: DialogFormButton) {
    const isClose = await button.callback(
      formToEntity(currentFormItems),
      currentFormItems
    )
    if (isClose) {
      open = false
    }
  }

  function formToEntity (form: FormItem[]) {
    const ent = {}
    for (const item of form) {
      if (item.type === 'option') {
        ent[item.name] = item.value
      } else if (item.type === 'switch') {
        ent[item.name] = item.value
      } else if (item.type === 'auto') {
        ent[item.name] = item.value
      } else if (item.type === 'text') {
        ent[item.name] = item.value
      } else if (item.type === 'number') {
        ent[item.name] = item.value
      } else if (item.type === 'optional-text') {
        if (item.enable) {
          ent[item.name] = item.value
        }
      } else if (item.type === 'optional-number') {
        if (item.enable) {
          ent[item.name] = item.value
        }
      } else if (item.type === 'optional-option') {
        if (item.enable) {
          ent[item.name] = item.value
        }
      }
    }

    return ent
  }

</script>

<Dialog
  bind:open
  fullscreen
  aria-labelledby="fullscreen-title"
  aria-describedby="fullscreen-content"
  on:SMUIDialog:closed={closeHandler}
>
  <Header>
    <Title id="fullscreen-title">{ title }</Title>
    <IconButton action="close" class="material-icons">close</IconButton>
  </Header>
  <Content id="fullscreen-content">
    <LayoutGrid>
      {#each currentFormItems as formItem (formItem.name)}
        {#if formItem.type === 'header'}
          <Cell span={12}>
            <h2>{formItem.label}</h2>
          </Cell>
        {:else}
          <Cell>
            {#if formItem.type === 'auto'}
              <Autocomplete
                disabled={formItem.disabled}
                options={formItem.options}
                bind:value={formItem.value}
                label={formItem.label}
              />
            {/if}

            {#if formItem.type === 'switch'}
              <FormField 
                disabled={formItem.disabled}
                align="start">
                <Switch bind:checked={formItem.value} />
                <span slot="label">{formItem.label}</span>
              </FormField>
            {/if}

            {#if formItem.type === 'text'}
              <Textfield 
                disabled={formItem.disabled}
                bind:value={formItem.value} label={formItem.label} />
            {/if}

            {#if formItem.type === 'number'}
              <Textfield
                disabled={formItem.disabled}
                bind:value={formItem.value}
                label={formItem.label}
                type="number"
              />
            {/if}

            {#if formItem.type === 'option'}
              <!-- avoid update devices issue -->
              {#key updateTime}
                <Select 
                  disabled={formItem.disabled}
                  bind:value={formItem.value} 
                  label={formItem.label}
                >
                  {#each formItem.options as option}
                    <Option value={option.value}>{option.label}</Option>
                  {/each}
                </Select>
              {/key}
            {/if}

            {#if formItem.type === 'optional-auto'}
              <FormField disabled={formItem.disabled}>
                <Checkbox bind:checked={formItem.enable} />
                <!-- avoid update devices issue -->
                <Autocomplete
                  slot="label"
                  disabled={!formItem.enable}
                  options={formItem.options}
                  bind:value={formItem.value}
                  label={formItem.label}
                />
              </FormField>
            {/if}

            {#if formItem.type === 'optional-option'}
              <FormField disabled={formItem.disabled}>
                <Checkbox bind:checked={formItem.enable} />
                <!-- avoid update devices issue -->
                {#key updateTime}
                  <Select
                    slot="label"
                    disabled={!formItem.enable}
                    bind:value={formItem.value}
                    label={formItem.label}
                  >
                    {#each formItem.options as option}
                      <Option value={option.value}>{option.label}</Option>
                    {/each}
                  </Select>
                {/key}
              </FormField>
            {/if}

            {#if formItem.type === 'optional-text'}
              <FormField disabled={formItem.disabled}>
                <Checkbox bind:checked={formItem.enable} />
                <Textfield
                  slot="label"
                  disabled={!formItem.enable}
                  bind:value={formItem.value}
                  label={formItem.label}
                />
              </FormField>
            {/if}

            {#if formItem.type === 'optional-number'}
              <FormField disabled={formItem.disabled}>
                <Checkbox bind:checked={formItem.enable} />
                <Textfield
                  slot="label"
                  disabled={!formItem.enable}
                  bind:value={formItem.value}
                  label={formItem.label}
                  type="number"
                />
              </FormField>
            {/if}
          </Cell>
        {/if}
      {/each}
    </LayoutGrid>
  </Content>
  <Actions>
    {#each buttons as button}
      <Button 
      action={button.action} 
      defaultAction={!!button.defaultAction}
        on:click={() => buttonHandler(button)}>
        <Label>{button.label}</Label>
      </Button>
    {/each}
  </Actions>
</Dialog>
