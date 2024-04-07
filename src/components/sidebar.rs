use super::sidebar_item::Channel;
use super::sidebar_item::DataRow;
use super::sidebar_item::SidebarRow;
use crate::AppMsg;
use gtk::gio;
use gtk::glib::clone;
use gtk::prelude::*;
use relm4::ComponentParts;
use relm4::SimpleComponent;
use std::rc::Rc;

#[derive(Debug)]
pub struct SidebarModel {
    channels_model: gtk::SingleSelection,
}

#[relm4::component(pub)]
impl SimpleComponent for SidebarModel {
    type Init = ();
    type Input = AppMsg;
    type Output = AppMsg;

    view! {
        gtk::ScrolledWindow {
            set_hscrollbar_policy: gtk::PolicyType::Never,
            set_vexpand: false,
            set_width_request: 300,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_css_classes: &["sidebar"],

                gtk::Label {
                    set_label: "Contacts",
                    set_margin_top: 10,
                    set_margin_start: 10,
                    set_margin_end: 10,
                    set_margin_bottom: 10,
                    set_halign: gtk::Align::Start,
                },

                #[name = "channels"]
                gtk::ListView {}
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: relm4::prelude::ComponentSender<Self>,
    ) -> relm4::prelude::ComponentParts<Self> {
        let store = gio::ListStore::new::<DataRow>();

        let model = SidebarModel {
            channels_model: gtk::SingleSelection::builder()
                .model(&store)
                .autoselect(true)
                .build(),
        };
        let widgets = view_output!();

        let factory = gtk::SignalListItemFactory::new();
        factory.connect_setup(move |_, item| {
            let component = SidebarRow::default();
            item.downcast_ref::<gtk::ListItem>()
                .unwrap()
                .set_child(Some(&component));
        });
        factory.connect_bind(move |_, item| {
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            let data_row = item.item().and_downcast::<DataRow>().unwrap();
            let child = item.child().and_downcast::<SidebarRow>().unwrap();
            child.set_content(data_row.property::<String>("label"));
        });

        model.channels_model.connect_selection_changed(
            clone!(@strong sender => move |selection, _, _| {
                if let Some(selected_item) = selection.selected_item() {
                    let item = selected_item.downcast_ref::<DataRow>().unwrap();
                    let property = item.property::<i64>("id");
                    sender.output(AppMsg::SetChannel(property)).unwrap()
                }
            }),
        );

        widgets.channels.set_factory(Some(&factory));
        widgets.channels.set_model(Some(&model.channels_model));

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: relm4::prelude::ComponentSender<Self>) {
        match message {
            AppMsg::ChannelsFetched(channels) => self.update_list(channels.clone()),
            _ => (),
        }
    }
}

impl SidebarModel {
    fn update_list(&mut self, channels: Rc<Vec<Channel>>) {
        let store = self
            .channels_model
            .model()
            .unwrap()
            .downcast::<gio::ListStore>()
            .unwrap();

        store.remove_all();

        for channel in channels.iter() {
            store.append(&DataRow::new(channel.id, channel.label.to_string()));
        }
    }
}
