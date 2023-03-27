<div class="record-interface" 
  bind:clientHeight={height}
  bind:clientWidth={width}
  on:mousedown={handleMouseDown}
  on:mouseup={handleMouseUp}
  on:mousemove={throttleMouseMove}
  on:contextmenu={handleContextMenu}
  on:mouseleave={handleMouseLeave}
  on:wheel={handleWheel}
>
</div>

<script lang="ts">
  import { onMount } from 'svelte'
  import { KeyEventType, MotionType, type RecordOperation } from '../../types'
  import { execAdbCommand } from './AdbCmd'

  let height = 0
  let width = 0
  let deviceSize = [0, 0]


  export let execute: (operation: RecordOperation) => void
  export let adbId: string

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
    execute({
      type: 'motion',
      motionType: MotionType.Down,
      x,
      y
    })
  }

  async function handleWheel (e: WheelEvent) {
    console.log('wheel', e)
    const [x, y] = getXYFromEvent(e)
    execute({
      type: 'scroll',
      x,
      y,
      v: e.deltaX,
      h: e.deltaY
    })
  }


  async function handleMouseUp (e: MouseEvent) {
    if (e.button !== 0) return

    console.log('mouseup', e)
    isMouseDown = false
    const [x, y] = getXYFromEvent(e)
    // addMouseEvent(MotionType.Up, x, y)
    execute({
      type: 'motion',
      motionType: MotionType.Up,
      x,
      y
    })
  }

  function handleContextMenu (e: MouseEvent) {
    execute({
      type: 'keyevent',
      key: 4,
      keyEventType: KeyEventType.Down
    })

    execute({
      type: 'keyevent',
      key: 4,
      keyEventType: KeyEventType.Up
    })

    e.preventDefault()
  }


  async function handleMouseLeave (e: MouseEvent) {
    if (e.button !== 0) return

    console.log('mouseleave', e)
    isMouseDown = false
    const [x, y] = getXYFromEvent(e)
    execute({
      type: 'motion',
      motionType: MotionType.Up,
      x,
      y
    })
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
    // addMouseEvent(MotionType.Move, x, y)
    execute({
      type: 'motion',
      motionType: MotionType.Move,
      x,
      y
    })
  }

  const throttleMouseMove = throttle(handleMouseMove, 10)

  async function getDeviceSize (adbId: string) {
    const text = await execAdbCommand(adbId, ['shell', 'wm', 'size'])
    const matched = text.trim().match(/(\d+)x(\d+)$/)
    if (matched) {
      const [, width, height] = matched
      return [width, height].map(Number)
    }
    throw new Error('get device size failed')
}

  async function initDeviceSize () {
    deviceSize = await getDeviceSize(adbId)
  }
  onMount(() => {
    initDeviceSize()
  })

</script>

<style>
  .record-interface{
    flex: 1;
    background-color: transparent;
  }
</style>