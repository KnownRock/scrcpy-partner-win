// exec command: prisma generate

import { exec } from 'child_process'
import { promisify } from 'util'

const execAsync = promisify(exec)


// exec('npx prisma generate', (err, stdout, stderr) => {
//   if (err) {
//     console.error(err)
//     return
//   }
//   console.log(stdout)
// })


async function main () {
  const { stdout, stderr } = await execAsync('npx prisma generate')
  console.log(stdout)

  const { stdout: stdout2, stderr: stderr2 } = await execAsync(
    'npx pkg -c ./package.json ./main.js',
    {
      cwd: './src-mini-prisma'
    }
  )

  console.log(stdout2)

  const { stdout: stdout3, stderr: stderr3 } = await execAsync(
    'copy  .\\main.exe ..\\src-tauri\\target\\debug\\mini-prisma.exe',
    {
      cwd: './src-mini-prisma'
    }
  )
  console.log(stdout3)


  const { stdout: stdout4, stderr: stderr4 } = await execAsync(
    'copy  .\\main.exe ..\\src-tauri\\target\\release\\mini-prisma.exe',
    {
      cwd: './src-mini-prisma'
    }
  )

  console.log(stdout4)

  const { stdout: stdout5, stderr: stderr5 } = await execAsync(
    'del .\\main.exe',
    {
      cwd: './src-mini-prisma'
    }
  )

  console.log(stdout5)

  console.log('stderr:')
  console.log(stderr)
  console.log(stderr2)
  console.log(stderr3)
  console.log(stderr4)
  console.log(stderr5)
}

main()
