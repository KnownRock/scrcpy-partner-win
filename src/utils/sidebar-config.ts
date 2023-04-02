
import generalDialogForm from '../store/general-dialog-form'
import prismaClientLike from './prisma-like-client'

export async function setDialog (buttons): Promise<void> {
  const configs = await prismaClientLike.sideBarConfig.findMany()
  generalDialogForm.set({
    title: 'Sidebar Config',
    show: true,
    buttons: [{
      label: 'Cancel',
      action: 'cancel',
      callback: () => {
        return true
      }
    }],
    formItems: [
      {
        type: 'table',
        columns: [
          {
            label: 'Name',
            name: 'name'
          },
          {
            label: 'Actions',
            name: 'actions',
            buttons
          }
        ],
        label: 'Record Scripts',
        name: 'recordScripts',

        value: configs as Array<Record<string, any>>
      }
    ],
    cancelCallback: () => {
      return true
    }
  })
}

export async function hide (): Promise<void> {
  generalDialogForm.set({
    show: false,
    title: '',
    buttons: [],
    formItems: [],
    cancelCallback: () => {
      return true
    }
  })
}
