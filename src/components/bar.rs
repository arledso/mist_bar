use gtk::{prelude::*, glib::{timeout_add_seconds, ControlFlow}};
use relm4::prelude::*;
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use chrono::prelude::*;

use crate::widget_templates::standard_box::StandardBox;

#[derive(Debug)]
pub enum BarMsg { 
    TickClock,
}

pub struct BarModel {
    time_string: String,
}

#[relm4::component(pub)]
impl SimpleComponent for BarModel {
    type Init = String;

    type Input = BarMsg;
    type Output = ();

    view! {
        gtk::Window {
            // Config:
            set_title: Some("Simple app"),
            inline_css: "background: #00000020; border-radius: 5px;",
            set_default_height: 35,

            // Positioning Config:
            init_layer_shell: (),

            auto_exclusive_zone_enable: (),
            set_layer: Layer::Overlay,

            set_anchor: (Edge::Top, true),
            set_anchor: (Edge::Left, true),
            set_anchor: (Edge::Right, true),

            set_margin: (Edge::Top, 5),
            set_margin: (Edge::Bottom, -5),
            set_margin: (Edge::Left, 15),
            set_margin: (Edge::Right, 15),

            // Components:
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 5,
                set_margin_all: 5,
                set_halign: gtk::Align::Center,

                #[template]
                StandardBox {
                    set_halign: gtk::Align::Center,
                    set_hexpand: true,

                    connect_map[sender] => move |_| { 
                        let sender_clone = sender.clone();
                        timeout_add_seconds(1, move || { sender_clone.input(BarMsg::TickClock); glib::ControlFlow::Continue });
                    },
                    gtk::Label {
                        #[watch]
                        set_label: &model.date_string,
                        inline_css: "color: #ebdbb2",
                        set_margin_all: 5,
                        set_halign: gtk::Align::Center,
                        set_hexpand: true,
                    }
                },
            },
        }
    }

    fn init(
        date_string: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = BarModel { date_string };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            BarMsg::TickClock => { self.date_string = Local::now().format("%I:%M%P").to_string(); println!("TickClock Msg Called and handled.") }
        }
    }
}
