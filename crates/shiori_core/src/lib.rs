mod app;
pub mod db;

pub use app::App;

pub struct ShioriCore {
    app: App,
}

impl ShioriCore {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ShioriCore {
        ShioriCore { app: App::new() }
    }

    pub fn get_app(&self) -> App {
        self.app.clone()
    }
}
