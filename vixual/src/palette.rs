use leptos::prelude::RwSignal;

// #RRGGBB
#[derive(Clone, Copy)]
pub struct Hex([u8; 3]);

#[derive(Clone, Copy)]
pub struct ColorPalette {
    pub background: RwSignal<Option<Hex>>,
    pub colors: RwSignal<Vec<Hex>>,
}
