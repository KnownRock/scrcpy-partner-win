// import { PrismaClient } from '@prisma/client'
const { PrismaClient } = require('@prisma/client')
const args = process.argv.slice(2)

const url = args[0]

const prisma = new PrismaClient({
  datasources: {
    db: {
      url
    }
  }
})


const tableName = args[1]
const functionName = args[2]
const functionArg = (args[3] && JSON.parse(args[3])) ?? undefined


prisma.$connect().then(async () => {
  const result = await prisma[tableName][functionName](functionArg)
  // https://stackoverflow.com/questions/63492126/key-must-be-a-string-when-deserializing-a-json-string-with-serde
  console.log(JSON.stringify(result))
  prisma.$disconnect()
})
