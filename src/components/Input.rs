use leptos::*;
use leptos::ev::Event;

#[component]
pub fn Input(
    value: String,
    #[prop(into)]
    on_change: Callback<Event>,
) -> impl IntoView {
    view! {
        <div class="mb-4" style="flex: 1;">
            <input
                type="text"
                on:input=on_change
                class="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring focus:border-blue-300"
                // the `prop:` syntax lets you update a DOM property,
                // rather than an attribute.
                prop:value=value
            />
        </div>
    }
}
