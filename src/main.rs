use leptos::*;
mod components;
use log::Level;

fn main() {
    console_log::init_with_level(Level::Debug);
    mount_to_body(|| view! { <components::App::App /> })
}