use leptos::*;

#[component]
pub fn Item(value: String, img_src: String) -> impl IntoView {
    view! {
        <div>
            <img src=img_src/>
            <span>{value}</span>
        </div>
    }
}
