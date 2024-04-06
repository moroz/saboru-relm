use glib::Object;
use gtk::subclass::prelude::*;
use relm4::gtk::{gio, glib};

glib::wrapper! {
    pub struct SidebarRow(ObjectSubclass<imp::SidebarRow>);
}

impl SidebarRow {
    pub fn new(name: &str, id: u64) -> Self {
        Object::builder().build()
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
        #[property(get, set)]
        pub name: RefCell<String>,
        #[property(get, set)]
        pub id: RefCell<u64>,
    }

    impl ObjectImpl for SidebarRow {}

    #[glib::object_subclass]
    impl ObjectSubclass for SidebarRow {
        const NAME: &'static str = "SidebarRow";
        type Type = super::SidebarRow;
        type ParentType = glib::Object;
    }
}
