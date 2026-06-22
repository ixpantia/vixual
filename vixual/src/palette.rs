use leptos::prelude::*;

// #RRGGBB
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Hex(pub [u8; 3]);

impl std::fmt::Display for Hex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.0[0], self.0[1], self.0[2])
    }
}

impl Hex {
    pub fn from_hex_str(s: &str) -> Option<Self> {
        let s = s.strip_prefix('#').unwrap_or(s);
        if s.len() != 6 {
            return None;
        }
        let r = u8::from_str_radix(&s[0..2], 16).ok()?;
        let g = u8::from_str_radix(&s[2..4], 16).ok()?;
        let b = u8::from_str_radix(&s[4..6], 16).ok()?;
        Some(Hex([r, g, b]))
    }
}

#[derive(Clone, Copy)]
pub struct ColorPalette {
    pub background: RwSignal<Option<Hex>>,
    pub colors: RwSignal<Vec<Hex>>,
}

impl ColorPalette {
    pub fn get_color(&self, index: usize) -> Hex {
        let colors = self.colors.get();
        if colors.is_empty() {
            Hex([0, 0, 0])
        } else {
            colors[index % colors.len()]
        }
    }

    pub fn set_colors(&self, colors: Vec<Hex>) {
        self.colors.set(colors);
    }

    pub fn set_background(&self, background: Option<Hex>) {
        self.background.set(background);
    }

    pub fn set_from_presets(&self, name: &str) {
        let colors = match name {
            "classic" => vec![
                Hex([31, 119, 180]),
                Hex([255, 127, 14]),
                Hex([44, 160, 44]),
                Hex([214, 39, 40]),
                Hex([148, 103, 189]),
                Hex([140, 86, 75]),
            ],
            "modern" => vec![
                Hex([0, 114, 178]),
                Hex([213, 94, 0]),
                Hex([0, 158, 115]),
                Hex([204, 121, 167]),
                Hex([240, 228, 66]),
                Hex([86, 180, 233]),
            ],
            "grayscale" => vec![
                Hex([50, 50, 50]),
                Hex([100, 100, 100]),
                Hex([150, 150, 150]),
                Hex([200, 200, 200]),
            ],
            _ => return,
        };
        self.colors.set(colors);
    }
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            background: RwSignal::new(None),
            colors: RwSignal::new(vec![
                Hex([31, 119, 180]),  // blue
                Hex([255, 127, 14]),  // orange
                Hex([44, 160, 44]),   // green
                Hex([214, 39, 40]),   // red
                Hex([148, 103, 189]), // purple
                Hex([140, 86, 75]),   // brown
            ]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::GetUntracked;

    #[test]
    fn test_hex_display() {
        let black = Hex([0, 0, 0]);
        let white = Hex([255, 255, 255]);
        let red = Hex([255, 0, 0]);
        let green = Hex([0, 255, 0]);
        let blue = Hex([0, 0, 255]);
        let custom = Hex([0x12, 0x34, 0x56]);

        assert_eq!(format!("{}", black), "#000000");
        assert_eq!(format!("{}", white), "#FFFFFF");
        assert_eq!(format!("{}", red), "#FF0000");
        assert_eq!(format!("{}", green), "#00FF00");
        assert_eq!(format!("{}", blue), "#0000FF");
        assert_eq!(format!("{}", custom), "#123456");
    }
}
