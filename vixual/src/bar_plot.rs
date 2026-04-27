use super::OnClient;
use leptos::prelude::*;
use leptos_use::{UseElementSizeReturn, use_element_size};

use crate::palette::ColorPalette;

pub trait Plotable: PartialOrd + Clone + Send + Sync + 'static {
    fn to_f64(&self) -> f64;
    fn to_string(&self) -> String;
}

impl Plotable for f64 {
    fn to_f64(&self) -> f64 {
        *self
    }
    fn to_string(&self) -> String {
        ToString::to_string(self)
    }
}

impl Plotable for i32 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
    fn to_string(&self) -> String {
        ToString::to_string(self)
    }
}

impl Plotable for String {
    fn to_f64(&self) -> f64 {
        self.parse().unwrap_or(0.0)
    }
    fn to_string(&self) -> String {
        self.clone()
    }
}

impl Plotable for &'static str {
    fn to_f64(&self) -> f64 {
        self.parse().unwrap_or(0.0)
    }
    fn to_string(&self) -> String {
        ToString::to_string(self)
    }
}

#[derive(Clone)]
pub struct BarPlotSeriesInner<X, Y> {
    x: Vec<X>,
    y: Vec<Y>,
}

#[derive(Clone, Copy)]
pub struct BarPlotSeries<X, Y>
where
    X: Plotable,
    Y: Plotable,
{
    data: RwSignal<BarPlotSeriesInner<X, Y>>,
}

impl<X, Y> BarPlotSeries<X, Y>
where
    X: Plotable,
    Y: Plotable,
{
    pub fn new(x: Vec<X>, y: Vec<Y>) -> Self {
        let data = BarPlotSeriesInner { x, y };
        Self {
            data: RwSignal::new(data),
        }
    }
    pub fn clear(&self) {
        self.data.update(|d| {
            d.x.clear();
            d.y.clear();
        });
    }
    /// This method updated the underlying signal to the series. Every push
    /// is an access to a Signal. If you need to call this in a loop
    /// it is recommended to use `append`
    pub fn push(&self, x: X, y: Y) {
        self.data.update(move |d| {
            d.x.push(x);
            d.y.push(y);
        });
    }

    pub fn append(&self, x: &mut Vec<X>, y: &mut Vec<Y>) {
        self.data.update(move |d| {
            d.x.append(x);
            d.y.append(y);
        });
    }

    pub fn replace_data(&self, x: Vec<X>, y: Vec<Y>) {
        self.data.update(move |d| {
            d.x = x;
            d.y = y;
        });
    }

    pub fn get_x(&self) -> Vec<X> {
        self.data.get().x
    }
    pub fn get_y(&self) -> Vec<Y> {
        self.data.get().y
    }
}

pub type FormatterFn<T> = std::sync::Arc<dyn Fn(&T) -> String + Send + Sync>;

#[derive(Clone)]
pub struct BarPlotConfig<X, Y>
where
    X: Plotable,
    Y: Plotable,
{
    pub palette: Signal<ColorPalette>,
    pub series: Signal<Vec<BarPlotSeries<X, Y>>>,
    pub title: Signal<String>,
    pub x_label: Signal<String>,
    pub y_label: Signal<String>,
    pub margin_top: Signal<f64>,
    pub margin_bottom: Signal<f64>,
    pub margin_left: Signal<f64>,
    pub margin_right: Signal<f64>,
    pub bar_relative_width: Signal<f64>,
    pub x_formatter: FormatterFn<X>,
    pub y_formatter: FormatterFn<Y>,
    pub x_tick_formatter: FormatterFn<X>,
    pub y_tick_formatter: FormatterFn<f64>,
}

impl<X, Y> BarPlotConfig<X, Y>
where
    X: Plotable,
    Y: Plotable,
{
    pub fn builder() -> BarPlotConfigBuilder<X, Y> {
        BarPlotConfigBuilder::new()
    }

    pub fn with_palette(mut self, palette: impl Into<Signal<ColorPalette>>) -> Self {
        self.palette = palette.into();
        self
    }

    pub fn with_title(mut self, title: impl Into<Signal<String>>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_x_label(mut self, x_label: impl Into<Signal<String>>) -> Self {
        self.x_label = x_label.into();
        self
    }

    pub fn with_y_label(mut self, y_label: impl Into<Signal<String>>) -> Self {
        self.y_label = y_label.into();
        self
    }

    pub fn with_margin_top(mut self, margin_top: impl Into<Signal<f64>>) -> Self {
        self.margin_top = margin_top.into();
        self
    }

    pub fn with_margin_bottom(mut self, margin_bottom: impl Into<Signal<f64>>) -> Self {
        self.margin_bottom = margin_bottom.into();
        self
    }

    pub fn with_margin_left(mut self, margin_left: impl Into<Signal<f64>>) -> Self {
        self.margin_left = margin_left.into();
        self
    }

    pub fn with_margin_right(mut self, margin_right: impl Into<Signal<f64>>) -> Self {
        self.margin_right = margin_right.into();
        self
    }

    pub fn with_bar_relative_width(mut self, bar_relative_width: impl Into<Signal<f64>>) -> Self {
        self.bar_relative_width = bar_relative_width.into();
        self
    }

    pub fn with_x_formatter(
        mut self,
        formatter: impl Fn(&X) -> String + Send + Sync + 'static,
    ) -> Self {
        self.x_formatter = std::sync::Arc::new(formatter);
        self
    }

    pub fn with_y_formatter(
        mut self,
        formatter: impl Fn(&Y) -> String + Send + Sync + 'static,
    ) -> Self {
        self.y_formatter = std::sync::Arc::new(formatter);
        self
    }

    pub fn with_x_tick_formatter(
        mut self,
        formatter: impl Fn(&X) -> String + Send + Sync + 'static,
    ) -> Self {
        self.x_tick_formatter = std::sync::Arc::new(formatter);
        self
    }

    pub fn with_y_tick_formatter(
        mut self,
        formatter: impl Fn(&f64) -> String + Send + Sync + 'static,
    ) -> Self {
        self.y_tick_formatter = std::sync::Arc::new(formatter);
        self
    }
}
pub struct BarPlotConfigBuilder<X, Y>
where
    X: Plotable,
    Y: Plotable,
{
    palette: Option<Signal<ColorPalette>>,
    series: Option<Signal<Vec<BarPlotSeries<X, Y>>>>,
    title: Option<Signal<String>>,
    x_label: Option<Signal<String>>,
    y_label: Option<Signal<String>>,
    margin_top: Option<Signal<f64>>,
    margin_bottom: Option<Signal<f64>>,
    margin_left: Option<Signal<f64>>,
    margin_right: Option<Signal<f64>>,
    bar_relative_width: Option<Signal<f64>>,
    x_formatter: Option<FormatterFn<X>>,
    y_formatter: Option<FormatterFn<Y>>,
    x_tick_formatter: Option<FormatterFn<X>>,
    y_tick_formatter: Option<FormatterFn<f64>>,
}

impl<X, Y> Default for BarPlotConfigBuilder<X, Y>
where
    X: Plotable,
    Y: Plotable,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<X, Y> BarPlotConfigBuilder<X, Y>
where
    X: Plotable,
    Y: Plotable,
{
    pub fn new() -> Self {
        Self {
            palette: None,
            series: None,
            title: None,
            x_label: None,
            y_label: None,
            margin_top: None,
            margin_bottom: None,
            margin_left: None,
            margin_right: None,
            bar_relative_width: None,
            x_formatter: None,
            y_formatter: None,
            x_tick_formatter: None,
            y_tick_formatter: None,
        }
    }

    pub fn palette(mut self, palette: impl Into<Signal<ColorPalette>>) -> Self {
        self.palette = Some(palette.into());
        self
    }

    pub fn with_series(mut self, series: impl Into<Signal<Vec<BarPlotSeries<X, Y>>>>) -> Self {
        self.series = Some(series.into());
        self
    }

    pub fn title(mut self, title: impl Into<Signal<String>>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn x_label(mut self, x_label: impl Into<Signal<String>>) -> Self {
        self.x_label = Some(x_label.into());
        self
    }

    pub fn y_label(mut self, y_label: impl Into<Signal<String>>) -> Self {
        self.y_label = Some(y_label.into());
        self
    }

    pub fn margin_top(mut self, margin_top: impl Into<Signal<f64>>) -> Self {
        self.margin_top = Some(margin_top.into());
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: impl Into<Signal<f64>>) -> Self {
        self.margin_bottom = Some(margin_bottom.into());
        self
    }

    pub fn margin_left(mut self, margin_left: impl Into<Signal<f64>>) -> Self {
        self.margin_left = Some(margin_left.into());
        self
    }

    pub fn margin_right(mut self, margin_right: impl Into<Signal<f64>>) -> Self {
        self.margin_right = Some(margin_right.into());
        self
    }

    pub fn bar_relative_width(mut self, bar_relative_width: impl Into<Signal<f64>>) -> Self {
        self.bar_relative_width = Some(bar_relative_width.into());
        self
    }

    pub fn x_formatter(mut self, formatter: impl Fn(&X) -> String + Send + Sync + 'static) -> Self {
        self.x_formatter = Some(std::sync::Arc::new(formatter));
        self
    }

    pub fn y_formatter(mut self, formatter: impl Fn(&Y) -> String + Send + Sync + 'static) -> Self {
        self.y_formatter = Some(std::sync::Arc::new(formatter));
        self
    }

    pub fn x_tick_formatter(
        mut self,
        formatter: impl Fn(&X) -> String + Send + Sync + 'static,
    ) -> Self {
        self.x_tick_formatter = Some(std::sync::Arc::new(formatter));
        self
    }

    pub fn y_tick_formatter(
        mut self,
        formatter: impl Fn(&f64) -> String + Send + Sync + 'static,
    ) -> Self {
        self.y_tick_formatter = Some(std::sync::Arc::new(formatter));
        self
    }

    pub fn build(self) -> BarPlotConfig<X, Y> {
        let palette = self
            .palette
            .unwrap_or_else(|| Signal::stored(ColorPalette::default()));

        let series = self.series.unwrap_or_else(|| Signal::stored(Vec::new()));

        let title = self.title.unwrap_or_else(|| Signal::stored(String::new()));

        let x_label = self
            .x_label
            .unwrap_or_else(|| Signal::stored(String::new()));

        let y_label = self
            .y_label
            .unwrap_or_else(|| Signal::stored(String::new()));

        let margin_top = self.margin_top.unwrap_or_else(|| Signal::stored(40.0));
        let margin_bottom = self.margin_bottom.unwrap_or_else(|| Signal::stored(40.0));
        let margin_left = self.margin_left.unwrap_or_else(|| Signal::stored(60.0));
        let margin_right = self.margin_right.unwrap_or_else(|| Signal::stored(20.0));
        let bar_relative_width = self
            .bar_relative_width
            .unwrap_or_else(|| Signal::stored(0.5));

        let x_formatter = self
            .x_formatter
            .unwrap_or_else(|| std::sync::Arc::new(|x: &X| x.to_string()));

        let y_formatter = self
            .y_formatter
            .unwrap_or_else(|| std::sync::Arc::new(|y: &Y| y.to_string()));

        let x_tick_formatter = self
            .x_tick_formatter
            .unwrap_or_else(|| std::sync::Arc::new(|x: &X| x.to_string()));

        let y_tick_formatter = self
            .y_tick_formatter
            .unwrap_or_else(|| std::sync::Arc::new(|y: &f64| format!("{:.1}", y)));

        BarPlotConfig {
            palette,
            series,
            title,
            x_label,
            y_label,
            margin_top,
            margin_bottom,
            margin_left,
            margin_right,
            bar_relative_width,
            x_formatter,
            y_formatter,
            x_tick_formatter,
            y_tick_formatter,
        }
    }
}

#[component]
pub fn BarPlot<X, Y>(
    config: Signal<BarPlotConfig<X, Y>>,
    #[prop(optional, into)] width: Option<Signal<String>>,
    #[prop(optional, into)] height: Option<Signal<String>>,
) -> impl IntoView
where
    X: Plotable,
    Y: Plotable,
{
    let el = NodeRef::<leptos::html::Div>::new();
    let UseElementSizeReturn {
        width: container_width,
        height: container_height,
    } = use_element_size(el);

    let resolved_width = move || {
        if let Some(w) = width.as_ref() {
            let w_val = w.get();
            if let Ok(val) = w_val.parse::<f64>() {
                return val;
            }
            // If it's a percentage or something else, we still need a pixel value for SVG internal coords
            // unless we want to change how it works.
            // For now, if it's not a direct number, we'll try to use container width.
        }
        container_width.get().max(100.0)
    };

    let resolved_height = move || {
        if let Some(h) = height.as_ref() {
            let h_val = h.get();
            if let Ok(val) = h_val.parse::<f64>() {
                return val;
            }
        }
        container_height.get().max(100.0)
    };

    let bars = move || {
        let width = resolved_width();
        let height = resolved_height();
        let conf = config.get();
        let series_vec = conf.series.get();
        let palette = conf.palette.get();
        let margin_top = conf.margin_top.get();
        let margin_bottom = conf.margin_bottom.get();
        let margin_left = conf.margin_left.get();
        let margin_right = conf.margin_right.get();
        let bar_relative_width = conf.bar_relative_width.get();

        let mut all_y = Vec::new();
        let mut total_bars = 0;

        for s in &series_vec {
            let y_vals: Vec<Y> = s.get_y();
            all_y.extend(y_vals.iter().map(|v| v.to_f64()));
            total_bars += s.get_x().len();
        }

        let max_y = all_y
            .into_iter()
            .fold(0.0, |a, b| if a > b { a } else { b });
        let scale_y = if max_y == 0.0 {
            1.0
        } else {
            (height - margin_top - margin_bottom) / max_y
        };

        let bar_slot_width = (width - margin_left - margin_right) / (total_bars as f64).max(1.0);

        let mut current_bar = 0;
        let mut elements = Vec::new();

        for (series_idx, s) in series_vec.iter().enumerate() {
            let x_vals: Vec<X> = s.get_x();
            let y_vals: Vec<Y> = s.get_y();
            let color = palette.get_color(series_idx);

            for (i, y_val) in y_vals.iter().enumerate() {
                let val_f64 = y_val.to_f64();
                let bar_height = val_f64 * scale_y;
                let x_center = margin_left + (current_bar as f64 + 0.5) * bar_slot_width;
                let bar_w = bar_slot_width * bar_relative_width;
                let x_pos = x_center - bar_w / 2.0;
                let y_pos = height - margin_bottom - bar_height;

                let formatted_x = (conf.x_formatter)(&x_vals[i]);
                let formatted_y = (conf.y_formatter)(y_val);
                let formatted_x_tick = (conf.x_tick_formatter)(&x_vals[i]);

                elements.push(view! {
                    <rect x=x_pos y=y_pos width=bar_w height=bar_height fill=color.to_string()>
                        <title>{format!("{}: {}", formatted_x, formatted_y)}</title>
                    </rect>
                    <text
                        x=x_center
                        y=height - margin_bottom + 15.0
                        text-anchor="middle"
                        font-size="12"
                    >
                        {formatted_x_tick}
                    </text>
                });
                current_bar += 1;
            }
        }
        elements
    };

    let y_axis = move || {
        let height = resolved_height();
        let conf = config.get();
        let series_vec = conf.series.get();
        let margin_top = conf.margin_top.get();
        let margin_bottom = conf.margin_bottom.get();
        let margin_left = conf.margin_left.get();

        let mut all_y = Vec::new();
        for s in &series_vec {
            let y_vals: Vec<Y> = s.get_y();
            all_y.extend(y_vals.iter().map(|v| v.to_f64()));
        }

        let max_y = all_y
            .into_iter()
            .fold(0.0, |a, b| if a > b { a } else { b });

        let num_ticks = 5;
        let mut ticks = Vec::new();

        for i in 0..=num_ticks {
            let tick_val = (max_y / num_ticks as f64) * (i as f64);
            let scale_y = if max_y == 0.0 {
                1.0
            } else {
                (height - margin_top - margin_bottom) / max_y
            };
            let y_pos = height - margin_bottom - (tick_val * scale_y);

            let formatted_tick = (conf.y_tick_formatter)(&tick_val);
            ticks.push(view! {
                <line x1=margin_left - 5.0 y1=y_pos x2=margin_left y2=y_pos stroke="black" />
                <text x=margin_left - 10.0 y=y_pos + 5.0 text-anchor="end" font-size="12">
                    {formatted_tick}
                </text>
            });
        }
        ticks
    };

    let title_view = move || {
        let width = resolved_width();
        let conf = config.get();
        let title_text = conf.title.get();
        let margin_top = conf.margin_top.get();
        if !title_text.is_empty() {
            Some(view! {
                <text
                    x=width / 2.0
                    y=margin_top / 2.0 + 10.0
                    text-anchor="middle"
                    font-size="18"
                    font-weight="bold"
                >
                    {title_text}
                </text>
            })
        } else {
            None
        }
    };

    let x_label_view = move || {
        let width = resolved_width();
        let height = resolved_height();
        let conf = config.get();
        let label_text = conf.x_label.get();
        if !label_text.is_empty() {
            Some(view! {
                <text x=width / 2.0 y=height - 1.0 text-anchor="middle" font-size="14">
                    {label_text}
                </text>
            })
        } else {
            None
        }
    };

    let y_label_view = move || {
        let height = resolved_height();
        let conf = config.get();
        let label_text = conf.y_label.get();
        if !label_text.is_empty() {
            Some(view! {
                <text
                    x=15.0
                    y=height / 2.0
                    text-anchor="middle"
                    font-size="14"
                    transform=format!("rotate(-90, 15, {})", height / 2.0)
                >
                    {label_text}
                </text>
            })
        } else {
            None
        }
    };

    let svg_width = move || {
        width
            .as_ref()
            .map(|w| w.get())
            .unwrap_or_else(|| "100%".to_string())
    };
    let svg_height = move || {
        height
            .as_ref()
            .map(|h| h.get())
            .unwrap_or_else(|| "100%".to_string())
    };

    view! {
        <div
            node_ref=el
            style:width=svg_width
            style:height=svg_height
            style="display: block; overflow: hidden;"
        >
            <OnClient>
                <svg
                    width="100%"
                    height="100%"
                    viewBox=move || {
                        let w = resolved_width();
                        let h = resolved_height();
                        format!("0 0 {w} {h}")
                    }
                >
                    // X-axis
                    <line
                        x1=move || config.get().margin_left.get()
                        y1=move || resolved_height() - config.get().margin_bottom.get()
                        x2=move || resolved_width() - config.get().margin_right.get()
                        y2=move || resolved_height() - config.get().margin_bottom.get()
                        stroke="black"
                    />
                    // Y-axis
                    <line
                        x1=move || config.get().margin_left.get()
                        y1=move || config.get().margin_top.get()
                        x2=move || config.get().margin_left.get()
                        y2=move || resolved_height() - config.get().margin_bottom.get()
                        stroke="black"
                    />
                    {title_view}
                    {x_label_view}
                    {y_label_view}
                    {y_axis}
                    {bars}
                </svg>
            </OnClient>
        </div>
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
            .with_series(vec![series1, series2])
            .title("Test Title")
            .margin_top(10.0)
            .margin_bottom(20.0)
            .margin_left(30.0)
            .margin_right(40.0)
            .build();

        assert_eq!(config.series.get_untracked().len(), 2);
        assert_eq!(config.title.get_untracked(), "Test Title");
        assert_eq!(config.margin_top.get_untracked(), 10.0);
        assert_eq!(config.margin_bottom.get_untracked(), 20.0);
        assert_eq!(config.margin_left.get_untracked(), 30.0);
        assert_eq!(config.margin_right.get_untracked(), 40.0);
    }

    #[test]
    fn test_bar_plot_builder_with_width() {
        let config = BarPlotConfig::<f64, f64>::builder()
            .bar_relative_width(0.5)
            .build();

        assert_eq!(config.bar_relative_width.get_untracked(), 0.5);
    }
}
