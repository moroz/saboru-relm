use std::{convert::identity, rc::Rc};

use components::sidebar::SidebarModel;
use gtk::prelude::*;
use relm4::{
    Component, ComponentController, ComponentParts, ComponentSender, Controller, RelmApp,
    SimpleComponent,
};

mod components;

#[derive(Debug)]
struct App {
    active_channel: Option<i64>,
    sidebar: Rc<Controller<SidebarModel>>,
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
        let sidebar = Rc::new(
            SidebarModel::builder()
                .launch(())
                .forward(sender.input_sender(), identity),
        );

        let model = App {
            sidebar: sidebar.clone(),
            active_channel: None,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    view! {
        #[root]
        gtk::Window {
            set_width_request: 1024,
            set_height_request: 768,
            set_title: Some("Saboru"),

            #[name = "top_sidebar"]
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                prepend: sidebar.clone().widget(),

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,

                    gtk::Label {
                        #[watch]
                        set_label: &match model.active_channel {
                            None => format!("No active channel."),
                            Some(id) => format!("Active channel: {id}"),
                        }
                    }
                }
            },
        }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            AppMsg::SetChannel(id) => self.active_channel = Some(id),
        }
    }
}

fn main() {
    let app = RelmApp::new("dev.moroz.saboru");
    app.run::<App>(());
}
