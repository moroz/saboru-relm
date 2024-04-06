use crate::AppMsg;
use gtk::gio;
use gtk::glib::Object;
use gtk::prelude::*;
use relm4::ComponentParts;
use relm4::SimpleComponent;

use self::row::SidebarRow;

pub mod row;

#[derive(Debug)]
pub struct SidebarModel {
    channels_model: gtk::SingleSelection,
}

#[derive(Debug)]
pub enum SidebarMsg {}

#[relm4::component(pub)]
impl SimpleComponent for SidebarModel {
    type Init = ();
    type Input = SidebarMsg;
    type Output = AppMsg;

    view! {
        gtk::ScrolledWindow {
            set_hscrollbar_policy: gtk::PolicyType::Never,
            set_vexpand: false,
            set_width_request: 300,

            #[name = "channels"]
            gtk::ListView {}
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: relm4::prelude::ComponentSender<Self>,
    ) -> relm4::prelude::ComponentParts<Self> {
        let store = gio::ListStore::new::<SidebarRow>();

        let initial_channels = &[("Alice", 1), ("Bob", 2)];

        for (name, id) in initial_channels {
            store.append(&SidebarRow::new(name, *id));
        }

        let model = SidebarModel {
            channels_model: gtk::SingleSelection::builder()
                .model(&store)
                .autoselect(true)
                .build(),
        };
        let widgets = view_output!();

        let factory = gtk::SignalListItemFactory::new();
        factory.connect_setup(move |_, item| {
            let root = gtk::Box::builder()
                .orientation(gtk::Orientation::Horizontal)
                .spacing(5)
                .build();
            let name_label = gtk::Label::builder().margin_bottom(5).margin_top(5).build();
            root.append(&name_label);

            item.downcast_ref::<gtk::ListItem>()
                .unwrap()
                .set_child(Some(&root));
        });
        factory.connect_bind(move |_, item| {
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            let sidebar_item = item.item().and_downcast::<SidebarRow>().unwrap();
            let child = item
                .child()
                .and_downcast::<gtk::Box>()
                .unwrap()
                .first_child()
                .and_downcast::<gtk::Label>()
                .unwrap();
            child.set_label(&sidebar_item.property::<String>("name"));
        });

        widgets.channels.set_factory(Some(&factory));
        widgets.channels.set_model(Some(&model.channels_model));

        ComponentParts { model, widgets }
    }
}
