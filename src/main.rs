use std::convert::identity;

use components::sidebar::SidebarModel;
use gtk::prelude::*;
use relm4::{
    Component, ComponentController, ComponentParts, ComponentSender, Controller, RelmApp,
    RelmWidgetExt, SimpleComponent,
};

mod components;

#[derive(PartialEq, Eq, Debug, Clone)]
struct Channel {
    pub id: u64,
    pub name: &'static str,
}

#[derive(Debug)]
struct App {
    sidebar: Controller<SidebarModel>,
}

#[derive(Debug)]
pub enum AppMsg {
    SetChannel(u64),
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

        let widgets = view_output!();
        let model = App { sidebar };

        ComponentParts { model, widgets }
    }

    view! {
        #[root]
        gtk::Window {
            set_width_request: 1024,
            set_height_request: 768,

            #[name = "top_sidebar"]
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                append: sidebar.widget(),
            },
        }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {}
}

fn main() {
    let app = RelmApp::new("dev.moroz.saboru");
    app.run::<App>(());
}
