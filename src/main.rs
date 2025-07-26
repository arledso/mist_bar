mod components;
mod widget_templates;

use components::bar::BarModel;

use relm4::RelmApp;

fn main() {
    let app = RelmApp::new("relm4.test.simple");
    relm4::set_global_css("
        * {
            font-family: 'FiraCode Nerd Font Mono';
            font-size: 12px;
        }
        label {
            font-weight: light;
        }
    ");
    app.run::<BarModel>(BarModel { date_string: String::from("00:00am"), time_string: String::from("...")});
}
