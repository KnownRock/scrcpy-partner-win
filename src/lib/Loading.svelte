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

  import { onMount } from 'svelte'
  import { init, getEnvArgs, closeApplication, runScrcpyCommand } from '../utils/app'
  import { connectTcpipDevice, getDevices, type DeviceExt } from '../utils/devices'
  import Button from '@smui/button'

  let error = ''

  function getSerialDevicesFromArgs (args) {
    const devicesInArgs:string[] = []
    args.forEach((arg, index) => {
      if (arg === '-s') {
        devicesInArgs.push(args[index + 1])
      } else if (arg === '--serial') {
        devicesInArgs.push(args[index + 1])
      } else if (arg.match(/^(-s|--serial)=?/)) {
        devicesInArgs.push(arg.match(/^(-s|--serial)(=|\s)(.*)/)[3])
      }
    })

    return devicesInArgs
  }

  function getTcpipDeviceFromArgs (args) {
    const devicesInArgs:string[] = []
    args.forEach((arg, index) => {
      if (arg === '-t') {
        devicesInArgs.push(args[index + 1])
      } else if (arg === '--tcpip') {
        devicesInArgs.push(args[index + 1])
      } else if (arg.match(/^(-t|--tcpip)=?/)) {
        devicesInArgs.push(arg.match(/^(-t|--tcpip)(=|\s)(.*)/)[3])
      }
    })

    return devicesInArgs
  }

  function replceTcpipArgToSerial (args) {
    const newArgs = args.map((arg, index) => {
      if (arg === '-t') {
        return '-s'
      } else if (arg === '--tcpip') {
        return '--serial'
      } else if (arg.match(/^(-t|--tcpip)=?/)) {
        return arg.replace(/^(-t|--tcpip)(=|\s)(.*)/, '$1$2$3')
      } else {
        return arg
      }
    })

    return newArgs
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
      const args = (await getEnvArgs()).slice(1)
      const deviceDict = await getAdbDeviceDict()

      const devicesInArgs = getSerialDevicesFromArgs(args)
      console.log('devicesInArgs', devicesInArgs)

      const tcpipDevicesInArgs = getTcpipDeviceFromArgs(args)

      if (devicesInArgs.length + tcpipDevicesInArgs.length > 1) {
        error = 'Only one device can be selected at a time'
      }

      if (devicesInArgs.length === 1) {
        const device = devicesInArgs[0]
        if (deviceDict[device]) {
          runScrcpyCommand(args)
          // init()
        } else {
          error = `Device ${device} not connected`
        }
      }


      if (tcpipDevicesInArgs.length === 1) {
        const device = tcpipDevicesInArgs[0]
        await connectTcpipDevice(device)
        const newDeviceDict = await getAdbDeviceDict()
  
        if (newDeviceDict[device]) {
          const newArgs = replceTcpipArgToSerial(args)
          // init()
          // change tcpip arg to serial arg
        } else {
          error = `Device ${device} not connected`
        }
      }

      // TODO: if is config arg
    }, 200)


    // make time for the splash screen to show before loading the app
    // setTimeout(() => {
    //   init()
    // }, 200)
  })

</script>