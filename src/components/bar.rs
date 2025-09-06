use gtk::{prelude::*, glib::{timeout_add_seconds, ControlFlow}};
use std::path::PathBuf;
use relm4::prelude::*;
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use chrono::prelude::*;
use rspotify::AuthCodeSpotify;
use cmd_lib::run_cmd;

use crate::widget_templates::standard_box::StandardBox;

#[derive(Debug)]
pub enum BarMsg { 
    TickClock,
    TickSpotifyWidget,
}

pub struct BarModel {
    pub time_string: String,
    pub date_string: String,
    pub spotify_currently_playing_string: String,
    pub spotify_api: AuthCodeSpotify,
    pub spotify_currently_playing_image: PathBuf,
}

#[relm4::component(pub)]
impl SimpleComponent for BarModel {
    type Init = BarModel;

    type Input = BarMsg;
    type Output = ();

    view! {
        gtk::Window {
            // Config:
            set_title: Some("Simple app"),
            inline_css: "background: #00000020; border-radius: 5px;",
            set_default_height: 40,

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

                connect_map[sender] => move |_| { 
                    let sender_clone = sender.clone();
                    timeout_add_seconds(1, move || { sender_clone.input(BarMsg::TickClock); ControlFlow::Continue });
                },

                connect_map[sender] => move |_| { 
                    let sender_clone = sender.clone();
                    timeout_add_seconds(1, move || { sender_clone.input(BarMsg::TickSpotifyWidget); ControlFlow::Continue });
                },


                gtk::Button {
                    set_halign: gtk::Align::Start,
                    set_hexpand: true,
                    set_margin_all: 0,
                    inline_css: "margin: 0px; padding: 0px;",

                    connect_clicked => move |_| {
                        run_cmd!(
                            spotify
                        );
                    },

                    #[template]
                    StandardBox {
                        set_halign: gtk::Align::Start,
                        set_hexpand: true,

                        gtk::Image {
                            #[watch]
                            set_from_file: Some(model.spotify_currently_playing_image.clone()),
                            inline_css: "box-shadow: rgb(38, 57, 77) 0px 20px 30px -10px; width: 10px; border-radius: 20px; margin: 0px 2px 0px 4px;",
                        },

                        gtk::Label {
                            #[watch]
                            set_label: &model.spotify_currently_playing_string,
                            inline_css: "color: #3c3836",
                            set_margin_all: 5,
                            set_halign: gtk::Align::Center,
                            set_hexpand: true,
                        },
                    },
                },

                gtk::Box {
                    set_halign: gtk::Align::Start,
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 5,

                    #[template]
                    StandardBox {
                        set_halign: gtk::Align::Center,
                        set_hexpand: true,

                        gtk::Label {
                            #[watch]
                            set_label: &model.date_string,
                            inline_css: "color: #3c3836",
                            set_margin_all: 5,
                            set_halign: gtk::Align::Center,
                            set_hexpand: true,
                        }
                    },
                    #[template]
                    StandardBox {
                        set_halign: gtk::Align::Center,
                        set_hexpand: true,

                        connect_map[sender] => move |_| { 
                            let sender_clone = sender.clone();
                            timeout_add_seconds(1, move || { sender_clone.input(BarMsg::TickClock); ControlFlow::Continue });
                        },
                        gtk::Label {
                            #[watch]
                            set_label: &model.time_string,
                            inline_css: "color: #3c3836",
                            set_margin_all: 5,
                            set_halign: gtk::Align::Center,
                            set_hexpand: true,
                        }
                    },

                }
            },
        }
    }

    fn init(
        bar_model: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = bar_model;
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            BarMsg::TickClock => { 
                self.date_string = Local::now().format("%I:%M%P").to_string(); println!("TickClock Msg Called and handled.");
                self.time_string = Local::now().format(" %Y | %b %d | %a ").to_string(); println!("TickClock Msg Called and handled.");
            }
            BarMsg::TickSpotifyWidget => { 
                use crate::backend::spotify::functions;
                
                let item_name = functions::try_get_current_playing_item_name(&self.spotify_api);
                match item_name {
                    Ok(name_option) => {
                        match name_option {
                            Some(name) => self.spotify_currently_playing_string = name,
                            None => self.spotify_currently_playing_string = String::from("..."),
                        }
                    }
                    Err(error) => {
                        self.spotify_currently_playing_string = String::from("error!! try running bar from cli!");
                        println!("{}", error);
                    }
                }
                
                let image_path = 
                    functions::try_get_current_playing_item_image_in_cache(&self.spotify_api);
                match image_path {
                    Ok(path) => {
                        match path {
                            Some(image) => self.spotify_currently_playing_image = image,
                            None => self.spotify_currently_playing_image = PathBuf::new()
                        }
                    },
                    Err(error) => println!("{}", error),
                }
            },
        }
    }
}
