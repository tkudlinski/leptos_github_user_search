use leptos::*;
use leptos_meta::*;
use crate::components::Container::Container;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Container />
    }
}