use glib::Object;
use gtk4::glib;

mod imp {
    use glib::subclass::{object::ObjectImpl, InitializingObject};
    use gtk4::prelude::{TextBufferExt, TextViewExt, WidgetExt};
    use gtk4::subclass::prelude::{BoxImpl, ObjectSubclass};
    use gtk4::subclass::widget::{
        CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetClassExt, WidgetImpl,
    };
    use gtk4::{Box, CompositeTemplate, Label, TemplateChild};
    use sourceview5::View;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/com/emarifer/request-maker/response_panel.ui")]
    pub struct ResponsePanel {
        #[template_child]
        pub response_body: TemplateChild<View>,

        #[template_child]
        pub response_meta: TemplateChild<Box>,

        #[template_child]
        pub status_code: TemplateChild<Label>,

        #[template_child]
        pub duration: TemplateChild<Label>,

        #[template_child]
        pub response_size: TemplateChild<Label>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for ResponsePanel {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "RequestMakerResponsePanel";
        type Type = super::ResponsePanel;
        type ParentType = Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ResponsePanel {
        fn constructed(&self) {
            self.response_body
                .buffer()
                .set_text("<xml>Under construction</xml>");

            self.response_meta.set_visible(true);
            self.status_code.set_label("HTTP 200");
            self.duration.set_label("2 s");
            self.response_size.set_label("400 B");
        }
    }

    impl WidgetImpl for ResponsePanel {}

    impl BoxImpl for ResponsePanel {}
}

glib::wrapper! {
    pub struct ResponsePanel(ObjectSubclass<imp::ResponsePanel>)
        @extends gtk4::Widget, gtk4::Box,
        @implements gtk4::Accessible, gtk4::Buildable, gtk4::ConstraintTarget, gtk4::Orientable;
}

impl Default for ResponsePanel {
    fn default() -> Self {
        Self::new()
    }
}

impl ResponsePanel {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
