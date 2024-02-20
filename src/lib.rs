use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

use std::{collections::HashMap, sync::Mutex};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Openurl;
#[cfg(mobile)]
use mobile::Openurl;

#[derive(Default)]
struct MyState(Mutex<HashMap<String, String>>);

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the openurl APIs.
pub trait OpenurlExt<R: Runtime> {
    fn openurl(&self) -> &Openurl<R>;
}

impl<R: Runtime, T: Manager<R>> crate::OpenurlExt<R> for T {
    fn openurl(&self) -> &Openurl<R> {
        self.state::<Openurl<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("openurl")
        .setup(|app, api| {
            #[cfg(mobile)]
            let openurl = mobile::init(app, api)?;
            #[cfg(desktop)]
            let openurl = desktop::init(app, api)?;
            app.manage(openurl);

            // manage state so it is accessible by the commands
            app.manage(MyState::default());
            Ok(())
        })
        .build()
}
