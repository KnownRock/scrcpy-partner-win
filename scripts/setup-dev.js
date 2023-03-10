import { spawn } from 'child_process'
import process from 'process'
async function main () {
  const args = process.argv.slice(2)


  console.log('running prisma server')
  spawn('node', ['./src-mini-prisma/dev.js'], {
    shell: true,
    stdio: 'inherit'
  })

  console.log('running tauri dev')
  spawn('npm', ['run', 'tauri', 'dev', ...args], {
    shell: true,
    stdio: 'inherit'
  })
}


main()
