use leptos::prelude::RwSignal;

use crate::palette::ColorPalette;

trait BarPlottable: PartialOrd {}

pub struct BarPlotSeries<X, Y> {
    x: Vec<X>,
    y: Vec<Y>,
}

pub struct BarPlotConfig<X, Y> {
    palette: ColorPalette,
    series: RwSignal<Vec<RwSignal<BarPlotSeries<X, Y>>>>,
}

pub struct BarConfigPlotBuilder {}
