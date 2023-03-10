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
  const { stdout, stderr } = await execAsync(
    'npx prisma generate',
    {
      env: {
        ...process.env,
        DATABASE_URL: 'file:./main.db'
      }
    }
  )
  console.log(stdout)

  const { stdout: stdout1, stderr: stderr1 } = await execAsync(
    'npx prisma migrate dev --name init',
    {
      env: {
        ...process.env,
        DATABASE_URL: 'file:./main.db'
      }
    }
  )
  console.log(stdout1)


  const { stdout: stdout2, stderr: stderr2 } = await execAsync(
    'npx pkg --compress GZip -c ./package.json ./main.js',
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


  // when in dev mode use this location mini-prisma.exe
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
  console.error(stderr)
  console.error(stderr1)
  console.error(stderr2)
  console.error(stderr3)
  console.error(stderr4)
  console.error(stderr5)
}

main()
