import { spawn, exec } from 'child_process'
import process from 'process'
import { promisify } from 'util'

const execAsync = promisify(exec)

async function main () {
  const args = process.argv.slice(2)

  await execAsync(
    'copy  .\\scrcpy-server.jar .\\src-tauri\\target\\debug\\scrcpy-server.jar'
  )

  // when in dev mode use this location mini-prisma.exe
  await execAsync(
    'copy  .\\scrcpy-server.jar .\\src-tauri\\target\\release\\scrcpy-server.jar'
  )

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
