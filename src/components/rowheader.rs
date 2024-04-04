use glib::{subclass::InitializingObject, GString};
use gtk4::{prelude::*, subclass::prelude::*, Box, CompositeTemplate, Entry};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/emarifer/request-maker/http_header_row.ui")]
pub struct RowHeaderImpl {
    #[template_child]
    pub entry_key: TemplateChild<Entry>,
    #[template_child]
    pub entry_value: TemplateChild<Entry>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for RowHeaderImpl {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "RowHeader";
    type Type = RowHeader;
    type ParentType = Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for RowHeaderImpl {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for RowHeaderImpl {}

impl BoxImpl for RowHeaderImpl {}

glib::wrapper! {
    pub struct RowHeader(ObjectSubclass<RowHeaderImpl>)
        @extends gtk4::Widget, gtk4::Box,
        @implements gtk4::Accessible, gtk4::Buildable, gtk4::ConstraintTarget,
                    gtk4::Actionable, gtk4::ActionBar, gtk4::ATContext;

}

impl RowHeader {
    pub fn new(key: &str, value: &str) -> Self {
        let obj: Self = glib::Object::builder().build();
        obj.imp().entry_key.set_text(key);
        obj.imp().entry_value.set_text(value);

        obj
    }

    pub fn get_key(&self) -> GString {
        self.imp().entry_key.text()
    }

    pub fn get_value(&self) -> GString {
        self.imp().entry_value.text()
    }
}
