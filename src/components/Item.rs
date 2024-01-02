use leptos::*;

#[component]
pub fn Item(value: String, img_src: String) -> impl IntoView {
    view! {
        <li class="bg-white rounded-md overflow-hidden shadow-md p-4">
            <div class="flex items-center mb-3">
                <img src=img_src class="w-12 h-12 rounded-full mr-2"/>
                <span class="text-gray-800 font-semibold">{value}</span>
            </div>
        </li>
    }
}
