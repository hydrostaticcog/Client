pub struct CustomRgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl CustomRgbColor {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub const fn red(&self) -> u8 {
        self.r
    }

    pub const fn green(&self) -> u8 {
        self.g
    }

    pub const fn blue(&self) -> u8 {
        self.b
    }
}
