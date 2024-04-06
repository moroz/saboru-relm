use glib::Object;
use gtk::subclass::prelude::*;
use relm4::gtk::{gio, glib};

#[derive(Default, Debug)]
pub struct SidebarRowData {
    pub name: String,
    pub id: u64,
}

glib::wrapper! {
    pub struct SidebarRow(ObjectSubclass<imp::SidebarRow>);
}

impl SidebarRow {
    pub fn new(name: &str, id: u64) -> Self {
        Object::builder()
            .property("name", name)
            .property("id", id)
            .build()
    }
}

mod imp {
    use std::cell::RefCell;

    use gtk::glib::subclass::types::ObjectSubclass;
    use gtk::glib::Properties;
    use relm4::gtk::prelude::*;
    use relm4::gtk::subclass::prelude::*;
    use relm4::gtk::{self, glib};

    #[derive(Default, Debug, Properties)]
    #[properties(wrapper_type = super::SidebarRow)]
    pub struct SidebarRow {
        #[property(name = "id", get, set, type = u64, member = id)]
        #[property(name = "name", get, set, type = String, member = name)]
        pub data: RefCell<super::SidebarRowData>,
    }

    #[glib::derived_properties]
    impl ObjectImpl for SidebarRow {}

    #[glib::object_subclass]
    impl ObjectSubclass for SidebarRow {
        const NAME: &'static str = "SidebarRow";
        type Type = super::SidebarRow;
        type ParentType = glib::Object;
    }
}
