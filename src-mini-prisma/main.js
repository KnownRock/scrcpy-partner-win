const args = process.argv.slice(2)

const url = args[0]
const pipeName = args[1]

async function main () {
  const init = require('./server')

  await init(url, pipeName)
}
main()
