import process from 'process'
import { exec, spawn } from 'child_process'
export async function execute (command, args) {
  const child = exec(command, args, (error, stdout, stderr) => {
    if (error) {
      console.error(`exec error: ${error}`)
      return
    }
    stdout && console.log(`stdout: ${stdout}`)
    stderr && console.error(`stderr: ${stderr}`)
  })

  return new Promise((resolve, reject) => {
    child.addListener('error', reject)
    child.addListener('exit', resolve)
  })
}


async function main () {
  const args = process.argv.slice(2)


  const cmds = [
    ['copy  .\\libs\\scrcpy-server.jar .\\src-tauri\\target\\debug\\scrcpy-server.jar'],
    ['copy  .\\libs\\scrcpy-server.jar .\\src-tauri\\target\\release\\scrcpy-server.jar']
  ]

  for (const cmd of cmds) {
    console.log(`> ${cmd[0]}`)
    // @ts-ignore
    await execute(cmd[0], cmd[1])
  }


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
