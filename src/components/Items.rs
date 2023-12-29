use crate::components::types::*;
use crate::components::Item::Item;
use leptos::*;

#[component]
pub fn Items(items: ReadSignal<Vec<GithubProfile>>) -> impl IntoView {
    view! {
        <ul>
            <For
                each=items
                key=|item| (*item).id
                children=move |item| {
                    view! {
                        <Item value=item.login img_src=item.avatar_url />
                    }
                }
            />

        </ul>
    }
}
