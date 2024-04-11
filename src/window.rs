use glib::{object::Cast, subclass::types::ObjectSubclassIsExt, GString, Object};
use gtk4::{
    gio, glib,
    prelude::{EditableExt, TextBufferExt, TextViewExt},
    StringObject,
};

use crate::application::RequestMakerApplication;

mod imp {
    use glib::{
        // object::Cast,
        subclass::{
            object::{ObjectImpl, ObjectImplExt},
            types::ObjectSubclass,
            InitializingObject,
        },
        types::StaticType,
    };
    /* use gtk4::{
        gio::{ListModel, ListStore},
        // prelude::ButtonExt,
        ListView,
        NoSelection,
    }; */
    use gtk4::{
        self,
        subclass::{application_window::ApplicationWindowImpl, widget::*, window::WindowImpl},
        template_callbacks, ApplicationWindow, Button, CompositeTemplate, DropDown, Entry,
        TemplateChild,
    };
    // use isahc::RequestExt;

    use crate::{
        // config::VERSION,
        // objects::Header,
        // client::{build_request, Request, RequestMethod},
        widgets::*,
    };

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/com/emarifer/request-maker/main_window.ui")]
    pub struct RequestMakerWindow {
        #[template_child(id = "method")]
        pub request_method: TemplateChild<DropDown>,

        #[template_child(id = "url")]
        pub request_url: TemplateChild<Entry>,

        #[template_child(id = "send")]
        pub send_button: TemplateChild<Button>,

        #[template_child]
        pub header_pane: TemplateChild<RequestHeaderPane>,

        #[template_child]
        pub request_body: TemplateChild<sourceview5::View>,

        #[template_child]
        pub response: TemplateChild<ResponsePanel>,
    }

    #[template_callbacks]
    impl RequestMakerWindow {
        #[template_callback]
        fn on_send_request(&self /* , _: &Button */) {
            /* let obj = &self.obj();
            let url = obj.request_url();
            let body = obj.request_body();
            let method = {
                let str = String::from(obj.request_method());
                RequestMethod::try_from(str.as_str())
            };

            let request = match method {
                Ok(method) => {
                    let request = Request {
                        url: String::from(url),
                        method,
                        body: String::from(body),
                        headers: HashMap::new(),
                    };

                    // println!("{:#?}", request);
                    build_request(&request)
                }
                Err(_) => {
                    println!("Error: invalid method");
                    return;
                }
            }; */

            /* let response = match request {
                Ok(req) => req.send(),
                Err(_) => {
                    println!("Error: invalid request");
                    return;
                }
            };

            match response {
                Err(_) => {
                    println!("Error: invalid response");
                }
                Ok(mut rsp) => {
                    let mut body_content = String::new();
                    let _ = rsp.body_mut().read_to_string(&mut body_content);
                    println!("{:#?}", rsp);
                    println!("{}", body_content);
                }
            } */

            /* let headers = self.get_headers();
            for h in headers {
                println!("{:?}", h); // what it prints are tuples
            } */
            let headers = self.header_pane.get_headers();
            for header in headers {
                header.print();
            }
        }

        /* fn get_headers(&self) -> Vec<(String, String)> {
            let mut headers = Vec::new();
            if let Some(model) = self.request_headers.model() {
                let no_selection = model.downcast::<NoSelection>().unwrap();
                let list_model = no_selection.model().unwrap();
                for item in &list_model {
                    if let Ok(thing) = item {
                        let header = thing.downcast::<Header>().unwrap();
                        let value = (header.header_name(), header.header_value());
                        headers.push(value);
                    }
                }
            }
            headers
        }

        fn add_header(&self, h: &Header) {
            if let Some(model) = self.request_headers.model() {
                let no_selection = model.downcast::<NoSelection>().unwrap();
                let list_model = no_selection
                    .model()
                    .unwrap()
                    .downcast::<ListStore>()
                    .unwrap();
                list_model.append(h);
            }
        } */
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for RequestMakerWindow {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "RequestMakerWindow";
        type Type = super::RequestMakerWindow;
        type ParentType = ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            // RequestHeaderRow::static_type();
            RequestHeaderPane::static_type();
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

            /* let model = ListStore::new::<Header>();
            let selection_model = gtk4::NoSelection::new(Some(model.upcast::<ListModel>()));
            self.request_headers.set_model(Some(&selection_model));

            self.add_header(&Header::new("Accept", "text/html"));
            let h = Header::new("Content-Type", "text/html");
            h.set_active(false);
            self.add_header(&h);
            self.add_header(&Header::new("Authorization", "Bearer roar")); */
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
