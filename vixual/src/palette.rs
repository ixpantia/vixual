use leptos::prelude::RwSignal;

// #RRGGBB
#[derive(Clone, Copy)]
pub struct Hex([u8; 3]);

impl std::fmt::Display for Hex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.0[0], self.0[1], self.0[2])
    }
}

#[derive(Clone, Copy)]
pub struct ColorPalette {
    pub background: RwSignal<Option<Hex>>,
    pub colors: RwSignal<Vec<Hex>>,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            background: RwSignal::new(None),
            colors: RwSignal::new(Vec::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
