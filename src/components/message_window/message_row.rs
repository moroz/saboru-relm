use gtk::prelude::*;
use relm4::factory::FactoryComponent;
use relm4::RelmWidgetExt;

use crate::components::types::ChatMessage;

#[derive(Debug)]
pub struct MessageRow {
    pub id: i64,
    pub body: String,
    pub sender: String,
}

#[relm4::factory(pub)]
impl FactoryComponent for MessageRow {
    type Init = ChatMessage;
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_halign: gtk::Align::Start,
            set_margin_all: 10,
            set_css_classes: &["message-row"],

            #[name(sender)]
            gtk::Label {
                set_halign: gtk::Align::Start,
                set_label: &self.sender,
                set_css_classes: &["sender"],
            },

            #[name(body)]
            gtk::Label {
                set_halign: gtk::Align::Start,
                set_label: &self.body,
            }
        }
    }

    fn init_model(
        init: Self::Init,
        _index: &Self::Index,
        _sender: relm4::prelude::FactorySender<Self>,
    ) -> Self {
        Self {
            id: init.id,
            body: init.body,
            sender: init.sender,
        }
    }
}
