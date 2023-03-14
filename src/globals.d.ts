
interface DialogFormButton {
  label: string
  callback: (
    // TODO: make this type more specific
    entity: Record<FormItem['name'], FormItem['value']>,
    formItems: FormItem[]
  ) => Promise<boolean> | boolean
  defaultAction?: boolean
  action?: string
}

interface MsgButton {
  label: string
  icon?: string
  callback: () => void
}

type FormItem = ({
  type: 'option'
  options: Array<{ label: string, value: string }>
  value: string
  defaultValue?: string
}
| {
  type: 'message'
  value: string
}
| {
  type: 'switch'
  value: boolean
}
| {
  type: 'text'
  value: string
}
| {
  type: 'number'
  value: number
}
| {
  type: 'auto'
  value: string
  options: string[]
}
| {
  type: 'optional-auto'
  value: string
  enable: boolean
  options: string[]
}
| {
  type: 'optional-text'
  value: string
  enable: boolean
}
| {
  type: 'optional-number'
  value: number
  enable: boolean
}
| {
  type: 'optional-switch'
  value: boolean
  enable: boolean
}
| {
  type: 'optional-auto-number'
  value: number
  enable: boolean
  options: number[]
}
| {
  type: 'optional-option'
  options: Array<{ label: string, value: string }>
  value: string
  defaultValue?: string
  enable: boolean
}
| {
  type: 'header'
}
| {
  type: 'table'
  value: Array<Record<string, any>>
  columns: Array<{
    label: string
    name: string
    buttons?: Array<{
      label: string
      callback: (row: Record<string, string>) => void
    }>
  }>

}) & {
  label: string
  name: string
  description?: string
  disabled?: boolean
}
