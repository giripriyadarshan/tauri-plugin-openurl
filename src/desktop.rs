use serde::de::DeserializeOwned;
use std::process::Command;
use tauri::{command, plugin::PluginApi, AppHandle, Runtime, Window};

// use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Openurl<R>> {
    Ok(Openurl(app.clone()))
}

/// Access to the openurl APIs.
pub struct Openurl<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Openurl<R> {
    pub fn open_url(&self, _url: String) -> crate::Result<()> {
        Ok(())
    }
}

#[command]
pub async fn open_url<R: Runtime>(_app: AppHandle<R>, _window: Window<R>, url: String) {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "start", url.as_str()])
            .spawn()
            .unwrap();
    } else if cfg!(target_os = "macos") {
        Command::new("open").arg(url).spawn().unwrap();
    } else {
        Command::new("xdg-open").arg(url).spawn().unwrap();
    }
}
