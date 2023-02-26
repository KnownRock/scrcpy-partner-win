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
  import { dialogForm } from '../../store/index'

  let open = false
  let currentFormItems : FormItem[] = []
  let buttons: DialogFormButton[] = []
  let cancelCallback: {
    (response: string, formItems: FormItem[]): boolean
  } = () => true

  let updateTime = 0
  let response = ''

  dialogForm.subscribe((value) => {
    if (value.show) {
      open = true
      currentFormItems = value.formItems
      buttons = value.buttons
      cancelCallback = value.cancelCallback
      updateTime++
    }
  })


  function closeHandler (e: CustomEvent<{ action: string }>) {
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
    <Title id="fullscreen-title">Config</Title>
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
                options={formItem.options}
                bind:value={formItem.value}
                label={formItem.label}
              />
            {/if}

            {#if formItem.type === 'switch'}
              <FormField align="start">
                <Switch bind:checked={formItem.value} />
                <span slot="label">{formItem.label}</span>
              </FormField>
            {/if}

            {#if formItem.type === 'text'}
              <Textfield bind:value={formItem.value} label={formItem.label} />
            {/if}

            {#if formItem.type === 'number'}
              <Textfield
                bind:value={formItem.value}
                label={formItem.label}
                type="number"
              />
            {/if}

            {#if formItem.type === 'option'}
              <!-- avoid update devices issue -->
              {#key updateTime}
                <Select 
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
              <FormField>
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
              <FormField>
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
              <FormField>
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
              <FormField>
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
      on:click={() => button.callback(
        currentFormItems
      )}>
        <Label>{button.label}</Label>
      </Button>
    {/each}
  </Actions>
</Dialog>
