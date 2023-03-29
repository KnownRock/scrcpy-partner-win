<script>
  import Tab, { Label } from '@smui/tab'
  import TabBar from '@smui/tab-bar'
  import { t } from 'svelte-i18n'
  const opTabs = [
    {
      label: $t('Key Events'),
      id: 'keyEvents'
    },
    {
      label: $t('Apps'),
      id: 'apps'
    },
    {
      label: $t('Functions'),
      id: 'functions'
    },
    {
      label: $t('Scripts'),
      id: 'scripts'
    },
    {
      label: $t('Recordes'),
      id: 'operations'
    }
  ]

  let opActive = opTabs[0]

  export let active = opActive.id

  $: active = opActive.id

</script>
<TabBar tabs={opTabs} let:tab bind:active={opActive}>
  <!-- Note: the `tab` property is required! -->
  <Tab {tab}>
    <Label>{tab.label}</Label>
  </Tab>
</TabBar>