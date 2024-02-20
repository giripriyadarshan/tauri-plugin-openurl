import { invoke } from '@tauri-apps/api/core'

export async function openUrl(url: string) {
  await invoke('plugin:openurl|openUrl', { url })
}