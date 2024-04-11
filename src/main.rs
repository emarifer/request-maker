mod application;
mod client;
#[rustfmt::skip]
mod config;
mod objects;
mod widgets;
mod window;

use self::application::RequestMakerApplication;
use self::config::GETTEXT_PACKAGE;
use gettextrs::LocaleCategory;
use gtk4::{gio, prelude::ApplicationExtManual};

fn main() -> glib::ExitCode {
    // Infer the location of DATADIR and PKGDATADIR from the executable location
    let exe = std::env::current_exe().expect("Cannot get current_exe() for app");
    let path = exe
        .parent()
        .and_then(|p| p.to_str())
        .expect("Cannot get current_exe() location");
    let locale_dir = format!("{}/../share/locale", path);
    let resource_file = format!("{}/../share/request-maker/request-maker.gresource", path);

    gettextrs::setlocale(LocaleCategory::LcAll, "");
    gettextrs::bindtextdomain(GETTEXT_PACKAGE, locale_dir).expect("Unable to bind the text domain");
    gettextrs::textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    let res = gio::Resource::load(resource_file).expect("Could not load gresource file");
    gio::resources_register(&res);

    let app = RequestMakerApplication::new();

    app.run()
}
