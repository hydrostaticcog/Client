mod color_types;
pub use color_types::CustomRgbColor;

/// The primary background color
///
/// rgb(0,0,0) in dark mode, and rgb(255, 255, 255) in light mode
pub const BACKGROUND_COLOR_PRIMARY: CustomRgbColor = CustomRgbColor::new(0, 0, 0);

/// The secondary background color
///
/// rgb(5,5,5) in dark mode, and rgb(250, 250, 250) in light mode
pub const BACKGROUND_COLOR_SECONDARY: CustomRgbColor = CustomRgbColor::new(5, 5, 5);

/// The foreground color
///
/// rgb(255,255,255) in dark mode, and rgb(0,0,0) in light mode
pub const FOREGROUND_COLOR: CustomRgbColor = CustomRgbColor::new(255, 255, 255);

/// Inactive UI elements (ie not in use)
///
/// rgb(10,10,10) in dark mode, and rgb(245, 245, 245) in light mode
pub const INACTIVE_ITEM: CustomRgbColor = CustomRgbColor::new(10, 10, 10);

/// Any currently selected item
///
/// rgb(127, 127, 127) in both dark and light mode.
pub const ACTIVE_SELECTION: CustomRgbColor = CustomRgbColor::new(127, 127, 127);

/// I have no idea what FREE is
///
/// rgb(255, 0, 0) on both dark and light that way it stands out
pub const FREE: CustomRgbColor = CustomRgbColor::new(255, 0, 0);

/// The default color for a UI frame
///
/// Defaults to BACKGROUND_COLOR_SECONDARY
pub const FRAME_COLOR: CustomRgbColor = BACKGROUND_COLOR_SECONDARY;
