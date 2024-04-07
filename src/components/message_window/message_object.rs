mod imp {
    use glib::Properties;
    use gtk::glib;
    use gtk::glib::subclass::types::ObjectSubclass;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use std::cell::RefCell;

    use super::MessageDataRow;

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::MessageObject)]
    pub struct MessageRow {
        #[property(name = "id", get, set, type = i64, member = id)]
        #[property(name = "body", get, set, type = String, member = body)]
        pub data: RefCell<MessageDataRow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MessageRow {
        const NAME: &'static str = "MessageRow";
        type Type = super::MessageObject;
    }

    #[glib::derived_properties]
    impl ObjectImpl for MessageRow {}
}

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct MessageObject(ObjectSubclass<imp::MessageRow>);
}

impl MessageObject {
    pub fn new(id: i64, body: String) -> Self {
        Object::builder()
            .property("id", id)
            .property("body", body)
            .build()
    }
}

#[derive(Default, Clone, Debug)]
pub struct MessageDataRow {
    pub id: i64,
    pub body: String,
}
