// import { PrismaClient } from '@prisma/client'
const { PrismaClient } = require('@prisma/client')

const prisma = new PrismaClient({
  datasources: {
    db: {
      url: 'file:../prisma/main.db'
    }
  }
})

async function main () {
  await prisma.$connect()

  prisma.deviceConfig.create({
    data: {
      device: {
        connect: {
          id: '7739c809-4cb0-41b5-8f04-e9b29cc437ac'
        }
      },
      name: 'test1'
    }
  }).then((result) => {
    console.log(result)
  })
}


main()
