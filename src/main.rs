mod application;
mod components;
#[rustfmt::skip]
mod config;
mod window;

use gettextrs::LocaleCategory;
use gtk4::{gio, prelude::ApplicationExtManual};
// pub use window::RequestMakerWindow;
use self::application::RequestMakerApplication;
use self::config::{GETTEXT_PACKAGE, LOCALEDIR, RESOURCES_FILE};

fn main() -> glib::ExitCode {
    gettextrs::setlocale(LocaleCategory::LcAll, "");
    gettextrs::bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    gettextrs::textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    let res = gio::Resource::load(RESOURCES_FILE).expect("Could not load gresource file");
    gio::resources_register(&res);

    let app = RequestMakerApplication::new();

    app.run()
}
