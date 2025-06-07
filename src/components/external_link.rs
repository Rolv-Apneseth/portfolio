use leptos::prelude::*;

#[component]
pub fn ExternalLink(#[prop()] href: String, #[prop()] content: String) -> impl IntoView {
    view! {
        <a
            class="not-italic font-bold duration-300 hover:underline motion-safe:transition-all hover:text-slate-950 hover:dark:text-slate-200"
            href=href
            target="_blank"
            rel="noreferrer noopener"
        >
            {content}
        </a>
    }
}
