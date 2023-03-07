const url = 'file:..\\prisma\\main.db'
const pipeName = '\\\\.\\pipe\\' + 'spw-mini-prisma'

async function main () {
  const init = require('./server')

  await init(url, pipeName)
}
main()
