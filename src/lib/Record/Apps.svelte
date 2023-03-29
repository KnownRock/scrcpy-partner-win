<div style="display: flex; flex-direction: column; padding:10px;">
    <Autocomplete
    options={applications}
    bind:value={currentApplication}
    label={$t('select application')}
  >
    <Textfield style="width:100%" label={$t('select application')} bind:value={currentApplication} />
  </Autocomplete>
 
  <Textfield
    bind:value={currentApplicationAndActivity}
    label={$t('app/activity')}
  />
  <div style="display: flex; flex-direction: row;">
    <Button on:click={() => {
      setApplicatons()
    }}>
      {$t('Fresh apps')}
    </Button>
  
    <Button on:click={setCurrentActivityName }>
      {$t('Get current activity')}
    </Button>
    <Button
      on:click={async () => {
        execute({
          type: 'am_start',
          packageName: currentApplicationAndActivity
        })
      }}
    >
      {$t('Start')}
    </Button>
  </div>
  
</div>

<script lang="ts">
  import Autocomplete from '@smui-extra/autocomplete'
  import Button from '@smui/button'
  import Textfield from '@smui/textfield'
  import type { RecordOperation } from '../../types'
  import { t } from 'svelte-i18n'
  import { execAdbCommand } from './AdbCmd'
  import { onMount } from 'svelte'

  let currentApplication = ''
  let currentApplicationAndActivity = ''
  let applications :string[] = []

  async function getDefaultActiveName (
    adbId: string,
    packageName: string
  ) {
    const text = await execAdbCommand(adbId, ['shell', 'cmd', 'package', 'resolve-activity', packageName])
    const lines = text.split('\n')
    let activityName = ''
    // FIXME: make it more robust
    lines.some(line => {
      const trimLine = line.trim()
      if (trimLine.startsWith('name=')) {
        activityName = trimLine.replace('name=', '')
        return true
      } else {
        return false
      }
    })
    return activityName
}

  $: currentApplication && (async () => {
    if (adbId) {
      const activeName = await getDefaultActiveName(adbId, currentApplication)
      const paName = `${currentApplication}/${activeName}`
      currentApplicationAndActivity = paName
    }
  })()


  export let execute: (operation: RecordOperation) => void
  export let adbId: string|undefined

  async function setCurrentActivityName (
  ) {
    if (adbId) {
      const text = await execAdbCommand(adbId, ['shell', 'dumpsys', 'window'])
      const lines = text.split('\n')
      let activityName: string | undefined
      lines.some(line => {
        const trimLine = line.trim()
        console.log('trimLine', trimLine)
        if (trimLine.startsWith('mCurrentFocus')) {
          activityName = trimLine.match(/mCurrentFocus.+\{.+ u[0-9]+ (.+)\}/)?.[1]
          console.log('activityName', activityName)
          return true
        } else {
          return false
        }
      })
      currentApplicationAndActivity = activityName || ''
    }
  }

  async function setApplicatons () {
    if (adbId) {
      const allPm = [] as string[]
      const text = await execAdbCommand(adbId, ['shell', 'pm', 'list', 'packages'])
      const lines = text.split('\n')
      for (const line of lines) {
        if (line.startsWith('package:')) {
          allPm.push(line.slice(8).trim())
        }
      }
      applications = allPm
    }
  }

  onMount(() => {
    setApplicatons()
  })

</script>
