use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use reactive_stores::Store;

use crate::components::third::SomePage;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[derive(Debug, Store, Default)]
pub struct StoreData {
    pub day: bool,
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let store = Store::new(StoreData::default());
    provide_context(store);

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-07-callback.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>
        <Suspense>
        { move || store.day().get() }
        </Suspense>
        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
        <SomePage/>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let store =
        use_context::<Store<StoreData>>().expect("App is not loaded. Store context not found!");

    // Creates a reactive value to update the button
    let on_click = move |_| {
        leptos::logging::log!("{:?}", store.day().get());
        store.day().set(!store.day().get());
        leptos::logging::log!("{:?}", store.day().get());
    };

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <Suspense>
        <b>{ move || store.day().get()}</b>
        </Suspense>
        <button on:click=on_click>"Click Me: "</button>
    }
}

// Button B receives a closure
#[component]
pub fn ButtonB(#[prop(into)] on_click: Callback<leptos::ev::MouseEvent>) -> impl IntoView {
    view! {
        <button on:click=move |e| on_click.run(e)>
            "Toggle"
        </button>
    }
}
