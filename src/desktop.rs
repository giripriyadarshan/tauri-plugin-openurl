use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};
use std::process::Command;

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
    pub fn open_url(&self, url: String) -> crate::Result<()> {
        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "start", url.as_str()])
                .spawn().unwrap();
        } else if cfg!(target_os = "macos") {
            Command::new("open")
                .arg(url)
                .spawn().unwrap();
        } else if cfg!(target_os = "linux") {
            Command::new("xdg-open")
                .arg(url)
                .spawn().unwrap();
        }
        Ok(())
    }
}
