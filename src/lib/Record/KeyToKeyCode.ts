// JS Key to Android Key Code
// https://developer.android.com/reference/android/view/KeyEvent.html
const keyToKeyCode = {
  Backspace: 67,
  Tab: 61,
  Enter: 66,
  Shift: 59,
  Control: 113,
  Alt: 57,
  CapsLock: 115,
  Escape: 111,
  ' ': 62,
  PageUp: 92,
  PageDown: 93

}

export function toKeyCode (key: string): number {
  if (key.length === 1) {
    return key.charCodeAt(0)
  }
  return keyToKeyCode[key]
}
