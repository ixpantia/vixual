use super::OnClient;
use leptos::prelude::*;
use leptos_use::{UseElementSizeReturn, use_element_size};

use crate::palette::ColorPalette;
use crate::plotable::{DataType, Plotable};
use crate::series::Series;

pub type FormatterFn<T> = std::sync::Arc<dyn Fn(&T) -> String + Send + Sync>;

#[derive(Clone)]
pub struct LinePlotConfig<X, Y>
where
    X: Plotable,
    Y: Plotable,
{
    pub palette: Signal<ColorPalette>,
    pub series: Signal<Vec<Series<X, Y>>>,
    pub title: Signal<String>,
    pub x_label: Signal<String>,
    pub y_label: Signal<String>,
    pub margin_top: Signal<f64>,
    pub margin_bottom: Signal<f64>,
    pub margin_left: Signal<f64>,
    pub margin_right: Signal<f64>,
    pub x_formatter: FormatterFn<X>,
    pub y_formatter: FormatterFn<Y>,
    pub x_tick_formatter: FormatterFn<X>,
    pub continuous_x_tick_formatter: FormatterFn<f64>,
    pub y_tick_formatter: FormatterFn<f64>,
}

impl<X, Y> LinePlotConfig<X, Y>
where
    X: Plotable,
    Y: Plotable,
{
    pub fn builder() -> LinePlotConfigBuilder<X, Y> {
        LinePlotConfigBuilder::new()
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

    pub fn with_continuous_x_tick_formatter(
        mut self,
        formatter: impl Fn(&f64) -> String + Send + Sync + 'static,
    ) -> Self {
        self.continuous_x_tick_formatter = std::sync::Arc::new(formatter);
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
pub struct LinePlotConfigBuilder<X, Y>
where
    X: Plotable,
    Y: Plotable,
{
    palette: Option<Signal<ColorPalette>>,
    series: Option<Signal<Vec<Series<X, Y>>>>,
    title: Option<Signal<String>>,
    x_label: Option<Signal<String>>,
    y_label: Option<Signal<String>>,
    margin_top: Option<Signal<f64>>,
    margin_bottom: Option<Signal<f64>>,
    margin_left: Option<Signal<f64>>,
    margin_right: Option<Signal<f64>>,
    x_formatter: Option<FormatterFn<X>>,
    y_formatter: Option<FormatterFn<Y>>,
    x_tick_formatter: Option<FormatterFn<X>>,
    continuous_x_tick_formatter: Option<FormatterFn<f64>>,
    y_tick_formatter: Option<FormatterFn<f64>>,
}

impl<X, Y> Default for LinePlotConfigBuilder<X, Y>
where
    X: Plotable,
    Y: Plotable,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<X, Y> LinePlotConfigBuilder<X, Y>
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
            x_formatter: None,
            y_formatter: None,
            x_tick_formatter: None,
            continuous_x_tick_formatter: None,
            y_tick_formatter: None,
        }
    }

    pub fn palette(mut self, palette: impl Into<Signal<ColorPalette>>) -> Self {
        self.palette = Some(palette.into());
        self
    }

    pub fn with_series(mut self, series: impl Into<Signal<Vec<Series<X, Y>>>>) -> Self {
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

    pub fn continuous_x_tick_formatter(
        mut self,
        formatter: impl Fn(&f64) -> String + Send + Sync + 'static,
    ) -> Self {
        self.continuous_x_tick_formatter = Some(std::sync::Arc::new(formatter));
        self
    }

    pub fn y_tick_formatter(
        mut self,
        formatter: impl Fn(&f64) -> String + Send + Sync + 'static,
    ) -> Self {
        self.y_tick_formatter = Some(std::sync::Arc::new(formatter));
        self
    }

    pub fn build(self) -> LinePlotConfig<X, Y> {
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

        let x_formatter = self
            .x_formatter
            .unwrap_or_else(|| std::sync::Arc::new(|x: &X| x.to_plot_string()));

        let y_formatter = self
            .y_formatter
            .unwrap_or_else(|| std::sync::Arc::new(|y: &Y| y.to_plot_string()));

        let x_tick_formatter = self
            .x_tick_formatter
            .unwrap_or_else(|| std::sync::Arc::new(|x: &X| x.to_plot_string()));

        let continuous_x_tick_formatter = self
            .continuous_x_tick_formatter
            .unwrap_or_else(|| std::sync::Arc::new(|x: &f64| format!("{:.1}", x)));

        let y_tick_formatter = self
            .y_tick_formatter
            .unwrap_or_else(|| std::sync::Arc::new(|y: &f64| format!("{:.1}", y)));

        LinePlotConfig {
            palette,
            series,
            title,
            x_label,
            y_label,
            margin_top,
            margin_bottom,
            margin_left,
            margin_right,
            x_formatter,
            y_formatter,
            x_tick_formatter,
            continuous_x_tick_formatter,
            y_tick_formatter,
        }
    }
}

#[component]
pub fn LinePlot<X, Y>(
    config: Signal<LinePlotConfig<X, Y>>,
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

    let lines = move || {
        let width = resolved_width();
        let height = resolved_height();
        let conf = config.get();
        let series_vec = conf.series.get();
        let palette = conf.palette.get();
        let margin_top = conf.margin_top.get();
        let margin_bottom = conf.margin_bottom.get();
        let margin_left = conf.margin_left.get();
        let margin_right = conf.margin_right.get();

        let mut all_y = Vec::new();
        let mut all_x_f64 = Vec::new();
        let mut max_x_len = 0;

        let is_continuous = X::data_type() == DataType::Continuous;

        for s in &series_vec {
            let y_vals: Vec<Y> = s.get_y();
            let x_vals: Vec<X> = s.get_x();
            all_y.extend(y_vals.iter().map(|v| v.to_f64()));
            if is_continuous {
                all_x_f64.extend(x_vals.iter().map(|v| v.to_f64()));
            }
            if x_vals.len() > max_x_len {
                max_x_len = x_vals.len();
            }
        }

        let max_y = all_y
            .into_iter()
            .fold(0.0, |a, b| if a > b { a } else { b });
        let scale_y = if max_y == 0.0 {
            1.0
        } else {
            (height - margin_top - margin_bottom) / max_y
        };

        let max_x_f64 = if is_continuous {
            all_x_f64
                .into_iter()
                .fold(0.0, |a, b| if a > b { a } else { b })
        } else {
            0.0
        };

        let scale_x = if max_x_f64 == 0.0 {
            1.0
        } else {
            (width - margin_left - margin_right) / max_x_f64
        };

        let x_slot_width = if max_x_len > 1 {
            (width - margin_left - margin_right) / (max_x_len as f64 - 1.0).max(1.0)
        } else {
            0.0
        };

        let mut elements = Vec::new();

        if is_continuous {
            let num_ticks = 5;
            for i in 0..=num_ticks {
                let tick_val = (max_x_f64 / num_ticks as f64) * (i as f64);
                let x_pos = margin_left + (tick_val * scale_x);
                let formatted_tick = (conf.continuous_x_tick_formatter)(&tick_val);
                elements.push(
                    view! {
                        <line x1=x_pos y1=height - margin_bottom x2=x_pos y2=height - margin_bottom + 5.0 stroke="black" />
                        <text
                            x=x_pos
                            y=height - margin_bottom + 15.0
                            text-anchor="middle"
                            font-size="12"
                        >
                            {formatted_tick}
                        </text>
                    }
                    .into_any(),
                );
            }
        } else {
            // Optional: draw x ticks from the longest series
            // Here we just pick the first series that matches max_x_len to draw x-ticks
            if let Some(s) = series_vec.iter().find(|s| s.get_x().len() == max_x_len) {
                let x_vals: Vec<X> = s.get_x();
                for (i, x_val) in x_vals.iter().enumerate() {
                    let x_pos = margin_left + (i as f64) * x_slot_width;
                    let formatted_x_tick = (conf.x_tick_formatter)(x_val);
                    elements.push(
                        view! {
                            <line x1=x_pos y1=height - margin_bottom x2=x_pos y2=height - margin_bottom + 5.0 stroke="black" />
                            <text
                                x=x_pos
                                y=height - margin_bottom + 15.0
                                text-anchor="middle"
                                font-size="12"
                            >
                                {formatted_x_tick}
                            </text>
                        }
                        .into_any(),
                    );
                }
            }
        }

        for (series_idx, s) in series_vec.iter().enumerate() {
            let x_vals: Vec<X> = s.get_x();
            let y_vals: Vec<Y> = s.get_y();
            let color = palette.get_color(series_idx);

            let mut points_str = String::new();
            let mut points_elements = Vec::new();

            for (i, y_val) in y_vals.iter().enumerate() {
                let val_f64 = y_val.to_f64();
                let x_pos = if is_continuous {
                    margin_left + (x_vals[i].to_f64() * scale_x)
                } else {
                    margin_left + (i as f64) * x_slot_width
                };
                let y_pos = height - margin_bottom - (val_f64 * scale_y);

                points_str.push_str(&format!("{},{} ", x_pos, y_pos));

                let formatted_x = (conf.x_formatter)(&x_vals[i]);
                let formatted_y = (conf.y_formatter)(y_val);

                points_elements.push(view! {
                    <circle cx=x_pos cy=y_pos r=4 fill=color.to_string()>
                        <title>{format!("{}: {}", formatted_x, formatted_y)}</title>
                    </circle>
                });
            }

            elements.push(
                view! {
                    <polyline
                        points=points_str
                        fill="none"
                        stroke=color.to_string()
                        stroke-width="2"
                    />
                    {points_elements}
                }
                .into_any(),
            );
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
                    {lines}
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
    fn test_line_plot_builder() {
        let series1 = Series::new(vec!["A".to_string(), "B".to_string()], vec![1.0, 2.0]);
        let series2 = Series::new(vec!["C".to_string(), "D".to_string()], vec![3.0, 4.0]);

        let config = LinePlotConfig::builder()
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
}
