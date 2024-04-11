use glib::{object::Cast, subclass::types::ObjectSubclassIsExt};
use gtk4::gio;

use crate::objects::Header;

mod imp {
    use std::borrow::BorrowMut;

    use glib::{
        closure_local,
        object::{Cast, CastNone, ObjectExt},
        subclass::{
            object::{ObjectImpl, ObjectImplExt},
            types::ObjectSubclass,
            InitializingObject,
        },
    };
    use gtk4::{
        gio,
        prelude::ListItemExt,
        subclass::{
            box_::BoxImpl,
            widget::{
                CompositeTemplateCallbacksClass, CompositeTemplateClass,
                CompositeTemplateInitializingExt, WidgetClassExt, WidgetImpl,
            },
        },
        CompositeTemplate, TemplateChild,
    };

    use crate::{objects::Header, widgets::RequestHeaderRow};

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/com/emarifer/request-maker/request_header_pane.ui")]
    pub struct RequestHeaderPane {
        #[template_child]
        list_view: TemplateChild<gtk4::ListView>,
        #[template_child]
        pub selection_model: TemplateChild<gtk4::NoSelection>,
        #[template_child]
        add_new: TemplateChild<gtk4::Button>,
    }

    #[gtk4::template_callbacks]
    impl RequestHeaderPane {
        #[template_callback]
        fn on_add_new_header(&self) {
            let empty_header = Header::default();
            empty_header.set_active(true);

            let store = self
                .selection_model
                .model()
                .and_downcast::<gio::ListStore>()
                .unwrap();

            store.append(&empty_header);
        }

        fn on_remove_row(&self, idx: u32) {
            if let Some(ref mut model) = self.selection_model.model().borrow_mut() {
                let store = model.clone().downcast::<gio::ListStore>().unwrap();
                store.remove(idx);
            };
        }
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for RequestHeaderPane {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "RequestMakerRequestHeaderPane";
        type Type = super::RequestHeaderPane;
        type ParentType = gtk4::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for RequestHeaderPane {
        fn constructed(&self) {
            self.parent_constructed();

            let store = gio::ListStore::new::<Header>();
            self.selection_model.set_model(Some(&store));

            /* Build the factory used to link the headers and the widgets. */
            let factory = gtk4::SignalListItemFactory::new();
            self.list_view.set_factory(Some(&factory));

            /* Called whenever the system wants a new empty widget. */
            factory.connect_setup(|_, item: &gtk4::ListItem| {
                let row = RequestHeaderRow::default();
                item.set_child(Some(&row));
            });

            /* Called whenever the system wants to stop using a widget. */
            factory.connect_teardown(|_, item: &gtk4::ListItem| {
                item.set_child(Option::<&gtk4::Widget>::None);
            });

            /* Called whenever the system will place a header in a widget. */
            factory.connect_bind(
                glib::clone!(@weak self as pane => move |_, item: &gtk4::ListItem| {
                    let widget = item.child().and_downcast::<RequestHeaderRow>().unwrap();
                    let header = item.item().and_downcast::<Header>().unwrap();

                    /* Add the initial data to the header. */
                    widget.set_header_name(header.header_name());
                    widget.set_header_value(header.header_value());
                    widget.set_active(header.active());

                    /* Create some binds to put the data back in this header. */
                    let bind_name = widget.bind_property("header-name", &header, "header-name").bidirectional().sync_create().build();
                    let bind_value = widget.bind_property("header-value", &header, "header-value").bidirectional().sync_create().build();
                    let bind_active = widget.bind_property("active", &header, "active").bidirectional().sync_create().build();
                    widget.set_bindings(bind_name, bind_value, bind_active);

                    let pos = item.position();
                    let delete_closure = widget.connect_closure("delete", false, closure_local!(@strong pane => move |_row: RequestHeaderRow| {
                        pane.on_remove_row(pos);

                    }));
                    widget.set_delete_closure(delete_closure);
                }),
            );

            /* Called whenever the system will stop using a header in a widget. */
            factory.connect_unbind(|_, item: &gtk4::ListItem| {
                let widget = item.child().and_downcast::<RequestHeaderRow>().unwrap();

                /* Disconnect the binds stored in the header. */
                widget.reset_bindings();

                /* Remove data from this widget to make it clean. */
                widget.set_header_name("");
                widget.set_header_value("");
                widget.set_active(false);

                /* Disconnect the closure. */
                if let Some(closure_id) = widget.delete_closure() {
                    widget.disconnect(closure_id);
                };
            });
        }
    }

    impl WidgetImpl for RequestHeaderPane {}

    impl BoxImpl for RequestHeaderPane {}
}

glib::wrapper! {
    pub struct RequestHeaderPane(ObjectSubclass<imp::RequestHeaderPane>)
        @extends gtk4::Widget, gtk4::Box,
        @implements gtk4::Accessible, gtk4::Buildable, gtk4::ConstraintTarget,
                gtk4::Orientable;
}

impl RequestHeaderPane {
    pub fn get_headers(&self) -> Vec<Header> {
        let imp = self.imp();
        let model = imp.selection_model.model().expect("Where is my ListModel?");
        let list_store = model.downcast::<gio::ListStore>().unwrap().clone();

        // TODO: Please use an iter, filters and maps.
        let mut v = Vec::new();
        for item in &list_store {
            if let Ok(header) = item {
                v.push(header.downcast::<Header>().expect("My header?"));
            }
        }

        v
    }
}
