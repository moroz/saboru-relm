mod imp {
    use glib::Properties;
    use gtk::glib;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use std::cell::RefCell;

    use super::SidebarItemData;

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::DataRow)]
    pub struct DataRow {
        #[property(name = "id", get, set, type = i64, member = id)]
        #[property(name = "label", get, set, type = String, member = label)]
        pub data: RefCell<SidebarItemData>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DataRow {
        const NAME: &'static str = "DataRow";
        type Type = super::DataRow;
    }

    #[glib::derived_properties]
    impl ObjectImpl for DataRow {}
}

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct DataRow(ObjectSubclass<imp::DataRow>);
}

impl DataRow {
    pub fn new(id: i64, label: impl Into<String>) -> Self {
        Object::builder()
            .property("label", label.into())
            .property("id", id)
            .build()
    }
}

#[derive(Default, Clone, Debug)]
pub struct SidebarItemData {
    pub label: String,
    pub id: i64,
}
