use leptos::prelude::*;
use reactive_stores::Store;

use crate::app::StoreData;
#[component]
pub fn SomePage() -> impl IntoView {
    let store =
        use_context::<Store<StoreData>>().expect("App is not loaded. Store context not found!");

    // Creates a reactive value to update the button
    let on_click = move |_| {
        leptos::logging::log!("{:?}", store.day().get());
        store.day().set(!store.day().get());
        leptos::logging::log!("{:?}", store.day().get());
    };

    view! {
        <h1>"Some page!"</h1>
        <Suspense>
        <b>{ move || store.day().get()}</b>
        </Suspense>
        <button on:click=on_click>"Click Me: "</button>
    }
}
