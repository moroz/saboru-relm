use std::convert::identity;
use std::rc::Rc;

use api::API;
use components::message_window::MessageWindowModel;
use components::sidebar::SidebarModel;
use components::sidebar_item::Channel;
use components::types::ChatMessage;
use gtk::glib;
use gtk::prelude::*;
use relm4::component::{AsyncComponentParts, SimpleAsyncComponent};
use relm4::{AsyncComponentSender, Component, ComponentController, Controller, RelmApp};

mod api;
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
    MessagesFetched(Rc<Vec<ChatMessage>>),
    FetchChannels,
    FetchMessages(i64),
}

#[relm4::component(async)]
impl SimpleAsyncComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    async fn init(
        _init: Self::Init,
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

                    #[name = "input"]
                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_height_request: 300,
                    },
                },
            },
        }
    }

    async fn update(&mut self, message: Self::Input, sender: AsyncComponentSender<Self>) {
        match message {
            AppMsg::SetChannel(id) => {
                self.active_channel = Some(id);
                self.message_window.sender().send(message).unwrap();
                sender.input(AppMsg::FetchMessages(id));
            }

            AppMsg::FetchChannels => {
                let channels = API::fetch_channels().await.unwrap();
                let first_channel = channels.first().unwrap().id;
                let channels = Rc::new(channels);

                self.sidebar
                    .sender()
                    .send(AppMsg::ChannelsFetched(channels.clone()))
                    .unwrap();

                self.channels = channels.clone();
                sender.input(AppMsg::SetChannel(first_channel));
            }

            AppMsg::FetchMessages(channel_id) => {
                let messages = API::fetch_messages(channel_id).await.unwrap();
                self.message_window
                    .sender()
                    .send(AppMsg::MessagesFetched(messages.into()))
                    .unwrap();
            }

            _ => (),
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
