<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
<!-- https://stackoverflow.com/questions/3149362/how-do-i-capture-a-key-press-or-keydown-event-on-a-div-element -->
<div class="record-interface" 
  tabindex="0"
  bind:clientHeight={height}
  bind:clientWidth={width}
  on:mousedown={handleMouseDown}
  on:mouseup={handleMouseUp}
  on:mousemove={throttleMouseMove}
  on:contextmenu={handleContextMenu}
  on:mouseleave={handleMouseLeave}
  on:wheel={handleWheel}
  on:keydown={handleKeyDown}
  on:keyup={handleKeyUp}
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

    let newWidth = width
    let newHeight = height

    if (width / height > deviceWidth / deviceHeight) {
      newWidth = height * deviceWidth / deviceHeight
    } else {
      newHeight = width * deviceHeight / deviceWidth
    }

    const offsetX = e.offsetX - (width - newWidth) / 2
    const offsetY = e.offsetY - (height - newHeight) / 2

    const x = Math.round(offsetX / newWidth * deviceWidth)
    const y = Math.round(offsetY / newHeight * deviceHeight)

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
      v: e.deltaY,
      h: e.deltaX
  
    })
  }

  const codeToKeyCode = {
    Digit0: 7,
    Digit1: 8,
    Digit2: 9,
    Digit3: 10,
    Digit4: 11,
    Digit5: 12,
    Digit6: 13,
    Digit7: 14,
    Digit8: 15,
    Digit9: 16,
  
    KeyA: 29,
    KeyB: 30,
    KeyC: 31,
    KeyD: 32,
    KeyE: 33,
    KeyF: 34,
    KeyG: 35,
    KeyH: 36,
    KeyI: 37,
    KeyJ: 38,
    KeyK: 39,
    KeyL: 40,
    KeyM: 41,
    KeyN: 42,
    KeyO: 43,
    KeyP: 44,
    KeyQ: 45,
    KeyR: 46,
    KeyS: 47,
    KeyT: 48,
    KeyU: 49,
    KeyV: 50,
    KeyW: 51,
    KeyX: 52,
    KeyY: 53,
    KeyZ: 54,

    Backquote: 68,
    Minus: 69,
    Equal: 70,
    BracketLeft: 71,
    BracketRight: 72,
    Backslash: 73,
    Semicolon: 74,
    Quote: 75,
    Comma: 76,
    Period: 77,
    Slash: 78,

    Space: 62,
    Enter: 66,
    Backspace: 67,
    Tab: 61,
    CapsLock: 115,
    ShiftLeft: 59,
    ShiftRight: 60,
    ControlLeft: 113,
    ControlRight: 114,
    AltLeft: 57,
    AltRight: 58,
    MetaLeft: 117,
    MetaRight: 118,
    ArrowLeft: 21,
    ArrowRight: 22,
    ArrowUp: 19,
    ArrowDown: 20,
    Escape: 111,
    Delete: 112,
    Insert: 124,
    Home: 122,
    End: 123,
    PageUp: 92,
    PageDown: 93,
    F1: 131,
    F2: 132,
    F3: 133,
    F4: 134,
    F5: 135,
    F6: 136,
    F7: 137,
    F8: 138,
    F9: 139,
    F10: 140,
    F11: 141,
    F12: 142,

    Numpad0: 144,
    Numpad1: 145,
    Numpad2: 146,
    Numpad3: 147,
    Numpad4: 148,
    Numpad5: 149,
    Numpad6: 150,
    Numpad7: 151,
    Numpad8: 152,
    Numpad9: 153,
    NumpadAdd: 157,
    NumpadSubtract: 156,
    NumpadMultiply: 155,
    NumpadDivide: 154,
    NumpadDecimal: 158,
    NumpadEnter: 160,

    PrintScreen: 120,
    ScrollLock: 116,
    Pause: 121
  }


  async function handleKeyDown (e: KeyboardEvent) {
    console.log('keydown', e)
    execute({
      type: 'keyevent',
      key: codeToKeyCode[e.code],
      keyEventType: KeyEventType.Down
    })
  }

  async function handleKeyUp (e: KeyboardEvent) {
    console.log('keyup', e)
    execute({
      type: 'keyevent',
      key: codeToKeyCode[e.code],
      keyEventType: KeyEventType.Up
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

    if (!isMouseDown) return

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

  $:adbId && initDeviceSize()


  onMount(() => {
    // initDeviceSize()
  })

</script>

<style>
  .record-interface{
    flex: 1;
    background-color: transparent;
  }
</style>