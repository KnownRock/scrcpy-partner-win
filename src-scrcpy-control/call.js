// @ts-nocheck
// call python main.py

const { spawn } = require('child_process')
const pyProg = spawn('python', ['main.py'])

pyProg.stdout.on('data', function (data) {
  console.log(data.toString())
})

pyProg.stderr.on('data', (data) => {
  console.error(data.toString())
})

pyProg.on('close', (code) => {
  console.log(`child process exited with code ${code}`)
})

// pyProg.stdin.write('touch 540 24')

async function main () {
  await new Promise((resolve, reject) => {
    setTimeout(() => {
      resolve()
    }, 2000)
  })

  console.log('done')

  async function writeData (str) {
    await new Promise((resolve, reject) => {
      pyProg.stdin.write(str, (err) => {
        if (err) {
          reject(err)
        } else {
          resolve()
        }
      })
    })
  }


  // pyProg.stdin.write('touch 540 24')
  await writeData('touch 540 24 0\n')
  const now = new Date()
  for (let i = 0; i < 95; i++) {
    await writeData(`touch 540 ${24 * i} 2\n`)
  }
  console.log(new Date() - now)
  await writeData('touch 540 2280 1\n')
}
main()
// Path: main.py
