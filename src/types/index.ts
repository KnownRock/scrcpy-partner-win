
export enum KeyEventType {
  Down = 0,
  Up = 1
}
export enum MotionType {
  Down = 0,
  Up = 1,
  Move = 2
}
export interface RecordOperationGroup {
  type: 'group'
  name: string
  operations: RecordOperation[]
}

export type RecordOperation = ({
  type: 'tap'
  x: number
  y: number
} | {
  type: 'keyevent'
  key: number
  keyEventType: KeyEventType
} | {
  type: 'am_start'
  packageName: string
} | {
  type: 'motion'
  motionType: MotionType
  x: number
  y: number
} | {
  type: 'scroll'
  x: number
  y: number
  v: number
  h: number
}
| RecordOperationGroup
| {
  type: 'delay'
  ms: number
}
| {
  type: 'exec_script'
  scriptId: string
  name: string
}
| {
  type: 'adb_cmd'
  cmd: string
}
| {
  type: 'message'
  message: string
}
)
