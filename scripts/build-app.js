// exec command: prisma generate

import { exec } from 'child_process'
import { promisify } from 'util'

const execAsync = promisify(exec)

async function main () {
  const { stdout, stderr } = await execAsync(
    'npx prisma generate',
    {
      env: {
        ...process.env,
        DATABASE_URL: 'file:./prod.db'
      }
    }
  )
  console.log(stdout)

  const { stdout: stdout1, stderr: stderr1 } = await execAsync(
    'npx prisma migrate deploy',
    {
      env: {
        ...process.env,
        DATABASE_URL: 'file:./prod.db'
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
    'copy  .\\prisma\\prod.db .\\src-tauri\\target\\release\\main.db'
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

  const { stdout: stdout6, stderr: stderr6 } = await execAsync(
    'del  .\\prisma\\prod.db*'
  )

  console.log(stdout6)

  const { stdout: stdout7, stderr: stderr7 } = await execAsync(
    'npx tauri build'
  )

  console.log(stdout7)


  console.log('stderr:')
  console.log(stderr)
  console.log(stderr1)
  console.log(stderr2)
  console.log(stderr3)
  console.log(stderr4)
  console.log(stderr5)
  console.log(stderr6)
  console.log(stderr7)
}

main()