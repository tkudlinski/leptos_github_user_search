use leptos::ev::MouseEvent;
use leptos::*;
#[component]
pub fn Button(
    #[prop(into)] label: String,
    #[prop(into)] on_click: Callback<MouseEvent>,
) -> impl IntoView {
    view! {
        <button
            class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:ring focus:border-blue-300"
            on:click=on_click
        >
            {label}
        </button>
    }
}
