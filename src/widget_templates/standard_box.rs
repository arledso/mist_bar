use relm4::prelude::*;
use relm4::gtk::prelude::*;

#[relm4::widget_template(pub)]
impl WidgetTemplate for StandardBox {
    view! {
        gtk::Box {
            inline_css: "border: 0px solid blue; background: #323027; background: linear-gradient(175deg,rgba(50, 48, 39, 1) 0%, rgba(60, 56, 54, 1) 100%); width: 10px; border-radius: 10px;",
            set_width_request: 90,
        }, 
    }
}
