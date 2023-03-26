export enum KeyEventType {
  Down = 0,
  Up = 1
}
export enum MotionType {
  Down = 0,
  Up = 1,
  Move = 2
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
})

export type RecordOperationWithTime = RecordOperation & {
  time: Date
}
