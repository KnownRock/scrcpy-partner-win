// import { PrismaClient } from '@prisma/client'
const { PrismaClient } = require('@prisma/client')

const prisma = new PrismaClient({
  datasources: {
    db: {
      url: 'file:../prisma/main.db'
    }
  }
})

const args = process.argv.slice(2)

const tableName = args[0]
const functionName = args[1]
const functionArgs = (args[2] && JSON.parse(args[2])) || []


prisma.$connect().then(async () => {
  const result = await prisma[tableName][functionName](...functionArgs)
  console.log(result)
  prisma.$disconnect()
})
