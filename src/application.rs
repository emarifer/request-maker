use glib::Object;
use gtk4::gio;

use crate::config::APP_ID;
use crate::window::RequestMakerWindow;

mod imp {
    use glib::subclass::{object::ObjectImpl, types::ObjectSubclass};
    use gtk4::{
        prelude::GtkWindowExt,
        subclass::{application::GtkApplicationImpl, prelude::*},
        Application, Window,
    };

    use super::*;

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
            self.parent_activate();
            self.obj().get_windodw().present();
        }

        fn startup(&self) {
            self.parent_startup();
            Window::set_default_icon_name(APP_ID);
        }
    }

    impl GtkApplicationImpl for RequestMakerApplication {}
}

glib::wrapper! {
    pub struct RequestMakerApplication(ObjectSubclass<imp::RequestMakerApplication>)
        @extends gio::Application, gtk4::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl Default for RequestMakerApplication {
    fn default() -> Self {
        Self::new()
    }
}

impl RequestMakerApplication {
    pub fn new() -> Self {
        Object::builder().property("application-id", APP_ID).build()
    }

    pub fn get_windodw(&self) -> RequestMakerWindow {
        RequestMakerWindow::new(self)
    }
}
