<div style="
display: flex; justify-content: center;
height: calc(100vh - 2px) ; align-items: center;
width: calc(100vw - 2px); flex-direction: column;
border: 1px solid #ccc;
">
  <h1>Scrcpy Partner</h1>
  <!-- <h1>Loading...</h1> -->
  {#if !error}
  <CircularProgress style="height: 32px; width: 32px;" indeterminate />
  {/if}

  {#if error}
    <h3>{error}</h3>
    <Button on:click={closeApplication}>
      Close
    </Button>
  {/if}
  
</div>

<script lang="ts">
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

  let error = ''

  function getSerialDevicesFromArg (args) {
    return args['--serial']
  }

  function getTcpipDeviceFromArgs (args) {
    return args['--tcpip']
  }

  // TODO: replace for new args
  function replceTcpipArgToSerial (args) {
    const newArgs = args.map((arg, index) => {
      if (arg === '-t') {
        return '-s'
      } else if (arg === '--tcpip') {
        return '--serial'
      } else if (arg.match(/^(-t|--tcpip)(=|\s)?(.*)/)) {
        return arg.replace(/^(-t|--tcpip)(=|\s)?(.*)/, '$1$2$3')
      } else {
        return arg
      }
    })

    return newArgs
  }

  // function getAdjustedLocationAndSizeToFitWithTitleBar (arg) {


  // }

  function getConfigIdFromArgs (args) {
    return args['--spw-config-id']
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


  onMount(() => {
    setTimeout(async () => {
      const rawArgs = (await getEnvArgs()).slice(1)
      const deviceDict = await getAdbDeviceDict()

      let args : Result<typeof argsTemplate> | null = null
      try {
        args = arg(argsTemplate, {
          argv: rawArgs,
          permissive: true
        })
      } catch (e) {
      }

      if (!args) {
        error = 'Invalid arguments'
        return
      }


      console.log('args', args)

      const serialDeviceInArgs = getSerialDevicesFromArg(args)
      console.log('serialDeviceInArgs', serialDeviceInArgs)


      const tcpipDeviceInArgs = getTcpipDeviceFromArgs(args)
      console.log('tcpipDeviceInArgs', tcpipDeviceInArgs)


      if (serialDeviceInArgs && tcpipDeviceInArgs) {
        error = 'Only one device can be selected at a time'
      }


      if (serialDeviceInArgs) {
        const device = serialDeviceInArgs
        if (deviceDict[device]) {
          await runScrcpyCommand(rawArgs)
          init()
        } else {
          error = `Device ${device} not connected`
        }

        return
      }


      if (tcpipDeviceInArgs) {
        const device = tcpipDeviceInArgs
        await connectTcpipDevice(device)
        const newDeviceDict = await getAdbDeviceDict()
  
        if (newDeviceDict[device]) {
          const newArgs = replceTcpipArgToSerial(args)
          await runScrcpyCommand(newArgs)
          init()
          // change tcpip arg to serial arg
        } else {
          error = `Device ${device} not connected`
        }

        return
      }

      const configId:string = getConfigIdFromArgs(args)
      if (configId) {
        console.log('configId', configId)

        const config = await getConfigById(configId)

        console.log('config', config)

        if (!config) {
          error = `Config ${configId} not found`
          return
        }

        const args = config.deviceConfigValue.map(el => {
          const value = JSON.parse(el.value)
          if (el.key.startsWith('spw-')) {
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
          error = `Device ${config.deviceId} not found`
          return
        }

        if (device.isTcpipDevice && !device.isConnected) {
          await connectTcpipDevice(device.adbId)
        }

        args.unshift('--serial=' + device.adbId)
        console.log('args', args)

        const isAutoSaveLocationAndSize =
          config.deviceConfigValue
            .find(el => el.key === 'spw-autosave-location-size')
            ?.value === 'true'


        await runScrcpyCommand(args)


        await init(true, isAutoSaveLocationAndSize, configId)

        updateDeviceLastSeenAt(device.id)
        updateConfigLastSeenAt(config.id)

        return
      }


      console.log('normal startup')


      // prismaClientLike.deviceConfigValue.update({
      //   where: {
      //     key: 'spw-autosave-location-size'
      //   }
      //   data: {
      //     value: JSON.stringify(true)
      //   }
      // })

      await init()
  

      // TODO: if is config arg
    }, 200)


    // make time for the splash screen to show before loading the app
    // setTimeout(() => {
    //   init()
    // }, 200)
  })

</script>