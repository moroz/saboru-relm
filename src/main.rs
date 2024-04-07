use std::convert::identity;

use components::message_window::MessageWindowModel;
use components::sidebar::SidebarModel;
use gtk::gdk::Display;
use gtk::prelude::*;
use gtk::{gio, glib, ApplicationWindow, CssProvider};
use relm4::{
    Component, ComponentController, ComponentParts, ComponentSender, Controller, RelmApp,
    SimpleComponent,
};

mod components;

#[derive(Debug)]
struct App {
    active_channel: Option<i64>,
    sidebar: Controller<SidebarModel>,
    message_window: Controller<MessageWindowModel>,
}

#[derive(Debug)]
pub enum AppMsg {
    SetChannel(i64),
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> relm4::prelude::ComponentParts<Self> {
        let sidebar = SidebarModel::builder()
            .launch(())
            .forward(sender.input_sender(), identity);
        let message_window = MessageWindowModel::builder()
            .launch(())
            .forward(sender.input_sender(), identity);

        let widgets = view_output!();

        let model = App {
            sidebar,
            message_window,
            active_channel: None,
        };

        ComponentParts { model, widgets }
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

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            AppMsg::SetChannel(id) => {
                self.active_channel = Some(id);
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("dev.moroz.saboru");
    let css =
        &glib::GString::from_string_checked(include_str!("css/style.css").to_string()).unwrap();
    app.set_global_css(css);
    app.run::<App>(());
}
