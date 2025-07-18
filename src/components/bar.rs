use gtk::prelude::{ BoxExt, GtkWindowExt, OrientableExt };
use relm4::{ gtk, ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent };

#[derive(Debug)]
pub enum BarMsg { }

pub struct BarModel {
    counter: u8,
}

#[relm4::component(pub)]
impl SimpleComponent for BarModel {
    type Init = u8;

    type Input = BarMsg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Simple app"),
            set_default_width: 300,
            set_default_height: 100,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,
            }
        }
    }

    fn init(
        counter: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = BarModel { counter };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            // BarMsg::Increment => { self.counter = self.counter.wrapping_add(1); }
            // BarMsg::Decrement => { self.counter = self.counter.wrapping_sub(1); }
        }
    }
}
