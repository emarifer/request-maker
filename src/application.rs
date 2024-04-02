use glib::Object;
use gtk4::gio;

use crate::window::RequestMakerWindow;

mod imp {
    use glib::subclass::{object::ObjectImpl, types::ObjectSubclass};
    use gtk4::{
        prelude::GtkWindowExt,
        subclass::{
            application::GtkApplicationImpl,
            prelude::{ApplicationImpl, ObjectSubclassExt},
        },
        Application,
    };

    #[derive(Default)]
    pub struct RequestMakerApplication;

    #[glib::object_subclass]
    impl ObjectSubclass for RequestMakerApplication {
        const NAME: &'static str = "RequestMakerApplication";
        type Type = super::RequestMakerApplication;
        type ParentType = Application;
    }

    impl ObjectImpl for RequestMakerApplication {}

    impl ApplicationImpl for RequestMakerApplication {
        fn activate(&self) {
            self.obj().get_windodw().present();
        }
    }

    impl GtkApplicationImpl for RequestMakerApplication {}
}

glib::wrapper! {
    pub struct RequestMakerApplication(ObjectSubclass<imp::RequestMakerApplication>)
        @extends gio::Application, gtk4::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl RequestMakerApplication {
    pub fn new() -> Self {
        Object::builder()
            .property("application-id", "com.emarifer.request-maker")
            .build()
    }

    pub fn get_windodw(&self) -> RequestMakerWindow {
        RequestMakerWindow::new(&self)
    }
}
