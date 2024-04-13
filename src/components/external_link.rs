use leptos::*;

#[component]
pub fn ExternalLink(#[prop()] href: String, #[prop()] content: String) -> impl IntoView {
    view! {
        <a
            class="font-bold duration-300 hover:text-black hover:underline motion-safe:transition-all hover:dark:text-slate-200"
            href=href
            target="_blank"
            rel="noreferrer noopener"
        >
            {content}
        </a>
    }
}
