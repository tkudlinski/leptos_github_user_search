use leptos::*;

#[component]
pub fn Item(value: String, img_src: String) -> impl IntoView {
    view! {
        <li class="text-gray-800 mb-2">
            <img src=img_src/>
            <span>{value}</span>
        </li>
    }
}
