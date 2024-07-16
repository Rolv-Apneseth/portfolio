use leptos::*;

mod app;
mod components;
mod sections;
mod utils;

use crate::app::App;

fn main() {
    mount_to_body(move || {
        view! { <App /> }
    })
}
