use crate::get_app::get_app;
use crate::set_colors::set_colors;
use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};

pub fn entrypoint() {
    let app = get_app();
    set_colors();

    let mut wind = Window::new(100, 100, 400, 300, "FerrisChat");
    let mut frame = Frame::new(0, 0, 400, 200, "");
    let mut but = Button::new(160, 210, 80, 40, "Click me!");
    wind.end();
    wind.show();
    but.set_callback(move |_| frame.set_label("Hello World!")); // the closure capture is mutable borrow to our button
    app.run().unwrap();
}
