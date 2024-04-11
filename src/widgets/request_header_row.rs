use glib::{property::PropertySet, subclass::types::ObjectSubclassIsExt, Binding, SignalHandlerId};
use gtk4::glib::{self, Object};

mod imp {
    use std::{cell::RefCell, sync::OnceLock};

    use glib::{
        subclass::{InitializingObject, Signal},
        Properties, SignalHandlerId,
    };
    use gtk4::{prelude::*, subclass::prelude::*, Box, CompositeTemplate, Entry};

    use super::HeaderRowBindings;

    // use crate::objects::Header;

    #[derive(CompositeTemplate, Default, Properties)]
    #[properties(wrapper_type = super::RequestHeaderRow)]
    #[template(resource = "/com/emarifer/request-maker/request_header_row.ui")]
    pub struct RequestHeaderRow {
        #[property(get, set)]
        active: RefCell<bool>,
        #[property(get, set)]
        header_name: RefCell<String>,
        #[property(get, set)]
        header_value: RefCell<String>,

        #[template_child]
        pub entry_key: TemplateChild<Entry>,
        #[template_child]
        pub entry_value: TemplateChild<Entry>,

        pub bindings: RefCell<Option<HeaderRowBindings>>,

        pub delete_signals: RefCell<Option<SignalHandlerId>>,
    }

    #[gtk4::template_callbacks]
    impl RequestHeaderRow {
        #[template_callback]
        fn on_delete_request(&self) {
            let obj = self.obj();
            obj.emit_by_name::<()>("delete", &[]);
        }
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for RequestHeaderRow {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "RequestMakerRequestHeaderRow";
        type Type = super::RequestHeaderRow;
        type ParentType = Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for RequestHeaderRow {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| vec![Signal::builder("delete").build()])
        }
    }

    impl WidgetImpl for RequestHeaderRow {}

    impl BoxImpl for RequestHeaderRow {}
}

glib::wrapper! {
    pub struct RequestHeaderRow(ObjectSubclass<imp::RequestHeaderRow>)
        @extends gtk4::Widget, gtk4::Box,
        @implements gtk4::Accessible, gtk4::Buildable, gtk4::ConstraintTarget,
                    gtk4::Actionable, gtk4::ActionBar, gtk4::ATContext;

}

impl Default for RequestHeaderRow {
    fn default() -> Self {
        Object::builder().build()
    }
}

impl RequestHeaderRow {
    pub fn set_bindings(&self, header_name: Binding, header_value: Binding, active: Binding) {
        let imp = self.imp();

        let binds = HeaderRowBindings::new(header_name, header_value, active);
        imp.bindings.set(Some(binds));
    }

    pub fn reset_bindings(&self) {
        let imp = self.imp();

        {
            let bindings = imp.bindings.borrow_mut();
            if let Some(binds) = &*bindings {
                binds.unbind();
            };
        }

        imp.bindings.set(None);
    }

    pub fn set_delete_closure(&self, hnd: SignalHandlerId) {
        let imp = self.imp();
        imp.delete_signals.set(Some(hnd));
    }

    pub fn delete_closure(&self) -> Option<SignalHandlerId> {
        let imp = self.imp();
        imp.delete_signals.take()
    }
}

/// Represents the bindings used by the HeaderRow component, which are required to
/// persist so that I can unbind them later when the component is being cleaned
/// to be reused with a different header later.
pub struct HeaderRowBindings {
    header_name: Binding,
    header_value: Binding,
    active: Binding,
}

impl HeaderRowBindings {
    pub fn new(header_name: Binding, header_value: Binding, active: Binding) -> Self {
        Self {
            header_name,
            header_value,
            active,
        }
    }

    pub fn unbind(&self) {
        self.header_name.unbind();
        self.header_value.unbind();
        self.active.unbind();
    }
}
