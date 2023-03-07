import { writable } from 'svelte/store'
import { configForm, generalDialogForm } from '.'
import prismaClientLike from '../utils/prisma-like-client'
import { deleteConfigById } from '../utils/configs'
const store = writable({
  show: false,
  deviceId: ''
})

store.subscribe(value => {
  (async () => {
    async function setDialog (
      formItems: FormItem[]
    ): Promise<void> {
      // debugger
      generalDialogForm.set({
        title: 'Config',
        show: value.show,
        formItems,
        buttons: [
          {
            label: 'Cancel',
            action: 'cancel',
            callback: (entity, formItems) => {
              return true
            }
          },
          {
            label: 'new',
            action: 'new',
            defaultAction: true,
            callback: async (entity, formItems) => {
            // const args = formEntityToArgs(entity)
            // await lanuchSelf(args)
            // FIXME: this is a hack to make the form show up
              setTimeout(() => {
                configForm.set({
                  show: true,
                  deviceId: value.deviceId,
                  type: 'new-by-device'
                })
              }, 100)

              return true
            }
          }
        ],
        cancelCallback: (response, formItems) => {
          return true
        }
      })
    }

    async function getForm (
      currentDeviceId: string
    ): Promise<FormItem[]> {
      const configs = await prismaClientLike.deviceConfig.findMany({
        where: {
          deviceId: currentDeviceId
        }
      })

      const form: FormItem[] = [
        {
          type: 'header',
          label: 'General',
          name: 'general'
        },
        {
          type: 'table',
          label: 'Configs',
          name: 'configs',
          value: configs,
          columns: [
            {
              label: 'Name',
              name: 'name'
            },
            {
              buttons: [{
                label: 'Edit',
                callback: (row) => {
                  console.log('edit', row)
                  setTimeout(() => {
                    configForm.set({
                      show: true,
                      deviceConfigId: row.id,
                      type: 'edit'
                    })
                  }, 100)
                }
              }, {
                label: 'Delete',
                // eslint-disable-next-line @typescript-eslint/no-misused-promises
                callback: async (row) => {
                  console.log('delete', row)
                  void deleteConfigById(row.id)

                  const formItems = await getForm(value.deviceId)
                  void setDialog(formItems)
                }
              }],
              name: 'actions',
              label: 'Actions'
            }
          ]
        }

      ]

      return form
    }

    const formItems = await getForm(value.deviceId)

    await setDialog(formItems)
  })().then(() => {
    console.log('config-select-form.ts: store.subscribe: then')
  }).catch(error => {
    console.error(error)
  })
})

export default store
