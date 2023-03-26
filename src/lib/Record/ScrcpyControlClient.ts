import { type Child, Command } from '@tauri-apps/api/shell'
import type { RecordOperation } from '../../types'
import { getCurrentExeDir } from '../../utils/app'

interface ScrcpyControlClientArgs {
  adbId: string
}

export default class ScrcpyControlClient {
  private adbShell: Child | null = null
  private readonly adbId = '' as string
  private controlShell: Child | null = null
  private readonly executeQueue: RecordOperation[] = []
  private executeLock = false
  constructor ({ adbId }: ScrcpyControlClientArgs) {
    this.adbId = adbId
  }

  async init (): Promise<void> {
    const adbId = this.adbId

    await this.initAdbShell(adbId)

    await this.initControlShell(adbId)
  }

  async initAdbShell (adbId: string): Promise<void> {
    const adbShellCmd = new Command('adb', ['-s', adbId, 'shell'])
    adbShellCmd.on('close', () => {
      console.log('adbShellCmd close')
    })
    adbShellCmd.on('error', (err) => {
      console.error('adbShellCmd error', err)

      console.log('try to reconnect')
      this.initAdbShell(adbId).catch(err => {
        console.error('reconnect error', err)
      })
    })
    adbShellCmd.stdout.on('data', line => { console.log(`command stdout: "${line as string}"`) })
    adbShellCmd.stderr.on('data', line => { console.error(`command stderr: "${line as string}"`) })

    this.adbShell = await adbShellCmd.spawn()
  }

  async initControlShell (adbId: string): Promise<void> {
    const currentExeDir = await getCurrentExeDir()
    const scrcpyJarPath = currentExeDir + '/scrcpy-server.jar'
    const pushScrcpyJarCmd = new Command('adb', ['-s', adbId, 'push', scrcpyJarPath, '/data/local/tmp/scrcpy-server.jar'])
    await pushScrcpyJarCmd.execute()

    const ctrlCmd = new Command('scrcpy-control', [adbId])
    ctrlCmd.on('close', () => {
      console.log('ctrlCmd close')
    })
    ctrlCmd.on('error', (err) => {
      console.error('ctrlCmd error', err)

      console.log('try to reconnect')
      this.initControlShell(adbId).catch(err => {
        console.error('reconnect error', err)
      })
    })
    ctrlCmd.stdout.on('data', line => { console.log(`command stdout: "${line as string}"`) })
    ctrlCmd.stderr.on('data', line => { console.error(`command stderr: "${line as string}"`) })

    this.controlShell = await ctrlCmd.spawn()
  }

  async close (): Promise<void> {
    if (this.controlShell !== null) {
      await this.controlShell.kill()
      this.controlShell = null
    }
    if (this.adbShell !== null) {
      await this.adbShell.kill()
      this.adbShell = null
    }
  }

  async execute (operation: RecordOperation): Promise<void> {
    if (this.controlShell === null) {
      throw new Error('controlShell is null')
    }
    if (this.adbShell === null) {
      throw new Error('adbShell is null')
    }

    this.executeQueue.push(operation)
    if (this.executeLock) {
      return
    }

    this.executeLock = true
    while (this.executeQueue.length > 0) {
      const operation = this.executeQueue.shift()
      if (operation === undefined) {
        continue
      }
      switch (operation.type) {
        case 'keyevent':
          await this.controlShell.write(`keycode ${operation.key} ${operation.keyEventType}\n`)
          break
        case 'am_start':
          await this.adbShell.write(`am start -n ${operation.packageName}\n`)
          break
        case 'tap':
          await this.controlShell.write(`tap ${operation.x} ${operation.y}\n`)
          break
        case 'motion':
          await this.controlShell.write(`touch ${operation.x} ${operation.y} ${operation.motionType}\n`)
          break
      }
    }
    this.executeLock = false
  }
}
