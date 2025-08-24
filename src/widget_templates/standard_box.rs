use relm4::prelude::*;
use relm4::gtk::prelude::*;

#[relm4::widget_template(pub)]
impl WidgetTemplate for StandardBox {
    view! {
        gtk::Box {
            // inline_css: "border: 2px solid #83a598; background: #323027; background: linear-gradient(175deg,rgba(50, 48, 39, 1) 0%, rgba(60, 56, 54, 1) 100%); width: 10px; border-radius: 5px;",
            inline_css: "box-shadow: rgb(38, 57, 77) 0px 20px 30px -10px;; border: 2px solid #98971a; background: #fbf1c7; width: 10px; border-radius: 5px;",
            set_width_request: 90,
        }, 
    }
}
