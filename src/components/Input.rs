use leptos::*;
use leptos::ev::Event;

#[component]
pub fn Input(
    value: String,
    #[prop(into)]
    on_change: Callback<Event>,
) -> impl IntoView {
    view! {
        <input
            type="text"
            on:input=on_change
            // the `prop:` syntax lets you update a DOM property,
            // rather than an attribute.
            prop:value=value
        />
    }
}
