use gtk4::{prelude::*, Application, ApplicationWindow};
use sourceview5;

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("com.emarifer.not-postman")
        .build();

    app.connect_activate(|app| {
        sourceview5::init();
        let ui_src = include_str!("../ui/prototype.ui");
        let builder = gtk4::Builder::from_string(ui_src);

        /* let window = ApplicationWindow::builder()
        .application(app)
        .default_width(800)
        .default_height(600)
        .title("Not Postman")
        .build() */

        let window = builder
            .object::<ApplicationWindow>("win")
            .expect("Couldn't get window");

        window.set_application(Some(app));

        window.present()
    });

    app.run()
}
