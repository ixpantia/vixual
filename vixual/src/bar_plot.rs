use leptos::prelude::{RwSignal, Signal};

use crate::palette::ColorPalette;

pub trait BarPlottable: PartialOrd + Clone + Send + Sync + 'static {}
impl<T: PartialOrd + Clone + Send + Sync + 'static> BarPlottable for T {}

#[derive(Clone, Copy)]
pub struct BarPlotSeries<X, Y>
where
    X: Send + Sync + 'static,
    Y: Send + Sync + 'static,
{
    x: RwSignal<Vec<X>>,
    y: RwSignal<Vec<Y>>,
}

impl<X, Y> BarPlotSeries<X, Y>
where
    X: Send + Sync + 'static,
    Y: Send + Sync + 'static,
{
    pub fn new(x: Vec<X>, y: Vec<Y>) -> Self {
        Self {
            x: RwSignal::new(x),
            y: RwSignal::new(y),
        }
    }
}

pub struct BarPlotConfig<X, Y>
where
    X: Send + Sync + 'static,
    Y: Send + Sync + 'static,
{
    palette: Signal<ColorPalette>,
    series: Signal<Vec<BarPlotSeries<X, Y>>>,
}

impl<X, Y> BarPlotConfig<X, Y>
where
    X: BarPlottable,
    Y: BarPlottable,
{
    pub fn builder() -> BarPlotConfigBuilder<X, Y> {
        BarPlotConfigBuilder::new()
    }
}
pub struct BarPlotConfigBuilder<X, Y>
where
    X: Send + Sync + 'static,
    Y: Send + Sync + 'static,
{
    palette: Option<Signal<ColorPalette>>,
    series: Option<Signal<Vec<BarPlotSeries<X, Y>>>>,
    series_vec: Vec<BarPlotSeries<X, Y>>,
}

impl<X, Y> BarPlotConfigBuilder<X, Y>
where
    X: BarPlottable,
    Y: BarPlottable,
{
    pub fn new() -> Self {
        Self {
            palette: None,
            series: None,
            series_vec: Vec::new(),
        }
    }

    pub fn palette(mut self, palette: impl Into<Signal<ColorPalette>>) -> Self {
        self.palette = Some(palette.into());
        self
    }

    pub fn add_series(mut self, series: BarPlotSeries<X, Y>) -> Self {
        self.series_vec.push(series);
        self
    }

    pub fn with_series(mut self, series: impl Into<Signal<Vec<BarPlotSeries<X, Y>>>>) -> Self {
        self.series = Some(series.into());
        self
    }

    pub fn build(self) -> BarPlotConfig<X, Y> {
        let palette = self
            .palette
            .unwrap_or_else(|| Signal::stored(ColorPalette::default()));

        let series = self
            .series
            .unwrap_or_else(|| Signal::stored(self.series_vec));

        BarPlotConfig { palette, series }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::GetUntracked;

    #[test]
    fn test_bar_plot_builder() {
        let series1 = BarPlotSeries::new(vec!["A".to_string(), "B".to_string()], vec![1.0, 2.0]);
        let series2 = BarPlotSeries::new(vec!["C".to_string(), "D".to_string()], vec![3.0, 4.0]);

        let config = BarPlotConfig::builder()
            .add_series(series1)
            .add_series(series2)
            .build();

        assert_eq!(config.series.get_untracked().len(), 2);
    }
}
