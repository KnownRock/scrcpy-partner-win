import { Command } from '@tauri-apps/api/shell'

export async function execAdbCommand (
  adbId: string,
  args: string[]
): Promise<string> {
  return await new Promise<string>((resolve, reject) => {
    const cmd = import.meta.env.MODE === 'development'
      ? new Command('adb-dev', ['-s', adbId, ...args])
      : new Command('adb', ['-s', adbId, ...args])

    // const cmd = new Command('adb', ['-s', adbId, ...args])
    const lines = [] as string[]
    cmd.stdout.on('data', (line: string) => {
      lines.push(line)
    })

    cmd.on('close', () => {
      resolve(lines.join('\n'))
    })

    cmd.on('error', (err) => {
      reject(err)
    })

    cmd.execute().catch(err => {
      reject(err)
    })
  })
}
