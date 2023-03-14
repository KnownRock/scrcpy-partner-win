<script lang="ts">
  import Select, { Option } from '@smui/select'
  import Dialog, { Header, Title, Content } from '@smui/dialog'
  import IconButton from '@smui/icon-button'
  import Button, { Label } from '@smui/button'
  import Switch from '@smui/switch'
  import Autocomplete from '@smui-extra/autocomplete'
  import FormField from '@smui/form-field'
  import Textfield from '@smui/textfield'
  import LayoutGrid, { Cell } from '@smui/layout-grid'
  import DataTable, { Head, Body, Row, Cell as TableCell } from '@smui/data-table'
  import { generalDialogForm } from '../../store/index'
  import Tooltip, { Wrapper } from '@smui/tooltip'
  import Optional from './Form/Optional.svelte'
  let open = false
  let currentFormItems : FormItem[] = []
  let buttons: DialogFormButton[] = []
  let cancelCallback: {
    (response: string, formItems: FormItem[]): boolean | Promise<boolean>
  } = () => true

  let updateTime = 0
  let response = ''
  let title = 'Form'

  generalDialogForm.subscribe((value) => {
    if (value.show) {
      open = true
      currentFormItems = value.formItems
      buttons = value.buttons
      cancelCallback = value.cancelCallback
      updateTime++
      title = value.title ?? 'Form'
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
      } else if (item.type === 'optional-auto') {
        if (item.enable) {
          ent[item.name] = item.value
        }
      } else if (item.type === 'optional-switch') {
        if (item.enable) {
          ent[item.name] = item.value
        }
      } else if (item.type === 'optional-auto-number') {
        if (item.enable) {
          ent[item.name] = item.value
        }
      } else {
        if (item.type === 'header' || item.type === 'message' || item.type === 'table') {
          console.log('no handle form item type: ' + item.type)
        } else {
          // throw new Error('Unknown form item type: ' + item.type)
          throw new Error('Unknown form item: ' + item)
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
        {#if formItem.type === 'header' || formItem.type === 'message' || formItem.type === 'table'}
          {#if formItem.type === 'header'}
            <Cell span={12}>
              <h2>{formItem.label}</h2>
            </Cell>
          {:else if formItem.type === 'message'}
            <Cell span={12}>
              <p>{formItem.value}</p>
            </Cell>
          {:else if formItem.type === 'table'}
          <Cell span={12}>
          <DataTable table$aria-label="People list" style="width: 100%;">
            <Head>
              <Row style="width: 100%">
                <!-- <TableCell>Name</TableCell>
                <TableCell>Favorite Color</TableCell>
                <TableCell numeric>Favorite Number</TableCell> -->
                {#each formItem.columns as column (column.name)}
                  <TableCell style="width: 100%">{column.label}</TableCell>
                {/each}
              </Row>
            </Head>
            <Body>
              <!-- <Row>
                <TableCell>Steve</TableCell>
                <TableCell>Red</TableCell>
                <TableCell numeric>45</TableCell>
              </Row> -->
              {#each formItem.value as row}
                <Row>
                  {#each formItem.columns as column (column.name)}

                    {#if column.buttons}
                      <TableCell>
                        {#each column.buttons as button (button.label)}
                          <Button on:click={() => button.callback(row)} >
                            <Label>{button.label}</Label>
                          </Button>
                        {/each}
                      </TableCell>
                    {:else}
                      <TableCell>{row[column.name]}</TableCell>
                    {/if}

                  {/each}
                </Row>
              {/each}
            </Body>
          </DataTable>
        </Cell>
          {/if}
        {:else}
          <Cell>
            <Wrapper>
            {#if formItem.type === 'auto'}
              <Autocomplete
                combobox 
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
              <!-- <FormField disabled={formItem.disabled}>
                <Checkbox bind:checked={formItem.enable} />
                <div 
                  on:click={e => { e.stopPropagation() }}
                  on:mousedown={e => e.stopPropagation()}
                  on:mouseup={e => e.stopPropagation()}
                >
                  <Autocomplete
                    type="text"
                    slot="label"
                    disabled={!formItem.enable}
                    onclick={e => {
                      e.stopPropagation()
                    }}

                    options={formItem.options}
                    bind:value={formItem.value}
                    label={formItem.label}
                  />
                </div>
                
              </FormField> -->
              <Optional bind:disabled={formItem.disabled} bind:enable={formItem.enable} >
                <Autocomplete
                  combobox
                  type="text"
                  disabled={!formItem.enable || formItem.disabled}
                  options={formItem.options}
                  bind:value={formItem.value}
                  label={formItem.label}
                />
              </Optional>
            {/if}

            {#if formItem.type === 'optional-auto-number'}
              <Optional bind:disabled={formItem.disabled} bind:enable={formItem.enable} >
                <Autocomplete
                  combobox
                  type="number"
                  disabled={!formItem.enable || formItem.disabled}
                  options={formItem.options}
                  bind:value={formItem.value}
                  label={formItem.label}
                />
              </Optional>
            {/if}

            {#if formItem.type === 'optional-option'}
              <Optional bind:disabled={formItem.disabled} bind:enable={formItem.enable} >
                <!-- avoid update devices issue -->
                {#key updateTime}
                  <Select
                    disabled={!formItem.enable}
                    bind:value={formItem.value}
                    label={formItem.label}
                  >
                    {#each formItem.options as option}
                      <Option value={option.value}>{option.label}</Option>
                    {/each}
                  </Select>
                {/key}

              </Optional>
            {/if}

            {#if formItem.type === 'optional-text'}
              <Optional bind:disabled={formItem.disabled} bind:enable={formItem.enable} >
                <Textfield
                  disabled={!formItem.enable}
                  bind:value={formItem.value}
                  label={formItem.label}
                />
              </Optional>
            {/if}

            {#if formItem.type === 'optional-switch'}
              <Optional bind:disabled={formItem.disabled} bind:enable={formItem.enable} >
                <FormField
                  align="start"
                >
                  <Switch 
                    disabled={!formItem.enable} 
                    bind:checked={formItem.value} />
                  <span slot="label">{formItem.label}</span>
                </FormField>
              </Optional>
              
            {/if}

            {#if formItem.type === 'optional-number'}
              <Optional bind:disabled={formItem.disabled} bind:enable={formItem.enable} >
                <Textfield
                  disabled={!formItem.enable}
                  bind:value={formItem.value}
                  label={formItem.label}
                  type="number"
                />
              </Optional>
            {/if}
            {#if formItem.description}
              <Tooltip xPos="start" yPos="above">
                {formItem.description}
              </Tooltip>
            {/if}
            </Wrapper>

          </Cell>
        {/if}
      {/each}
    </LayoutGrid>
  </Content>
  <!-- <Actions> -->

    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div  on:click={e => {
        e.stopPropagation()
        e.preventDefault()
      }} style="display: flex; justify-content: flex-end; padding:  1rem 1rem 0.5rem 1rem;">

 
      {#each buttons as button}
        <Button 
        action={button.action} 
        defaultAction={!!button.defaultAction}
          on:click={() => buttonHandler(button)}>
          <Label>{button.label}</Label>
        </Button>
      {/each}
    </div>
  <!-- </Actions> -->
</Dialog>
