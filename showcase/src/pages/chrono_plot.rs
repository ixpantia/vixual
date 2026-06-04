use chrono::{Duration, Utc};
use leptos::prelude::*;
use vixual::line_plot::{LinePlot, LinePlotConfig};
use vixual::series::Series;

#[component]
pub fn ChronoPlotPage() -> impl IntoView {
    let now = Utc::now();

    let series = RwSignal::new(vec![
        Series::new(
            vec![
                now - Duration::days(4),
                now - Duration::days(3),
                now - Duration::days(2),
                now - Duration::days(1),
                now,
            ],
            vec![10.0, 15.0, 8.0, 12.0, 20.0],
        ),
        Series::new(
            vec![
                now - Duration::days(4),
                now - Duration::days(3),
                now - Duration::days(2),
                now - Duration::days(1),
                now,
            ],
            vec![5.0, 8.0, 12.0, 10.0, 15.0],
        ),
    ]);

    let config = LinePlotConfig::builder()
        .with_series(series)
        .title("Chrono DateTime Support")
        .x_label("Date")
        .y_label("Value")
        // Use a custom formatter for the x-axis ticks to show only date
        .continuous_x_tick_formatter(|v| {
            chrono::DateTime::from_timestamp_millis(*v as i64)
                .unwrap_or_default()
                .format("%Y-%m-%d")
                .to_string()
        })
        .build();

    let config_sig = Signal::derive(move || config.clone());

    view! {
        <div style="padding: 40px; font-family: sans-serif;">
            <h1>"Chrono DateTime Support"</h1>
            <p>"This page demonstrates using chrono::DateTime<Utc> as the X-axis data type."</p>

            <div style="margin-top: 20px; background: white; padding: 20px; border: 1px solid #eee; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.05);">
                <div style="height: 400px; border: 1px dashed #ccc;">
                    <LinePlot config=config_sig />
                </div>
            </div>
        </div>
    }
}
