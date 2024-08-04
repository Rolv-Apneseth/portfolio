use leptos::*;

use crate::components::external_link::ExternalLink;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="flex flex-col gap-3 mt-10 text-sm italic leading-tight">
            <p>
                This site is built from scratch using
                <ExternalLink href="https://leptos.dev/".to_string() content="Leptos".to_string() />
                (Rust + WASM) and
                <ExternalLink
                    href="https://tailwindcss.com/".to_string()
                    content="Tailwind
       CSS,".to_string()
                />and hosted using
                <ExternalLink
                    href="https://pages.cloudflare.com/".to_string()
                    content="Cloudflare Pages.".to_string()
                />{" "}Check out the code by
                <ExternalLink
                    href="https://github.com/Rolv-Apneseth/portfolio".to_string()
                    content="clicking here!".to_string()
                />
            </p>
            <p>
                Style inspired by
                <ExternalLink
                    href="https://brittanychiang.com/".to_string()
                    content="brittanychiang.com.".to_string()
                />
            </p>
        </footer>
    }
}
