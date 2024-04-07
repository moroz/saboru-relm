mod imp {
    use gtk::{glib, subclass::prelude::*};

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(file = "message_row.ui")]
    pub struct MessageRow {
        #[template_child]
        pub body: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MessageRow {
        const NAME: &'static str = "MessageRow";
        type Type = super::MessageRow;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MessageRow {}
    impl WidgetImpl for MessageRow {}
    impl BoxImpl for MessageRow {}
}

use gtk::{glib, subclass::prelude::*};

glib::wrapper! {
    pub struct MessageRow(ObjectSubclass<imp::MessageRow>)
    @extends gtk::Widget, gtk::Box;
}

impl Default for MessageRow {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl MessageRow {
    pub fn set_content(&self, body: String) {
        self.imp().body.set_text(&body);
    }
}
