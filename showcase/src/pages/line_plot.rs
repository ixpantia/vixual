use leptos::prelude::*;
use rand::Rng;
use vixual::legend::Legend;
use vixual::line_plot::{LinePlot, LinePlotConfig};
use vixual::palette::ColorPalette;
use vixual::series::Series;

fn generate_random_raw_data() -> (Vec<f64>, Vec<f64>) {
    let mut rng = rand::thread_rng();
    let num_points = rng.gen_range(5..15);

    let mut x_vals: Vec<f64> = (0..num_points)
        .map(|i| i as f64 + rng.gen_range(0.0..1.0))
        .collect();
    x_vals.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let values: Vec<f64> = (0..num_points).map(|_| rng.gen_range(5.0..30.0)).collect();
    (x_vals, values)
}

fn generate_random_data(label: &str) -> Series<f64, f64> {
    let (labels, values) = generate_random_raw_data();
    Series::with_label(labels, values, label)
}

#[component]
pub fn LinePlotPage() -> impl IntoView {
    let series = RwSignal::new(vec![
        Series::with_label(
            vec![1.0, 2.0, 3.0, 4.0, 5.0],
            vec![15.0, 22.0, 18.0, 25.0, 20.0],
            "Series A",
        ),
        Series::with_label(
            vec![1.0, 2.0, 3.0, 4.0, 5.0],
            vec![12.0, 25.0, 14.0, 18.0, 28.0],
            "Series B",
        ),
    ]);
    let palette = RwSignal::new(ColorPalette::default());
    let title = RwSignal::new("Data Trends".to_string());
    let x_label = RwSignal::new("Time".to_string());
    let y_label = RwSignal::new("Quantity".to_string());
    let margin_top = RwSignal::new(40.0);
    let margin_bottom = RwSignal::new(40.0);
    let margin_left = RwSignal::new(60.0);
    let margin_right = RwSignal::new(20.0);

    let config = LinePlotConfig::builder()
        .with_series(series)
        .palette(palette)
        .title(title)
        .x_label(x_label)
        .y_label(y_label)
        .margin_top(margin_top)
        .margin_bottom(margin_bottom)
        .margin_left(margin_left)
        .margin_right(margin_right)
        .x_tick_formatter(|v| format!("{v:.2}"))
        .build();

    let config_sig = Signal::stored(config);

    let on_palette_change = move |ev| {
        let val = event_target_value(&ev);
        palette.get_untracked().set_from_presets(&val);
    };

    let color_input_ref = NodeRef::<leptos::html::Input>::new();

    let on_add_color = move |_| {
        if let Some(input) = color_input_ref.get() {
            let val = input.value();
            if let Some(hex) = vixual::palette::Hex::from_hex_str(&val) {
                palette
                    .get_untracked()
                    .colors
                    .update(|colors| colors.push(hex));
            }
        }
    };

    let on_add_series = move |_| {
        let current_count = series.get().len();
        let label = format!("Series {}", (b'A' + (current_count % 26) as u8) as char);
        let new_series = generate_random_data(&label);
        series.update(|s| s.push(new_series));
    };

    view! {
        <div style="display: flex; min-height: 100vh; font-family: sans-serif; width: 100%;">
            // Sidebar
            <div style="width: 300px; background: #f4f4f4; padding: 20px; border-right: 1px solid #ddd; box-sizing: border-box; overflow-y: auto; flex-shrink: 0;">
                <h3>"Plot Customization"</h3>

                <section style="margin-bottom: 25px;">
                    <h4>"General"</h4>
                    <div style="margin-bottom: 10px;">
                        <label style="display: block; font-size: 0.9em; margin-bottom: 5px;">
                            "Title"
                        </label>
                        <input
                            type="text"
                            prop:value=title
                            on:input=move |ev| title.set(event_target_value(&ev))
                            style="width: 100%; padding: 5px; border: 1px solid #ccc; border-radius: 4px; box-sizing: border-box;"
                        />
                    </div>
                    <div style="margin-bottom: 10px;">
                        <label style="display: block; font-size: 0.9em; margin-bottom: 5px;">
                            "X Label"
                        </label>
                        <input
                            type="text"
                            prop:value=x_label
                            on:input=move |ev| x_label.set(event_target_value(&ev))
                            style="width: 100%; padding: 5px; border: 1px solid #ccc; border-radius: 4px; box-sizing: border-box;"
                        />
                    </div>
                    <div style="margin-bottom: 10px;">
                        <label style="display: block; font-size: 0.9em; margin-bottom: 5px;">
                            "Y Label"
                        </label>
                        <input
                            type="text"
                            prop:value=y_label
                            on:input=move |ev| y_label.set(event_target_value(&ev))
                            style="width: 100%; padding: 5px; border: 1px solid #ccc; border-radius: 4px; box-sizing: border-box;"
                        />
                    </div>
                </section>

                <section style="margin-bottom: 25px;">
                    <h4>"Margins"</h4>
                    <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 10px;">
                        <div>
                            <label style="display: block; font-size: 0.8em; margin-bottom: 3px;">
                                "Top"
                            </label>
                            <input
                                type="number"
                                prop:value=margin_top
                                on:input=move |ev| {
                                    if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                                        margin_top.set(val);
                                    }
                                }
                                style="width: 100%; padding: 4px; border: 1px solid #ccc; border-radius: 4px; box-sizing: border-box;"
                            />
                        </div>
                        <div>
                            <label style="display: block; font-size: 0.8em; margin-bottom: 3px;">
                                "Bottom"
                            </label>
                            <input
                                type="number"
                                prop:value=margin_bottom
                                on:input=move |ev| {
                                    if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                                        margin_bottom.set(val);
                                    }
                                }
                                style="width: 100%; padding: 4px; border: 1px solid #ccc; border-radius: 4px; box-sizing: border-box;"
                            />
                        </div>
                        <div>
                            <label style="display: block; font-size: 0.8em; margin-bottom: 3px;">
                                "Left"
                            </label>
                            <input
                                type="number"
                                prop:value=margin_left
                                on:input=move |ev| {
                                    if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                                        margin_left.set(val);
                                    }
                                }
                                style="width: 100%; padding: 4px; border: 1px solid #ccc; border-radius: 4px; box-sizing: border-box;"
                            />
                        </div>
                        <div>
                            <label style="display: block; font-size: 0.8em; margin-bottom: 3px;">
                                "Right"
                            </label>
                            <input
                                type="number"
                                prop:value=margin_right
                                on:input=move |ev| {
                                    if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                                        margin_right.set(val);
                                    }
                                }
                                style="width: 100%; padding: 4px; border: 1px solid #ccc; border-radius: 4px; box-sizing: border-box;"
                            />
                        </div>
                    </div>
                </section>

                <section style="margin-bottom: 25px;">
                    <h4>"Colors"</h4>
                    <div style="margin-bottom: 10px;">
                        <label style="display: block; font-size: 0.9em; margin-bottom: 5px;">
                            "Preset"
                        </label>
                        <select on:change=on_palette_change style="width: 100%;">
                            <option value="classic">"Classic"</option>
                            <option value="modern">"Modern"</option>
                            <option value="grayscale">"Grayscale"</option>
                        </select>
                    </div>
                    <div style="margin-bottom: 10px;">
                        <label style="display: block; font-size: 0.9em; margin-bottom: 5px;">
                            "Add Color"
                        </label>
                        <div style="display: flex; gap: 5px;">
                            <input
                                type="color"
                                node_ref=color_input_ref
                                style="flex-grow: 1; height: 30px; padding: 2px; border: 1px solid #ccc; border-radius: 4px;"
                            />
                            <button on:click=on_add_color>"Add"</button>
                        </div>
                    </div>

                    <div style="margin-top: 15px;">
                        <label style="display: block; font-size: 0.9em; margin-bottom: 5px;">
                            "Current Palette"
                        </label>
                        <ul style="list-style: none; padding: 0; margin: 0;">
                            <For
                                each=move || {
                                    palette
                                        .get()
                                        .colors
                                        .get()
                                        .into_iter()
                                        .enumerate()
                                        .map(|(idx, c)| (idx, c, c.to_string()))
                                        .collect::<Vec<_>>()
                                }
                                key=|item| (item.0, item.2.clone())
                                children=move |(idx, color, _)| {
                                    let c_str = color.to_string();
                                    view! {
                                        <li style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 5px; padding: 5px; background: white; border: 1px solid #eee; border-radius: 4px;">
                                            <div style="display: flex; align-items: center; gap: 8px;">
                                                <div style=format!(
                                                    "width: 16px; height: 16px; background: {}; border-radius: 2px;",
                                                    c_str,
                                                ) />
                                                <span style="font-size: 0.8em; font-family: monospace;">
                                                    {c_str.clone()}
                                                </span>
                                            </div>
                                            <button
                                                style="color: #cc0000; border: none; background: transparent; cursor: pointer; font-size: 1.2em; line-height: 1;"
                                                on:click=move |_| {
                                                    palette
                                                        .get_untracked()
                                                        .colors
                                                        .update(|colors| {
                                                            if colors.len() > idx {
                                                                colors.remove(idx);
                                                            }
                                                        });
                                                }
                                            >
                                                "×"
                                            </button>
                                        </li>
                                    }
                                }
                            />
                        </ul>
                    </div>
                </section>

                <hr style="border: 0; border-top: 1px solid #ddd; margin: 20px 0;" />

                <section>
                    <h4>"Series"</h4>
                    <button
                        on:click=on_add_series
                        style="width: 100%; padding: 8px; margin-bottom: 10px;"
                    >
                        "Add Random Series"
                    </button>

                    <label style="display: block; font-size: 0.9em; margin-bottom: 5px;">
                        "Active Series"
                    </label>
                    <ul style="list-style: none; padding: 0; margin: 0;">
                        <For
                            each=move || 0..series.get().len()
                            key=|idx| *idx
                            children=move |idx| {
                                view! {
                                    <li style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 5px; padding: 5px; background: white; border: 1px solid #eee; border-radius: 4px;">
                                        <span style="font-size: 0.8em;">"Series " {idx + 1}</span>
                                        <div style="display: flex; gap: 8px; align-items: center;">
                                            <button
                                                style="color: #0066cc; border: none; background: transparent; cursor: pointer; font-size: 1.2em; line-height: 1;"
                                                on:click=move |_| {
                                                    let (labels, values) = generate_random_raw_data();
                                                    series.get()[idx].replace_data(labels, values);
                                                }
                                                title="Re-generate data"
                                            >
                                                "↻"
                                            </button>
                                            <button
                                                style="color: #cc0000; border: none; background: transparent; cursor: pointer; font-size: 1.2em; line-height: 1;"
                                                on:click=move |_| {
                                                    series
                                                        .update(|s| {
                                                            if s.len() > idx {
                                                                s.remove(idx);
                                                            }
                                                        });
                                                }
                                                title="Remove series"
                                            >
                                                "×"
                                            </button>
                                        </div>
                                    </li>
                                }
                            }
                        />
                    </ul>
                </section>
            </div>

            // Main Content
            <div style="flex-grow: 1; padding: 40px; overflow-y: auto;">
                <h1>"vixual Line Plot Showcase"</h1>
                <p>"Use the sidebar to customize the plot data and appearance."</p>

                <div style="margin-top: 20px; background: white; padding: 20px; border: 1px solid #eee; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.05);">
                    <h2>"Responsive Line Plot (100% parent)"</h2>
                    <div style="height: 400px; border: 1px dashed #ccc;">
                        <LinePlot config=config_sig />
                    </div>
                    <div style="margin-top: 15px;">
                        <h3 style="margin-bottom: 10px; font-size: 14px;">"Legend"</h3>
                        <Legend config=config_sig />
                    </div>
                </div>

            </div>
        </div>
    }
}
