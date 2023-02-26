
interface DialogFormButton {
  label: string
  callback: (formItems: FormItem[]) => boolean
  defaultAction?: boolean
  action?: 'submit' | 'cancel'
}

type FormItem = {
  type: 'option'
  label: string
  name: string
  options: Array<{ label: string, value: string }>
  value: string
  defaultValue?: string
}
| {
  type: 'switch'
  label: string
  name: string
  value: boolean
}
| {
  type: 'text'
  label: string
  name: string
  value: string
}
| {
  type: 'number'
  label: string
  name: string
  value: number
}
| {
  type: 'auto'
  label: string
  name: string
  value: string
  options: string[]
}
| {
  type: 'optional-auto'
  label: string
  name: string
  value: string
  enable: boolean
  options: string[]
}
| {
  type: 'optional-text'
  label: string
  name: string
  value: string
  enable: boolean
}
| {
  type: 'optional-number'
  label: string
  name: string
  value: number
  enable: boolean
}
| {
  type: 'optional-option'
  label: string
  name: string
  options: Array<{ label: string, value: string }>
  value: string
  defaultValue?: string
  enable: boolean
}
| {
  type: 'header'
  label: string
  name: string
}
