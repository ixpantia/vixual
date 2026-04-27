use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div style="padding: 40px;">
            <h1>"Welcome to vixual Showcase"</h1>
            <p>"Select a plot type from the navigation menu to explore its features."</p>
        </div>
    }
}
