<div class="record-container">
  <div class="record-panel">
    <div>
      {#each keyButtons as button}
        <IconButton 
          class="material-icons"
        on:click={() => {
          const cmd = new Command('adb', ['-s', adbId, 'shell', 'input', 'keyevent', button.adbKey.toString()])
          cmd.spawn()

          addOperation({
            type: 'keyevent',
            key: button.adbKey
          })
        }}>
          {button.icon}
        </IconButton>
      {/each}
    </div>
    <div>
      <div style="display: flex; flex-direction: column;">
        <Autocomplete
          combobox 
          options={applications}
          bind:value={currentApplication}
          label={$t('select application')}
        />
        <Textfield
          bind:value={currentApplicationAndActivity}
          label={$t('select app & activity')}
        />
        <Button on:click={() => {
          setApplicatons()
        }}>
          {$t('Fresh apps')}
        </Button>

        <Button on:click={async () => {
          const currentActivity = await getCurrentActivityName(adbId)
          const paName = `${currentActivity}`
          currentApplicationAndActivity = paName
        }}>
          {$t('Get current activity')}
        </Button>
        <Button
          on:click={async () => {
            // const activeName = await getDefaultActiveName(adbId, currentApplication)
            // const paName = `${currentApplication}/${activeName}`
            const cmd = new Command('adb', ['-s', adbId, 'shell', 'am', 'start', '-n', currentApplicationAndActivity])
            cmd.spawn()

            addOperation({
              type: 'am_start',
              packageName: currentApplicationAndActivity
            })
          }}
        >
          {$t('Start')}
        </Button>
      </div>
    </div>
    <div class="record-operations">
      {#each operations as operation}
        {#if operation.type === 'tap'}
          <div>{operation.type} {operation.x} {operation.y}</div>
        {:else if operation.type === 'keyevent'}
          <div>{operation.type} {operation.key}</div>
        {:else if operation.type === 'am_start'}
          <div>{operation.type} {operation.packageName}</div>

        {:else if operation.type === 'motion'}
          <div>{operation.type} {operation.motionType} {operation.x} {operation.y}</div>

        {:else}
          <div>Unknown operation</div>
        {/if}
      {/each}
    </div>
    
  </div>
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <div class="record-interface" 
    bind:clientHeight={height}
    bind:clientWidth={width}
    on:mousedown={handleMouseDown}
    on:mouseup={handleMouseUp}
    on:mousemove={throttleMouseMove}
    on:contextmenu={handleContextMenu}
    >
  </div>
</div>

<script lang="ts">
  import 'svelte-material-ui/bare.css'
  import { t } from 'svelte-i18n'
  import { getConfigId, getCurrentExeDir, getCurrentExePath, getDeviceSize } from '../utils/app'
  import { onMount } from 'svelte'
  import { getConfigById } from '../utils/configs'
  import prismaClientLike from '../utils/prisma-like-client'
  import { appWindow } from '@tauri-apps/api/window'
  import { Child, Command } from '@tauri-apps/api/shell'
  import Autocomplete from '@smui-extra/autocomplete'
  import Button from '@smui/button'
  import IconButton from '@smui/icon-button/src/IconButton.svelte'
  import Textfield from '@smui/textfield'
  let height = 0
  let width = 0
  let adbId = ''
  let deviceSize = [0, 0]

  let currentApplication = ''
  let currentApplicationAndActivity = ''

  let motionAdbShell:Child | null = null

  // const scrcpyAdbCommandArgs = ['CLASSPATH=/data/local/tmp/scrcpy-server.jar', 'app_process', '/', 'com.genymobile.scrcpy.Server', '1.25', 'tunnel_forward=true']
  let scrcpyCtrlShell:Child | null = null

  enum MotionType {
    Down = 0,
    Up = 1,
    Move = 2
  }

  type Operation = ({
    type: 'tap'
    x: number
    y: number
  } | {
    type: 'keyevent'
    key: number
  } | {
    type: 'am_start'
    packageName: string
  } | {
    type: 'motion'
    motionType: MotionType,
    x: number
    y: number
  })

  type OperationWithTime = Operation & {
    time: Date
  }

  let operations : OperationWithTime[] = []


  let mouseEventQueue: {
    type: MotionType
    x: number
    y: number
  }[] = []

  function handleContextMenu (e: MouseEvent) {
    e.preventDefault()
  }

  let motionAdbShellLock = false
  function addMouseEvent (type: MotionType, x: number, y: number) {
    mouseEventQueue.push({
      type,
      x,
      y
    })

    operations.push({
      type: 'motion',
      motionType: type,
      x,
      y,
      time: new Date()
    })

    operations = operations

    if (motionAdbShellLock) return

  
    motionAdbShellLock = true
    setTimeout(async () => {
      for (const event of mouseEventQueue) {
        if (scrcpyCtrlShell) {
          // await motionAdbShell.write(`input motionevent ${event.type} ${event.x} ${event.y}\n`)
          await scrcpyCtrlShell.write(`touch ${event.x} ${event.y} ${event.type}\n`)
        }
      }
      mouseEventQueue = []

      motionAdbShellLock = false
    }, 0)
  }
  

  function getXYFromEvent (e: MouseEvent) {
    const [deviceWidth, deviceHeight] = deviceSize
    const x = ~~(e.offsetX / width * deviceWidth)
    const y = ~~(e.offsetY / height * deviceHeight)
    return [x, y]
  }

  let isMouseDown = false
  async function handleMouseDown (e: MouseEvent) {
    if (e.button !== 0) return

    console.log('mousedown', e)
    isMouseDown = true
    const [x, y] = getXYFromEvent(e)
    addMouseEvent(MotionType.Down, x, y)
    // if (motionAdbShell) motionAdbShell.write(`input motionevent down ${x} ${y}\n`)
  }

  async function handleMouseUp (e: MouseEvent) {
    if (e.button !== 0) return

    console.log('mouseup', e)
    isMouseDown = false
    const [x, y] = getXYFromEvent(e)
    addMouseEvent(MotionType.Up, x, y)
    // if (motionAdbShell) motionAdbShell.write(`input motionevent up ${x} ${y}\n`)
  }

  const throttle = (fn: Function, delay: number) => {
    let lastCall = 0
    return function (...args: any[]) {
      const now = (new Date()).getTime()
      if (now - lastCall < delay) {
        return
      }
      lastCall = now
      return fn(...args)
    }
  }


  async function handleMouseMove (e: MouseEvent) {
    if (!isMouseDown) return
    console.log('mousemove', e)
    const [x, y] = getXYFromEvent(e)
    addMouseEvent(MotionType.Move, x, y)
    // if (motionAdbShell) motionAdbShell.write(`input motionevent move ${x} ${y}\n`)
  }

  const throttleMouseMove = throttle(handleMouseMove, 10)

  async function handleClick (e : MouseEvent) {
    console.log('click', e)

    const [x, y] = getXYFromEvent(e)

    // const cmd = new Command('adb', ['-s', adbId, 'shell', 'input', 'tap', x.toString(), y.toString()])
    // cmd.spawn()
    if (motionAdbShell) motionAdbShell.write(`input tap ${x} ${y}\n`)

    addOperation({
      type: 'tap',
      x,
      y
    })
    operations = operations
    // open('adb', ['shell', 'input', 'tap', (~~x).toString(), (~~y).toString()], 'd:\\')
  }


  $: currentApplication && (async () => {
    const activeName = await getDefaultActiveName(adbId, currentApplication)
    const paName = `${currentApplication}/${activeName}`
    currentApplicationAndActivity = paName
  })()

  let applications :string[] = []


  function addOperation (operation: Operation) {
    operations.push({
      ...operation,
      time: new Date()
    })
    operations = operations
  }

  // FIXME: notify when android version is not supported (<7)
  async function getDefaultActiveName (
    adbId: string,
    packageName: string
  ) {
    // adb shell cmd package resolve-activity [name]
    return new Promise((resolve, reject) => {
      const cmd = new Command('adb', ['-s', adbId, 'shell', 'cmd', 'package', 'resolve-activity', packageName])
      cmd.stdout.on('data', (data) => {
        const lines = data.split('\n')
        // FIXME: check if there is more than one line
        lines.some(line => {
          const trimLine = line.trim()
          if (trimLine.startsWith('name=')) {
            const activityName = trimLine.replace('name=', '')
            console.log('activityName', activityName)
            resolve(activityName)
            return true
          } else {
            return false
          }
        })
      })
  
      cmd.spawn()


      cmd.on('close', () => {
        reject(new Error('No activity found'))
      })
    })
  }

  async function getCurrentActivityName (
    adbId: string
  ) {
    // adb shell dumpsys window windows | grep -E 'mCurrentFocus|mFocusedApp'
    return new Promise((resolve, reject) => {
      const cmd = new Command('adb', ['-s', adbId, 'shell', 'dumpsys', 'window'])
      cmd.stdout.on('data', (data) => {
        const lines = data.split('\n')
        lines.forEach(line => {
          // console.log(line)
          const trimLine = line.trim()
          // mCurrentFocus
          if (trimLine.startsWith('mCurrentFocus')) {
            // FIXME: make more accurate
            const activityName = trimLine.match(/mCurrentFocus.+\{.+ u[0-9]+ (.+)\}/)?.[1]
            console.log('activityName', activityName)
            resolve(activityName)
          }
        })
      })

      cmd.spawn()

      // resolve('com.android.settings/.Settings')
    })
}

  const keyButtons = [{
    label: 'Home',
    command: 'home',
    icon: 'radio_button_unchecked',
    adbKey: 3
  }, {
    label: 'Back',
    command: 'back',
    icon: 'arrow_back',
    adbKey: 4
  }, {
    label: 'Menu',
    command: 'menu',
    icon: 'menu',
    adbKey: 82
  }, {
    label: 'Power',
    command: 'power',
    icon: 'power_settings_new',
    adbKey: 26
  }, {
    label: 'Volume Up',
    command: 'volumeUp',
    icon: 'volume_up',
    adbKey: 24
  }, {
    label: 'Volume Down',
    command: 'volumeDown',
    icon: 'volume_down',
    adbKey: 25
  }]

  async function setApplicatons () {
    const allPm = [] as string[]
    const pmList = new Command('adb', ['-s', adbId, 'shell', 'pm', 'list', 'packages'])
    pmList.stdout.on('data', (data) => {
      const lines = data.toString().split('\n')
      for (const line of lines) {
        if (line.startsWith('package:')) {
          // applications.push(line.slice(8).trim())
          allPm.push(line.slice(8).trim())
        }
      }
    })
    await pmList.spawn()
    applications = allPm
  }


  onMount(async () => {
    const configId = await getConfigId()
    const config = await getConfigById(configId)
    if (config == null) {
      appWindow.close()

      return
    }

    const device = await prismaClientLike.device.findUnique({
      where: {
        id: config.deviceId
      }
    })

    console.log('config', config)
    console.log('device', device)

    if (device == null) {
      appWindow.close()

      return
    }

    deviceSize = await getDeviceSize(device.adbId)
    console.log('deviceSize', deviceSize)

    adbId = device.adbId


    const adbShellCmd = new Command('adb', ['shell'])
    const adbShell = await adbShellCmd.spawn()
    motionAdbShell = adbShell
  
    // tauri get executable path
    // const executeDir = await tauri.promisified({ cmd: 'get_executable_path' })
    // console.log('executeDir', await executableDir())
    const currentExeDir = await getCurrentExeDir()
    console.log('currentExeDir', currentExeDir)


    const scrcpyJarPath = currentExeDir + '/scrcpy-server.jar'

    const pushScrcpyJarCmd = new Command('adb', ['-s', adbId, 'push', scrcpyJarPath, '/data/local/tmp/scrcpy-server.jar'])
    // pushScrcpyJarCmd.on('close', () => {
    //   console.log('pushScrcpyJarCmd close')
    // })
    pushScrcpyJarCmd.on('error', (err) => {
      console.log('pushScrcpyJarCmd error', err)
    })
    // pushScrcpyJarCmd.stdout.on('data', line => console.log(`command stdout: "${line}"`))
    await pushScrcpyJarCmd.execute()

    const ctrlCmd = new Command('scrcpy-control', [])
    ctrlCmd.on('close', () => {
      console.log('ctrlCmd close')
    })
    ctrlCmd.on('error', (err) => {
      console.log('ctrlCmd error', err)
    })
    ctrlCmd.stdout.on('data', line => console.log(`command stdout: "${line}"`))
    ctrlCmd.stderr.on('data', line => console.log(`command stderr: "${line}"`))


    scrcpyCtrlShell = await ctrlCmd.spawn()

    return () => {
      motionAdbShell?.kill()
      scrcpyCtrlShell?.kill()
    }
  })

</script>

<style>
  .record-container {
    height: 100%;
    margin: 0;
    background-color: transparent;
  }

  /* FIXME: temp */
  :global(#app, body, html){
    height: 100%;
    background-color: transparent;
  }
  
  .record-container{
    height: 100vh;
    width: 100vw;

    display: flex;
  }

  .record-panel{
    height: 100vh;
    display: flex;
    flex-direction: column;
    width: 400px;
    background-color: #ccc;
  }

  .record-operations{
    overflow: auto;
  }

  .record-interface{
    flex: 1;
    background-color: transparent;
  }
</style>

