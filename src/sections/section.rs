use leptos::{
    html,
    prelude::*,
};
use leptos_use::use_intersection_observer;

use crate::utils::capitalise;

pub struct SectionData {
    title: String,
    id: String,
}
impl SectionData {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            title: capitalise(id),
        }
    }
}

#[component]
pub fn Section(
    #[prop()] section_data: SectionData,
    #[prop()] is_visible: WriteSignal<bool>,
    children: Children,
) -> impl IntoView {
    let el = NodeRef::<html::Section>::new();

    use_intersection_observer(el, move |entries, _| {
        is_visible.set(entries[0].is_intersecting());
    });

    view! {
        <section node_ref=el id=section_data.id>
            <h3 class="flex gap-4 items-center mb-2 text-2xl lg:text-3xl after:inline-block after:flex-grow after:h-[1px] text-slate-800 after:bg-slate-800 after:dark:bg-slate-200 dark:text-slate-200">
                {section_data.title}
            </h3>
            {children()}
        </section>
    }
}
