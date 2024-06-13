use leptos::*;

#[component]
pub fn NavLink(#[prop()] title: String, #[prop()] is_visible: ReadSignal<bool>) -> impl IntoView {
    view! {
        <li class=move || if is_visible.get() { "section-in-view" } else { "" }>
            <a
                class="flex relative gap-1 items-center py-1 w-max text-lg capitalize duration-300 lg:text-xl hover:text-black motion-safe:transition-text-color after:absolute after:bottom-0 after:left-0 after:w-0 after:bg-slate-950 after:h-[2px] after:hover:w-full after:motion-safe:transition-width after:duration-300 before:content-['->'] before:text-violet-600 before:dark:text-purple-500 before:font-bold dark:hover:text-slate-200 dark:after:bg-slate-200"
                href=format!("#{}", title.to_lowercase())
            >
                {title}
            </a>
        </li>
    }
}

#[component]
pub fn Nav(
    #[prop()] sections: Vec<(String, (ReadSignal<bool>, WriteSignal<bool>))>,
) -> impl IntoView {
    let links_views = sections
        .into_iter()
        .map(|(label, is_visible)| {
            view! { <NavLink title=label is_visible=is_visible.0/> }
        })
        .collect_view();

    view! {
        <nav class="hidden lg:block nav" aria-label="Page section links">
            <ul class="flex flex-col gap-1 mt-16 w-max">{links_views}</ul>
        </nav>
    }
}
