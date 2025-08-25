mod components;
mod widget_templates;
mod backend;

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
            font-weight: bold;
        }
    ");

    use crate::backend::spotify::functions;
    let bar_model = BarModel { 
        date_string: String::from("00:00am"), 
        time_string: String::from("..."),
        spotify_api: functions::get_authed_spotify(),
        spotify_currently_playing_string: String::from("...") // TODO: change all placeholders to
                                                              // proper values on initiation
                                                              // please, Amen!
    };
    app.run::<BarModel>(bar_model);
}
