use leptos::prelude::*;

mod app;
mod components;
mod sections;
mod utils;

use crate::app::App;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App)
}
