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
    ['npx prisma generate', { env: { ...process.env, DATABASE_URL: 'file:./main.db' } }],
    ['npx prisma migrate dev --name init', { env: { ...process.env, DATABASE_URL: 'file:./main.db' } }],
    ['npx pkg --compress GZip -c ./package.json ./main.js', { cwd: './src-mini-prisma' }],
    ['copy  .\\main.exe ..\\src-tauri\\target\\debug\\mini-prisma.exe', { cwd: './src-mini-prisma' }],
    ['copy  .\\main.exe ..\\src-tauri\\target\\release\\mini-prisma.exe', { cwd: './src-mini-prisma' }],
    ['del .\\main.exe', { cwd: './src-mini-prisma' }]
  ]


  for (const cmd of cmds) {
    console.log(`> ${cmd[0]}`)
    // @ts-ignore
    await execute(cmd[0], cmd[1])
  }
}

main()
