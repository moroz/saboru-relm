use gtk::prelude::*;
use relm4::factory::FactoryComponent;

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

            #[name(body)]
            gtk::Label {
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
