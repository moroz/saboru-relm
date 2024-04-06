mod data_row;
pub use data_row::DataRow;

mod imp {
    use gtk::{glib, subclass::prelude::*};

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(file = "sidebar_row.ui")]
    pub struct SidebarRow {
        #[template_child]
        pub name: TemplateChild<gtk::Label>,
        #[template_child]
        pub avatar: TemplateChild<gtk::Image>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SidebarRow {
        const NAME: &'static str = "SidebarRow";
        type Type = super::SidebarRow;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for SidebarRow {}
    impl WidgetImpl for SidebarRow {}
    impl BoxImpl for SidebarRow {}
}

use gtk::{glib, subclass::prelude::*};

glib::wrapper! {
    pub struct SidebarRow(ObjectSubclass<imp::SidebarRow>)
    @extends gtk::Widget, gtk::Box;
}

impl Default for SidebarRow {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl SidebarRow {
    pub fn set_content(&self, label: String) {
        let imp = self.imp();
        imp.name.set_text(&label);
    }
}
