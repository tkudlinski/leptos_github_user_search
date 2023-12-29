use leptos::*;
use leptos::ev::MouseEvent;
#[component]
pub fn Button(
    #[prop(into)]
    label: String,
    #[prop(into)]
    on_click: Callback<MouseEvent>,
) -> impl IntoView {
    view! {
        <button on:click=on_click>{label}</button>
    }
}
