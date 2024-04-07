use relm4::factory::FactoryVecDeque;
use relm4::{ComponentParts, SimpleComponent};
pub mod message_row;

use crate::AppMsg;
use message_row::MessageRow;

use super::sidebar_item::Channel;

#[derive(Debug)]
pub struct MessageWindowModel {
    messages: FactoryVecDeque<MessageRow>,
    active_channel: Option<Channel>,
}

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

                #[local_ref]
                messages_box -> gtk::Box {}
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: relm4::prelude::ComponentSender<Self>,
    ) -> relm4::prelude::ComponentParts<Self> {
        let messages = FactoryVecDeque::builder()
            .launch(gtk::Box::default())
            .detach();

        let model = MessageWindowModel {
            messages,
            active_channel: None,
        };

        let messages_box = model.messages.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: relm4::prelude::ComponentSender<Self>) {
        println!("{:?}", message);
    }
}
