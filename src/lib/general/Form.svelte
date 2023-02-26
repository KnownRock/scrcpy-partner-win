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

  export let open = false
  export let currentForm : FormItem[] = []

  type FormItem =
    | {
        type: 'option';
        label: string;
        name: string;
        options: { label: string; value: string }[];
        value: string;
        defaultValue?: string;
      }
    | {
        type: 'switch';
        label: string;
        name: string;
        value: boolean;
      }
    | {
        type: 'text';
        label: string;
        name: string;
        value: string;
      }
    | {
        type: 'number';
        label: string;
        name: string;
        value: number;
      }
    | {
        type: 'auto';
        label: string;
        name: string;
        value: string;
        options: string[];
      }
    | {
        type: 'optional-auto';
        label: string;
        name: string;
        value: string;
        enable: boolean;
        options: string[];
      }
    | {
        type: 'optional-text';
        label: string;
        name: string;
        value: string;
        enable: boolean;
      }
    | {
        type: 'optional-number';
        label: string;
        name: string;
        value: number;
        enable: boolean;
      }
    | {
        type: 'optional-option';
        label: string;
        name: string;
        options: { label: string; value: string }[];
        value: string;
        defaultValue?: string;
        enable: boolean;
      }
    | {
        type: 'header';
        label: string;
        name: string;
      };


  let response = ''

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
      {#each currentForm as formItem (formItem.name)}
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
    <Button action="cancel">
      <Label>Cancel</Label>
    </Button>
    <Button action="start" defaultAction on:click={() => start()}>
      <Label>Start</Label>
    </Button>
  </Actions>
</Dialog>
