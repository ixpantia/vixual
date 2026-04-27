use leptos::prelude::*;
use leptos_meta::{HashedStylesheet, MetaTags, Title, provide_meta_context};
use leptos_router::{
    components::{A, Outlet, ParentRoute, Route, Router, Routes},
    path,
};

use crate::pages::bar_plot::BarPlotPage;
use crate::pages::home::HomePage;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options=options.clone() />
                <HashedStylesheet options=options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; min-height: 100vh; font-family: sans-serif;">
            <header style="background: #333; color: white; padding: 15px 20px; display: flex; align-items: center; justify-content: space-between;">
                <h2 style="margin: 0;">"vixual Showcase"</h2>
                <nav style="display: flex; gap: 15px;">
                    <A href="/" exact=true>"Home"</A>
                    <A href="/bar-plot">"Bar Plot"</A>
                </nav>
            </header>

            <style>
                "nav a { color: white; text-decoration: none; padding: 5px 10px; border-radius: 4px; transition: background 0.2s; }"
                "nav a:hover { background: rgba(255,255,255,0.1); }"
                "nav a[aria-current=\"page\"] { background: rgba(255,255,255,0.2); font-weight: bold; }"
            </style>

            <main style="flex-grow: 1; display: flex; flex-direction: column;">
                <Outlet />
            </main>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // sets the document title
        <Title text="vixual Showcase" />

        <Router>
            <Routes fallback=|| "Page not found.".into_view()>
                <ParentRoute path=path!("") view=Layout>
                    <Route path=path!("") view=HomePage />
                    <Route path=path!("bar-plot") view=BarPlotPage />
                </ParentRoute>
            </Routes>
        </Router>
    }
}
