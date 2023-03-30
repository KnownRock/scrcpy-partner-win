import { exec } from 'child_process'
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
  const cmds = [
    ['npx prisma generate', { env: { ...process.env, DATABASE_URL: 'file:./prod.db' } }],
    ['npx prisma migrate deploy', { env: { ...process.env, DATABASE_URL: 'file:./prod.db' } }],
    ['npx pkg --compress GZip -c ./package.json ./main.js', { cwd: './src-mini-prisma' }],
    ['copy  .\\prisma\\prod.db .\\src-tauri\\target\\release\\main.db'],
    ['copy  .\\main.exe ..\\src-tauri\\target\\release\\mini-prisma.exe', { cwd: './src-mini-prisma' }],
    ['copy  .\\libs\\scrcpy-server.jar .\\src-tauri\\target\\debug\\scrcpy-server.jar'],
    ['copy  .\\libs\\scrcpy-server.jar .\\src-tauri\\target\\release\\scrcpy-server.jar'],
    ['del .\\main.exe', { cwd: './src-mini-prisma' }],
    ['del  .\\prisma\\prod.db*'],
    ['npx tauri build'],
    ['pyinstaller main.spec', { cwd: './src-scrcpy-control' }],
    ['copy  .\\dist\\scrcpy-control.exe ..\\src-tauri\\target\\release\\scrcpy-control.exe', { cwd: './src-scrcpy-control' }]
  ]

  for (const cmd of cmds) {
    console.log(`> ${cmd[0]}`)
    // @ts-ignore
    await execute(cmd[0], cmd[1])
  }
}
main()
