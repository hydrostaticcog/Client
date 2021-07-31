use client_consts::{
    ACTIVE_SELECTION as AS, BACKGROUND_COLOR_PRIMARY as BG1, BACKGROUND_COLOR_SECONDARY as BG2,
    FOREGROUND_COLOR as FG, FRAME_COLOR as FC, FREE as FR, INACTIVE_ITEM as II,
};
use fltk::{app, enums::Color};

pub fn set_colors() {
    app::set_color(Color::BackGround, BG1.red(), BG1.green(), BG1.blue());
    app::set_color(Color::BackGround2, BG2.red(), BG2.green(), BG2.blue());
    app::set_color(Color::ForeGround, FG.red(), FG.green(), FG.blue());
    app::set_color(Color::FrameDefault, FC.red(), FC.green(), FC.blue());
    app::set_color(Color::Selection, AS.red(), AS.green(), AS.blue());
    app::set_color(Color::Free, FR.red(), FR.green(), FR.blue());
    app::set_color(Color::Inactive, II.red(), II.green(), II.blue());
}
