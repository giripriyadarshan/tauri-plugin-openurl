import { invoke } from '@tauri-apps/api/core'

export async function open_url(url: string) {
  try {
    await invoke('plugin:openurl|open_url', { url })
  }
  catch (e) {
    console.error(e)
  }
}