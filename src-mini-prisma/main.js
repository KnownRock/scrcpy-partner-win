// import { PrismaClient } from '@prisma/client'
const { PrismaClient } = require('@prisma/client')
const args = process.argv.slice(2)
const net = require('net')

const url = args[0]
const pipeName = args[1]

const prisma = new PrismaClient({
  datasources: {
    db: {
      url
    }
  }
})

async function main () {
  await prisma.$connect()

  const server = net.createServer((socket) => {
    socket.on('data', (jsonBuffer) => {
      // const args = data.toString().split(' ')
      // const tableName = args[0]
      // const functionName = args[1]
      // const functionArg = (args[2] && JSON.parse(args[2])) ?? undefined

      console.log(jsonBuffer.toString())

      const { table, func, arg_json } = JSON.parse(jsonBuffer.toString())

      const argJson = arg_json ? JSON.parse(arg_json) : undefined


      console.log(table, func, argJson)

      prisma[table][func](argJson).then((result) => {
        console.log(JSON.stringify(result))
        socket.write(JSON.stringify(result))
      })
    })
  })

  server.listen(pipeName, () => {
    console.log('listening on ' + pipeName)
  })
}


main()
