use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

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
struct MyState(());

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the openurl APIs.
pub trait OpenurlExt<R: Runtime> {
    fn open_url(&self) -> &Openurl<R>;
}

impl<R: Runtime, T: Manager<R>> crate::OpenurlExt<R> for T {
    fn open_url(&self) -> &Openurl<R> {
        self.state::<Openurl<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    let builder = Builder::new("openurl").setup(|app, api| {
        #[cfg(mobile)]
        let open_url = mobile::init(app, api)?;
        #[cfg(desktop)]
        let open_url = desktop::init(app, api)?;
        app.manage(open_url);

        // manage state so it is accessible by the commands
        app.manage(MyState::default());
        Ok(())
    });

    if cfg!(mobile) {
        builder.build()
    } else {
        builder
            .invoke_handler(tauri::generate_handler![desktop::open_url])
            .build()
    }
}
