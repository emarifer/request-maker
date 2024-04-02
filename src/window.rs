use glib::{object::Cast, subclass::types::ObjectSubclassIsExt, GString, Object};
use gtk4::{
    gio, glib,
    prelude::{EditableExt, TextBufferExt, TextViewExt},
    ListBox, StringObject,
};

use std::collections::HashMap;

use crate::{application::RequestMakerApplication, components::rowheader::RowHeader};

fn mock_map() -> HashMap<String, String> {
    let mut map = HashMap::new();

    map.insert(String::from("Accept"), String::from("text/html"));
    map.insert(
        String::from("User-Agent"),
        String::from("request-maker/1.0"),
    );
    map.insert(String::from("Accept-Encoding"), String::from("bzig"));

    map
}

fn populate_list(list_box: &ListBox, map: &HashMap<String, String>) {
    for (name, value) in map.iter() {
        // let entry_box = create_row(&name, &value);
        let rowheader = RowHeader::new(&name, &value);
        list_box.append(&rowheader);
    }
}

mod imp {
    use glib::subclass::{
        object::{ObjectImpl, ObjectImplExt},
        types::{ObjectSubclass, ObjectSubclassExt},
        InitializingObject,
    };
    use gtk4::{
        subclass::{application_window::ApplicationWindowImpl, widget::*, window::WindowImpl},
        template_callbacks, ApplicationWindow, Button, CompositeTemplate, DropDown, Entry, ListBox,
        TemplateChild,
    };

    use super::{mock_map, populate_list};

    #[derive(Default, CompositeTemplate)]
    #[template(file = "../data/ui/prototype.ui")]
    pub struct RequestMakerWindow {
        #[template_child(id = "method")]
        pub request_method: TemplateChild<DropDown>,

        #[template_child(id = "url")]
        pub request_url: TemplateChild<Entry>,

        #[template_child(id = "send")]
        pub send_button: TemplateChild<Button>,

        #[template_child]
        pub request_headers: TemplateChild<ListBox>,

        #[template_child]
        pub request_body: TemplateChild<sourceview5::View>,
    }

    #[template_callbacks]
    impl RequestMakerWindow {
        #[template_callback]
        fn on_request_send(&self /* , _: &Button */) {
            let obj = &self.obj();
            let method = obj.request_method();
            let url = obj.request_url();
            let body = obj.request_body();

            println!("{} {}", method, url);
            println!("{}", body)
        }
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for RequestMakerWindow {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "RequestMakerWindow";
        type Type = super::RequestMakerWindow;
        type ParentType = ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for RequestMakerWindow {
        fn constructed(&self) {
            self.parent_constructed();

            let fake_headers = mock_map();
            populate_list(&self.request_headers, &fake_headers)
        }
    }

    impl WidgetImpl for RequestMakerWindow {}

    impl WindowImpl for RequestMakerWindow {}

    impl ApplicationWindowImpl for RequestMakerWindow {}
}

glib::wrapper! {
    pub struct RequestMakerWindow(ObjectSubclass<imp::RequestMakerWindow>)@extends gtk4::Widget, gtk4::Window, gtk4::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl RequestMakerWindow {
    pub fn new(app: &RequestMakerApplication) -> Self {
        Object::builder().property("application", Some(app)).build()
    }

    pub fn request_url(&self) -> GString {
        self.imp().request_url.text()
    }

    pub fn request_method(&self) -> GString {
        let method = &self.imp().request_method;

        method
            .selected_item()
            .unwrap()
            .downcast::<StringObject>()
            .unwrap()
            .string()
    }

    pub fn request_body(&self) -> GString {
        let body = &self.imp().request_body;

        let (start, end) = body.buffer().bounds();
        body.buffer().text(&start, &end, true)
    }
}