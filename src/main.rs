mod components;

use components::bar::BarModel;

use relm4::RelmApp;

fn main() {
    let app = RelmApp::new("relm4.test.simple");
    app.run::<BarModel>(0);
}
