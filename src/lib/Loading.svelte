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
    <Button on:click={closeApplication}>
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
  import { init, getEnvArgs, closeApplication, runScrcpyCommand } from '../utils/app'
  import {
    connectTcpipDevice, getDevices,
    updateDeviceLastSeenAt
    , type DeviceExt
  } from '../utils/devices'
  import Button from '@smui/button'
  import { getConfigById, updateConfigLastSeenAt } from '../utils/configs'
  import { argsTemplate } from '../utils/scrcpy'

  async function sleep (ms) {
    return new Promise(resolve => setTimeout(resolve, ms))
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
    }

    return args
  }


  onMount(() => {
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
          await runScrcpyCommand(getScrcpyArgsFromArgs(args))
          init(true)
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
          await runScrcpyCommand(getScrcpyArgsFromArgs(args).concat(['--serial', device]))
          init(true)
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
  
        await runScrcpyCommand(getScrcpyArgsFromArgs(getArgsFromRawArgs(rawArgs)))

        await init(true, isAutoSaveLocationAndSize, isWindowBorderless, configId)

        updateDeviceLastSeenAt(device.id)
        updateConfigLastSeenAt(config.id)

        return
      }


      console.log('normal startup')

      await init()
    }, 10)
  })

</script>