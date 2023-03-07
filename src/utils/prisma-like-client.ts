import type { PrismaClient } from '@prisma/client/index'
import { callTauriFunction } from './tauri'
import jsonSchema from '../model/json-schema.json' assert { type: 'json' }
const allTables = Object.keys(jsonSchema.properties)

// const allTables = ['user', 'post', 'test'] as const

type PrismaAction =
    | 'findUnique'
    | 'findMany'
    | 'findFirst'
    | 'create'
    | 'createMany'
    | 'update'
    | 'updateMany'
    | 'upsert'
    | 'delete'
    | 'deleteMany'
    | 'executeRaw'
    | 'queryRaw'
    | 'aggregate'
    | 'count'
    | 'runCommandRaw'
    | 'findRaw'

const allPrismaFunctions: PrismaAction[] = [
  'findUnique',
  'findMany',
  'findFirst',
  'create',
  'createMany',
  'update',
  'updateMany',
  'upsert',
  'delete',
  'deleteMany',
  'executeRaw',
  'queryRaw',
  'aggregate',
  'count',
  'runCommandRaw',
  'findRaw'
]

type PrismaClientLike = {
  [table in typeof allTables[number]]: {
    [action in PrismaAction]: (arg: any) => Promise<any>
  }
}

const prismaClientLike = allTables.reduce(
  (acc, table) => ({
    ...acc,
    [table]: allPrismaFunctions.reduce(
      (acc, action) => ({
        ...acc,
        [action]: async (arg: any) => {
          // console.log(`prisma.${table}.${action}(${arg})`)
          // return {}
          let argJson = '{}'
          if (arg !== undefined && arg !== null) {
            argJson = JSON.stringify(arg)
          }

          const rawOutput = await callTauriFunction('call_prisma', {
            table,
            func: action,
            argJson
          })

          const result = JSON.parse(rawOutput as string)
          if ((result.error as object | undefined) != null) {
            throw new Error(result.error)
          }
          return result.data
        }
      }),
      {}
    )
  }),
  {}
) as PrismaClientLike

// FIXME: make this more type safe
export default prismaClientLike as unknown as PrismaClient
