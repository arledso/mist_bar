use relm4::prelude::*;

#[relm4::widget_template(pub)]
impl WidgetTemplate for Clock {
    view! {
        gtk::Box {
            set_margin_all: 10,
            inline_css: "border: 2px solid blue",
        } 
    }
}
