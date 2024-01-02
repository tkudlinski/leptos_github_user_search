use crate::components::types::*;
use crate::components::Item::Item;
use leptos::*;

#[component]
pub fn Items(items: ReadSignal<Vec<GithubProfile>>) -> impl IntoView {
    view! {
        <ul class="mb-4" style="overflow: auto;">
            <Suspense fallback=move || {
                view! {
                    <div
                        class="absolute inset-0 flex items-center justify-center bg-white opacity-75"
                    >
                        <div class="spinner"></div>
                    </div>
                }
            }>
                <For
                    each=items
                    key=|item| (*item).id
                    children=move |item| {
                        view! { <Item value=item.login img_src=item.avatar_url/> }
                    }
                />

            </Suspense>
        </ul>
    }
}
