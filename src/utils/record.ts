
import generalDialogForm from '../store/general-dialog-form'
import prismaClientLike from './prisma-like-client'

export async function setDialog (buttons): Promise<void> {
  const records = await prismaClientLike.recordScript.findMany({})
  generalDialogForm.set({
    title: 'Record',
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
            label: 'Description',
            name: 'description'
          },
          {
            label: 'Actions',
            name: 'actions',
            buttons
          }
        ],
        label: 'Record Scripts',
        name: 'recordScripts',

        value: records as Array<Record<string, any>>
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
