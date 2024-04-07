use relm4::{ComponentParts, SimpleComponent};
pub mod message_object;
pub mod message_row;

use crate::AppMsg;
use message_row::MessageRow;

use self::message_object::MessageObject;

use super::sidebar_item::Channel;

#[derive(Debug)]
pub struct MessageWindowModel {
    messages_model: gtk::NoSelection,
    active_channel: Option<Channel>,
}

use gtk::gio;
use gtk::prelude::*;

#[relm4::component(pub)]
impl SimpleComponent for MessageWindowModel {
    type Init = ();
    type Input = AppMsg;
    type Output = AppMsg;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 10,
            set_css_classes: &["message-window"],
            set_height_request: 768 - 300,

            gtk::Label {
                set_label: "Messages",
                set_margin_top: 10,
                set_margin_start: 10,
                set_margin_end: 10,
                set_css_classes: &["header"],
                set_halign: gtk::Align::Start,
            },

            gtk::ScrolledWindow {
                set_hscrollbar_policy: gtk::PolicyType::Never,
                set_vexpand: false,
                set_hexpand: true,

                #[name = "messages"]
                gtk::ListView {}
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: relm4::prelude::ComponentSender<Self>,
    ) -> relm4::prelude::ComponentParts<Self> {
        let store = gio::ListStore::new::<MessageObject>();

        let model = MessageWindowModel {
            messages_model: gtk::NoSelection::new(Some(store)),
            active_channel: None,
        };

        let widgets = view_output!();

        let factory = gtk::SignalListItemFactory::new();
        factory.connect_setup(move |_, item| {
            let component = MessageRow::default();
            item.downcast_ref::<gtk::ListItem>()
                .unwrap()
                .set_child(Some(&component));
        });

        factory.connect_bind(move |_, item| {
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            let data_row = item.item().and_downcast::<MessageObject>().unwrap();
            let child = item.child().and_downcast::<MessageRow>().unwrap();
            child.set_content(data_row.property::<String>("body"));
        });

        widgets.messages.set_factory(Some(&factory));
        widgets.messages.set_model(Some(&model.messages_model));

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: relm4::prelude::ComponentSender<Self>) {
        println!("{:?}", message);
    }
}
