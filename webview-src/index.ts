import { invoke } from '@tauri-apps/api/core'

export async function execute() {
  await invoke('plugin:openurl|execute')
}

export async function openurl(url: string) {
  await invoke('plugin:openurl|openurl', { url })
}
