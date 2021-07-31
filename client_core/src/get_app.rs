use fltk::app::{self, App};
use fltk::enums::Mode;

pub fn get_app() -> App {
    let app = app::App::default().load_system_fonts();

    app
}
