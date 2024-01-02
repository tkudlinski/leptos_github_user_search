use leptos::{error::Result, *};
use leptos_reactive::watch;
use leptos_use::*;
use crate::components::Input::Input;
use crate::components::Button::Button;
use crate::components::Items::Items;
use crate::components::types::*;

const GITHUB_PER_PAGE: usize = 10;

async fn fetch_profiles(params: (String, usize)) -> Result<Vec<GithubProfile>> {
    let (value, page) = params;

    // make the request
    let res = reqwasm::http::Request::get(&format!(
        "https://api.github.com/search/users?q={value}&page={page}&per_page={GITHUB_PER_PAGE}",
    ))
    .send()
    .await?
    // convert it to JSON
    .json::<GithubResponse>()
    .await?;

    Ok(res.items)
}

#[component]
pub fn Container() -> impl IntoView {
    let (page, set_page) = create_signal(1);
    let (value, set_value) = create_signal("".to_string());
    let throttled: Signal<String> = signal_throttled_with_options(value, 3000.0, ThrottleOptions::default().leading(true).trailing(true));
    let (data, set_data) = create_signal::<Vec<GithubProfile>>(Vec::<GithubProfile>::new());

    let _stop = watch(
        move || throttled.get(),
        move |throttled, prev_throttled, _| {
            if throttled != prev_throttled.unwrap() {
                set_page(1);
                set_data(Vec::<GithubProfile>::new());
            }
        },
        false,
    );

    let items = create_local_resource(move || (throttled.get(), page.get()), fetch_profiles);

    create_effect(move |_| {
        items.and_then(|new_items| {
            set_data(new_items.to_vec());
        });
    });

    view! {
        <div class="max-w-md p-6 bg-white rounded-md shadow-md" style="
            max-height: 100vh;
            overflow-y: hidden;
            display: flex;
            flex-direction: column;
        ">
            <Input
                on_change=move |ev| {
                    let value = event_target_value(&ev);
                    set_value(value);
                }

                value=value.get()
            />
            <Items items=data/>
            <div class="flex justify-between" style="flex: 1;">
                {move || {
                    if page.get() > 1 {
                        view! {
                            <div>
                                <Button
                                    on_click=move |_| {
                                        set_page(page.get() - 1);
                                    }

                                    label="Prev"
                                />
                            </div>
                        }
                    } else {
                        view! { <div></div> }
                    }
                }}
                <Button
                    on_click=move |_| {
                        set_page(page.get() + 1);
                    }

                    label="Next"
                />
            </div>
        </div>
    }
}
