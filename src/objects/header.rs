use glib::Object;

mod imp {
    use std::cell::RefCell;

    use glib::{object::ObjectExt, subclass::object::ObjectImplExt, Properties};
    use gtk4::subclass::prelude::{DerivedObjectProperties, ObjectImpl, ObjectSubclass};

    // Object holding the state
    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::Header)]
    pub struct Header {
        #[property(get, set)]
        active: RefCell<bool>,

        #[property(get, set)]
        header_name: RefCell<String>,

        #[property(get, set)]
        header_value: RefCell<String>,
    }

    // The central trait for subclassing a GObject
    // No ParentType
    #[glib::object_subclass]
    impl ObjectSubclass for Header {
        const NAME: &'static str = "RequestMakerHeader";
        type Type = super::Header;
    }

    // Trait shared by all GObjects
    #[glib::derived_properties]
    impl ObjectImpl for Header {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }
}

// Since it is a pure GObject, it does not
// need to extend any class and/or interface
glib::wrapper! {
    pub struct Header(ObjectSubclass<imp::Header>);
}

impl Header {
    pub fn new(name: &str, value: &str) -> Self {
        Object::builder()
            .property("header-name", name)
            .property("header-value", value)
            .property("active", true)
            .build()
        // IMPORTANT!: properties keys cannot have an underscore (`_`)
        // as a standard Rust attribute; They can only contain normal
        // hyphens (`-`), because otherwise the framework
        // will not find these properties.

        /* Self {
            active: RefCell::new(true),
            header_name: RefCell::new(String::from(name)),
            header_value: RefCell::new(String::from(value)),
        } */
    }

    pub fn print(&self) {
        println!(
            "[{}] {}: {}",
            self.active(),
            self.header_name(),
            self.header_value()
        );
    }
}

impl Default for Header {
    fn default() -> Self {
        Object::builder().build()
    }
}

#[cfg(test)]
mod tests {

    use super::Header;

    #[test]
    pub fn test_this_works() {
        let header = Header::new("Accept", "text/html");
        /* assert_eq!(header.header_name.borrow().as_str(), "Accept");
        assert_eq!(header.header_value.borrow().as_str(), "text/html");
        assert!(header.active.borrow().to_owned()); */

        /* // *header.header_name.borrow_mut() = "Content-Type".to_owned(); // ↓ is the same ↓
        header.header_name.replace("Content-Type".to_owned());
        assert_ne!(header.header_name.borrow().as_str(), "Accept");
        assert_eq!(header.header_name.borrow().as_str(), "Content-Type"); */

        // to print during a test: `cargo test -- --nocapture`
        println!(
            "\nDefault value of the `active` property: {}\n",
            header.active()
        );

        assert_eq!(header.header_name(), "Accept");
        assert_eq!(header.header_value(), "text/html");
        assert!(!header.active());

        // modifying the value of the `header_name` property with a `setter`…
        header.set_header_name("Content-Type");
        assert_ne!(header.header_name(), "Accept");
        assert_eq!(header.header_name(), "Content-Type");
        // modifying the value of the `active` property with a `setter`…
        header.set_active(true);
        assert!(header.active());
    }
}
