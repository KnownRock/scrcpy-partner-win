// import { PrismaClient } from '@prisma/client'
const { PrismaClient } = require('@prisma/client')
const net = require('net')


async function init (url, pipeName) {
  const prisma = new PrismaClient({
    datasources: {
      db: {
        url
      }
    }
  })

  await prisma.$connect()

  const server = net.createServer((socket) => {
    socket.on('data', (jsonBuffer) => {
      // const args = data.toString().split(' ')
      // const tableName = args[0]
      // const functionName = args[1]
      // const functionArg = (args[2] && JSON.parse(args[2])) ?? undefined


      // console.log(jsonBuffer.toString())

      const { table, func, arg_json: json } = JSON.parse(jsonBuffer.toString())

      const argJson = json ? JSON.parse(json) : undefined

      if (func === 'exit') {
        process.exit(0)
      }


      console.log(table, func, argJson)

      try {
        prisma[table][func](argJson).then((result) => {
          // console.log(JSON.stringify(result))
          socket.write(JSON.stringify({ data: result }))
        }).catch((e) => {
          console.error(e)
          socket.write(JSON.stringify({ error: e.message }))
        }).finally(() => {
          setTimeout(() => {
            socket.end()
          }, 0)
        })
      } catch (error) {
        console.error(error)
        socket.write(JSON.stringify({ error: error.message }))
        socket.end()
      }
    })
  })

  server.listen(pipeName, () => {
    console.log('listening on ' + pipeName)
  })
}

module.exports = init
