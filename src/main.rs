use std::convert::identity;
use std::rc::Rc;
use std::time::Duration;

use components::message_window::MessageWindowModel;
use components::sidebar::SidebarModel;
use components::sidebar_item::Channel;
use gtk::gdk::Display;
use gtk::prelude::*;
use gtk::{gio, glib, ApplicationWindow, CssProvider};
use relm4::component::{AsyncComponentParts, SimpleAsyncComponent};
use relm4::{
    AsyncComponentSender, Component, ComponentController, ComponentParts, ComponentSender,
    Controller, RelmApp, SimpleComponent,
};

mod components;

#[derive(Debug)]
struct App {
    active_channel: Option<i64>,
    channels: Rc<Vec<Channel>>,
    sidebar: Controller<SidebarModel>,
    message_window: Controller<MessageWindowModel>,
}

#[derive(Debug)]
pub enum AppMsg {
    SetChannel(i64),
    ChannelsFetched(Rc<Vec<Channel>>),
    FetchChannels,
}

#[relm4::component(async)]
impl SimpleAsyncComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    async fn init(
        init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> relm4::prelude::AsyncComponentParts<Self> {
        let sidebar = SidebarModel::builder()
            .launch(())
            .forward(sender.input_sender(), identity);
        let message_window = MessageWindowModel::builder()
            .launch(())
            .forward(sender.input_sender(), identity);

        let widgets = view_output!();

        let model = App {
            sidebar,
            channels: Rc::new(vec![]),
            message_window,
            active_channel: None,
        };

        sender.input_sender().send(AppMsg::FetchChannels).unwrap();

        AsyncComponentParts { model, widgets }
    }

    view! {
        #[root]
        gtk::Window {
            set_width_request: 1024,
            set_height_request: 768,
            set_title: Some("Saboru"),

            #[name = "layout"]
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                prepend: sidebar.widget(),

                #[name = "content"]
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    prepend: message_window.widget(),
                }
            },
        }
    }

    async fn update(&mut self, message: Self::Input, sender: AsyncComponentSender<Self>) {
        match message {
            AppMsg::SetChannel(id) => {
                self.active_channel = Some(id);
                self.message_window.sender().send(message).unwrap();
            }

            AppMsg::FetchChannels => {
                tokio::time::sleep(Duration::from_millis(2000)).await;
                let channel_names = [(1, "Alice"), (2, "Bob")];
                let channels = channel_names
                    .iter()
                    .map(|(id, label)| Channel {
                        id: *id,
                        label: label.to_string(),
                    })
                    .collect();

                sender
                    .input_sender()
                    .send(AppMsg::ChannelsFetched(Rc::new(channels)))
                    .unwrap();
            }
            AppMsg::ChannelsFetched(channels) => {
                self.channels = channels.clone();
                self.sidebar
                    .sender()
                    .send(AppMsg::ChannelsFetched(channels))
                    .unwrap();
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("dev.moroz.saboru");
    let css =
        &glib::GString::from_string_checked(include_str!("css/style.css").to_string()).unwrap();
    app.set_global_css(css);
    app.run_async::<App>(());
}
