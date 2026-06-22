use crate::palette::Hex;
use leptos::prelude::*;

/// A single item in a legend, containing a label and its associated color.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct LegendItem {
    pub label: String,
    pub color: Hex,
}

/// Trait for types that can provide legend data.
/// This is automatically implemented for plot configurations.
pub trait LegendSource: Send + Sync + Clone + 'static {
    fn get_legend_items(&self) -> Vec<LegendItem>;
}

#[component]
pub fn Legend<S>(
    config: Signal<S>,
    #[prop(optional, into)] width: Option<Signal<String>>,
    #[prop(optional, into)] height: Option<Signal<String>>,
) -> impl IntoView
where
    S: LegendSource,
{
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
            .unwrap_or_else(|| "auto".to_string())
    };

    let items = move || config.get().get_legend_items();

    view! {
        <div
            style:width=svg_width
            style:height=svg_height
            style="display: flex; flex-wrap: wrap; gap: 10px; align-items: center;"
        >
            <For each=items key=|item| item.clone() let:item>
                {
                    view! {
                        <div style="display: flex; align-items: center; gap: 6px;">
                            <div style=format!(
                                "width: 16px; height: 16px; background-color: {}; border-radius: 3px;",
                                item.color.to_string(),
                            ) />
                            <span style="font-size: 14px; color: #333;">{item.label.clone()}</span>
                        </div>
                    }
                }
            </For>
        </div>
    }
}
