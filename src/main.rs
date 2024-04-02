mod application;
mod components;
mod window;

use application::RequestMakerApplication;
use gtk4::prelude::ApplicationExtManual;
// pub use window::RequestMakerWindow;

fn main() -> glib::ExitCode {
    let app = RequestMakerApplication::new();

    app.run()
}
