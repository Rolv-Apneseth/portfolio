use leptos::prelude::*;

use crate::components::external_link::ExternalLink;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="flex flex-col gap-3 mt-10 text-sm italic leading-tight">
            <p>
                This site is built from scratch using
                <ExternalLink href="https://leptos.dev/".to_owned() content="Leptos".to_owned() />
                (Rust + WASM) and
                <ExternalLink
                    href="https://tailwindcss.com/".to_owned()
                    content="Tailwind
       CSS,".to_owned()
                />and hosted using
                <ExternalLink
                    href="https://pages.cloudflare.com/".to_owned()
                    content="Cloudflare Pages.".to_owned()
                />{" "}Check out the code by
                <ExternalLink
                    href="https://github.com/Rolv-Apneseth/portfolio".to_owned()
                    content="clicking here!".to_owned()
                />
            </p>
            <p>
                Style inspired by
                <ExternalLink
                    href="https://brittanychiang.com/".to_owned()
                    content="brittanychiang.com.".to_owned()
                />
            </p>
        </footer>
    }
}
