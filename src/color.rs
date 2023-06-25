pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn to_string(&self) -> String {
        return format!("{};{};{}", self.r, self.g, self.b);
    }
}

pub const DEEP_DARK_GRAY: Color = Color {
    r: 63,
    g: 69,
    b: 85,
};

pub const DARK_GRAY: Color = Color {
    r: 192,
    g: 197,
    b: 214,
};

pub const GRAY: Color = Color {
    r: 72,
    g: 80,
    b: 95,
};

pub const LIGHT_GRAY: Color = Color {
    r: 72,
    g: 93,
    b: 119,
};

pub const LIGHT_BLUE: Color = Color {
    r: 135,
    g: 203,
    b: 233,
};

pub const GREEN_BLUE: Color = Color {
    r: 0,
    g: 162,
    b: 170,
};

pub const BLUE: Color = Color {
    r: 50,
    g: 108,
    b: 149,
};

pub const SUPER_LIGHT_BLUE: Color = Color {
    r: 166,
    g: 210,
    b: 246,
};
