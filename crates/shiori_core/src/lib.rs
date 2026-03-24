mod app;
pub mod db;

pub use app::App;

pub struct ShioriCore {
    app: App,
}

impl ShioriCore {
    pub fn new() -> ShioriCore {
        ShioriCore { app: App::new() }
    }

    pub fn get_app(&self) -> App {
        self.app.clone()
    }
}
