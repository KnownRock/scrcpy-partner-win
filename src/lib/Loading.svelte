<div style="
  display: flex; justify-content: center;
  height: calc(100vh - 2px) ; align-items: center;
  width: calc(100vw - 2px); flex-direction: column;
  border: 1px solid #ccc;
  ">
  <h1>Scrcpy Partner</h1>
  {#if !error}
  <CircularProgress style="height: 32px; width: 32px;" indeterminate />
  {/if}

  {#if error}
    <h3>{error}</h3>
    <Button on:click={exit}>
      {$t('Close')}
    </Button>
  {/if}
  
</div>

<script lang="ts">
  import { t } from 'svelte-i18n'
  import 'svelte-material-ui/bare.css'
  import CircularProgress from '@smui/circular-progress'
  import arg, { type Result } from 'arg'
  import { onMount } from 'svelte'
  import { init, getEnvArgs, exit, getProcessHwnd } from '../utils/app'
  import {
    connectTcpipDevice, getDevices,
    updateDeviceLastSeenAt
    , type DeviceExt
  } from '../utils/devices'
  import { type Child, Command } from '@tauri-apps/api/shell'
  import Button from '@smui/button'
  import { getConfigById, updateConfigLastSeenAt } from '../utils/configs'
  import { argsTemplate } from '../utils/scrcpy'
  import { appWindow } from '@tauri-apps/api/window'

  async function sleep (ms) {
    return new Promise(resolve => setTimeout(resolve, ms))
  }

  async function runScrcpyCommand (args) : Promise<{
    hwnd?: number,
    pid?: number,
    error?: string
  } > {
    const cmd = import.meta.env.MODE === 'development'
      ? 'scrcpy-dev'
      : 'scrcpy'
  
    const env : {
      [key:string]: string
    } = !(import.meta.env.MODE === 'development')
      ? {
          SCRCPY_SERVER_PATH: './scrcpy-server'
        }
      : {}

    const scrcpyCommand = new Command(cmd, args, { env })

    let error = ''
    scrcpyCommand.stdout.on('data', (data) => {
      console.log('stdout', data)
    })
    scrcpyCommand.stderr.on('data', (data) => {
      console.log('stderr', data)
      error += data
    })
    let closed = false
    scrcpyCommand.on('close', (code) => {
      console.log('close', code)
      closed = true
    })

    const child: Child = await scrcpyCommand.spawn()

    let hwnd = 0

    // eslint-disable-next-line
    return new Promise(async (resolve) => {
      while (true) {
        await sleep(100)

        if (closed) {
          if (error) {
            console.log('error', error)
            resolve({ error })
          } else {
            resolve({ error: 'closed' })
          }
          break
        }

        if (hwnd === 0) {
          hwnd = await getProcessHwnd(child.pid)
          console.log('hwnd', hwnd)
        } else {
          console.log('hwnd', hwnd)
          console.log('pid', child.pid)

          if (hwnd !== 0) {
            resolve({ hwnd, pid: child.pid })
            break
          }
        }
      }
    })
  }

  let error = ''

  function getSerialDevicesFromArg (args) {
    return args['--serial']
  }

  function getTcpipDeviceFromArgs (args) {
    return args['--tcpip']
  }

  function getConfigIdFromArgs (args) {
    return args['--spw-config-id']
  }

  function getScrcpyArgsFromArgs (args) {
    const isWindowBorderless = args['--window-borderless'] === true


    return Object.keys(args).reduce((acc, cur) => {
      if (cur.startsWith('--spw-')) {
        return acc
      }

      if (cur.startsWith('--tcpip')) {
        return acc
      }

      if (cur === '_') {
        if (args._.length > 0) {
          console.warn('additional args', args._)
        }

        return acc
      }

      if (args[cur] === true) {
        return [...acc, cur]
      } else if (args[cur] === false) {
        return acc
      } else {
        // windows board magic number
        if (!isWindowBorderless) {
          if (cur === '--window-y') {
            // FIXME: windows borader size magic number
            return [...acc, cur, (args[cur] + 31) + '']
          } else if (cur === '--window-height') {
            return [...acc, cur, (args[cur] - 31) + '']
          } else if (cur === '--window-width') {
            return [...acc, cur, (args[cur] - 2 * 0) + '']
          } else if (cur === '--window-x') {
            return [...acc, cur, (args[cur] + 1 * 0) + '']
          }
        }

        return [...acc, cur, args[cur] + '']
      }
    }, [])
  }


  async function getAdbDeviceDict () {
    const adbDevices = await getDevices('only adb')
    const deviceDict = adbDevices.reduce<{
      [adbId:string]: DeviceExt
    }>((acc, cur) => {
      return {
        ...acc,
        [cur.adbId]: cur
      }
    }, {})

    return deviceDict
  }

  function getArgsFromRawArgs (rawArgs) {
    let args : Result<typeof argsTemplate> | null = null
    try {
      args = arg(argsTemplate, {
        argv: rawArgs,
        permissive: true
      })
    } catch (e) {
      console.error(e)
      return null
    }

    return args
  }


  onMount(() => {
    appWindow.setTitle($t('Loading...'))


    // TODO: move logic to utils
    setTimeout(async () => {
      const rawArgs = (await getEnvArgs()).slice(1)
      const deviceDict = await getAdbDeviceDict()

      const args = getArgsFromRawArgs(rawArgs)

      if (args === null) {
        error = $t('Invalid arguments')
        return
      }

      const serialDeviceInArgs = getSerialDevicesFromArg(args)
      console.log('serialDeviceInArgs', serialDeviceInArgs)


      const tcpipDeviceInArgs = getTcpipDeviceFromArgs(args)
      console.log('tcpipDeviceInArgs', tcpipDeviceInArgs)


      if (serialDeviceInArgs && tcpipDeviceInArgs) {
        error = $t('Only one device can be selected at a time')
      }


      if (serialDeviceInArgs) {
        const device = serialDeviceInArgs
        if (deviceDict[device]) {
          const {
            hwnd,
            pid,
            error: err
          } = await runScrcpyCommand(getScrcpyArgsFromArgs(args))

          if (err) {
            error = err
            return
          }
          if (!pid || !hwnd) {
            error = $t('Failed to start scrcpy')
            return
          }

          init(true, pid, hwnd)
          // appWindow.close() // because close cause scrcpy process exit
          appWindow.hide()
        } else {
          error = `${$t('Device')} ${device} ${$t('not connected')}`
        }

        return
      }


      if (tcpipDeviceInArgs) {
        const device = tcpipDeviceInArgs
        await connectTcpipDevice(device)
        const newDeviceDict = await getAdbDeviceDict()
  
        if (newDeviceDict[device]) {
          const {
            hwnd,
            pid,
            error: err
          } = await runScrcpyCommand(getScrcpyArgsFromArgs(args).concat(['--serial', device]))
  
          if (err) {
            error = err
            return
          }
          if (!pid || !hwnd) {
            error = $t('Failed to start scrcpy')
            return
          }
  
          init(true, pid, hwnd)
          // appWindow.close() // because close cause scrcpy process exit
          appWindow.hide()
          // change tcpip arg to serial arg
        } else {
          error = `${$t('Device')} ${device} ${$t('not connected')}`
        }

        return
      }

      const configId:string = getConfigIdFromArgs(args)
      if (configId) {
        console.log('configId', configId)

        const config = await getConfigById(configId)

        console.log('config', config)

        if (!config) {
          error = `${$t('Config')} ${configId} ${$t('not found')}`
          return
        }

        const rawArgs = config.deviceConfigValue.map(el => {
          const value = JSON.parse(el.value)
          if (el.key.startsWith('--spw-')) {
            return null
          }

          if (value === true) {
            return `${el.key}`
          } else if (value === false) {
            return null
          } else {
            return `${el.key}=${value}`
          }
        })
          .filter(el => el !== null) as string[]
  
        const devices = await getDevices()
        const device = devices.find(el => el.id === config.deviceId)

        if (!device) {
          error = `${$t('Device')} ${config.deviceId} ${$t('not found')}`
          return
        }

        if (device.isTcpipDevice && !device.isConnected) {
          await connectTcpipDevice(device.adbId)
          await sleep(1000)
        }

        rawArgs.unshift('--serial=' + device.adbId)
        console.log('rawArgs', rawArgs)

        const isAutoSaveLocationAndSize =
          config.deviceConfigValue
            .find(el => el.key === '--spw-autosave-location-size')
            ?.value === 'true'

        const args = getArgsFromRawArgs(rawArgs)
        const isWindowBorderless = args?.['--window-borderless'] === true
  
        const {
          hwnd,
          pid,
          error: err
        } = await runScrcpyCommand(getScrcpyArgsFromArgs(getArgsFromRawArgs(rawArgs)))

        if (err) {
          error = err
          return
        }
        if (!pid || !hwnd) {
          error = $t('Failed to start scrcpy')
          return
        }
        await init(true, pid, hwnd, isAutoSaveLocationAndSize, isWindowBorderless, configId)
        appWindow.close()

        updateDeviceLastSeenAt(device.id)
        updateConfigLastSeenAt(config.id)

        return
      }


      console.log('normal startup')

      await init()
      appWindow.close()
    }, 10)
  })

</script>