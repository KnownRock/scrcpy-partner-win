
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

type FormItem = {
  type: 'option'
  label: string
  name: string
  options: Array<{ label: string, value: string }>
  value: string
  defaultValue?: string
  disabled?: boolean
}
| {
  type: 'message'
  label: string
  name: string
  value: string
}
| {
  type: 'switch'
  label: string
  name: string
  value: boolean
  disabled?: boolean
}
| {
  type: 'text'
  label: string
  name: string
  value: string
  disabled?: boolean
}
| {
  type: 'number'
  label: string
  name: string
  value: number
  disabled?: boolean
}
| {
  type: 'auto'
  label: string
  name: string
  value: string
  options: string[]
  disabled?: boolean
}
| {
  type: 'optional-auto'
  label: string
  name: string
  value: string
  enable: boolean
  options: string[]
  disabled?: boolean
}
| {
  type: 'optional-text'
  label: string
  name: string
  value: string
  enable: boolean
  disabled?: boolean
}
| {
  type: 'optional-number'
  label: string
  name: string
  value: number
  enable: boolean
  disabled?: boolean
}
| {
  type: 'optional-option'
  label: string
  name: string
  options: Array<{ label: string, value: string }>
  value: string
  defaultValue?: string
  enable: boolean
  disabled?: boolean
}
| {
  type: 'header'
  label: string
  name: string
  disabled?: boolean
}
| {
  type: 'table'
  label: string
  name: string
  value: Array<Record<string, any>>
  disabled?: boolean
  columns: Array<{
    label: string
    name: string
    buttons?: Array<{
      label: string
      callback: (row: Record<string, string>) => void
    }>
  }>

}
