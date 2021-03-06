use std::fmt;

#[derive(Copy, Clone)]
#[repr(packed)]
pub struct Color {
    pub data: u32,
}

impl Color {
    // Create a new color from RGB
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color {
            data: 0xFF00_0000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32),
        }
    }

    // Create a new color from RGBA
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color {
            data: ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32),
        }
    }

    // Get the red value
    pub fn r(self) -> u8 {
        ((self.data & 0x00FF_0000) >> 16) as u8
    }

    // Get the green value
    pub fn g(self) -> u8 {
        ((self.data & 0x0000_FF00) >> 8) as u8
    }

    // Get the blue value
    pub fn b(self) -> u8 {
        (self.data & 0x0000_00FF) as u8
    }

    // Get the alpha value
    pub fn a(self) -> u8 {
        ((self.data & 0xFF00_0000) >> 24) as u8
    }

    // Interpolate between two colors
    pub fn interpolate(start_color: Color, end_color: Color, scale: f64) -> Color {
        let r = Color::interp(start_color.r(), end_color.r(), scale);
        let g = Color::interp(start_color.g(), end_color.g(), scale);
        let b = Color::interp(start_color.b(), end_color.b(), scale);
        let a = Color::interp(start_color.a(), end_color.a(), scale);
        Color::rgba(r, g, b, a)
    }

    fn interp(start_color: u8, end_color: u8, scale: f64) -> u8 {
        (end_color as f64 - start_color as f64).mul_add(scale, start_color as f64) as u8
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        if self.a() == 0 {
            return String::from("transparent");
        }

        let data = self.data;

        let mut color = format!("#{:x}", data);
        color.remove(1);
        color.remove(1);
        color
    }
}

impl From<&str> for Color {
    fn from(s: &str) -> Color {
        let clean_hex = s.trim_start_matches('#');
        match clean_hex.len() {
            6 | 8 => {
                let mut x = match u32::from_str_radix(&clean_hex, 16) {
                    Ok(x) => x,
                    Err(_) => 0,
                };

                if clean_hex.len() == 6 {
                    x |= 0xFF_000_000;
                }

                Color { data: x }
            }
            _ => Color { data: 0 },
        }
    }
}

impl From<String> for Color {
    fn from(s: String) -> Color {
        Color::from(s.as_str())
    }
}

impl From<Color> for femtovg::Color {
    fn from(src: Color) -> femtovg::Color {
        femtovg::Color::rgba(src.r(), src.g(), src.b(), src.a())
    }
}

/// Compare two colors (Do not take care of alpha)
impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        self.r() == other.r() && self.g() == other.g() && self.b() == other.b()
    }
}

impl std::fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(r: {}, g: {}, b: {} a: {})",
            self.r(),
            self.g(),
            self.b(),
            self.a()
        )
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::rgba(0, 0, 0, 0)
    }
}
