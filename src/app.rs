use leptos::prelude::*;
use leptos_darkmode::Darkmode;
use leptos_meta::*;

use crate::{
    components::theme_toggle::ThemeToggle,
    sections::{
        contact::Contact,
        contributions::ContributionsSection,
        experience::ExperienceSection,
        footer::Footer,
        nav::{
            Nav,
            Sections,
        },
        projects::{
            ArchivedProjectsSection,
            ProjectsSection,
        },
        section::{
            Section,
            SectionData,
        },
    },
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let sections: Sections = vec![
        ("experience".to_string(), signal(false)),
        ("projects".to_string(), signal(false)),
        ("archive".to_string(), signal(false)),
        ("contributions".to_string(), signal(false)),
    ];
    let darkmode = Darkmode::init();

    let sections_views = sections
        .iter()
        .map(|(label, is_visible)| {
            let child = match label.as_str() {
                "experience" => view! { <ExperienceSection /> }.into_any(),
                "contributions" => view! { <ContributionsSection /> }.into_any(),
                "projects" => view! { <ProjectsSection /> }.into_any(),
                "archive" => view! { <ArchivedProjectsSection /> }.into_any(),
                _ => unreachable!("unknown label"),
            };

            view! {
                <Section section_data=SectionData::new(label) is_visible=is_visible.1>
                    {child}
                </Section>
            }
        })
        .collect_view();

    view! {
        <Html
            {..}
            lang="en"
            class=move || if darkmode.is_dark() { "scroll-smooth dark" } else { "scroll-smooth" }
        />

        <Body
            {..}
            class="relative py-12 px-6 mx-auto max-w-screen-xl min-h-screen antialiased leading-relaxed duration-300 md:py-20 md:px-12 lg:py-0 lg:px-24 bg-slate-200 transition-bg-color motion-reduce:transition-none dark:bg-primary dark:text-slate-400 dark:selection:bg-teal-300 dark:selection:text-teal-900"
        />

        <ThemeToggle />

        <div class="min-h-screen lg:flex lg:justify-between lg:gap-30">

            <header class="lg:flex lg:sticky lg:top-0 lg:flex-col lg:justify-between lg:py-24 lg:w-1/2 lg:max-h-screen">
                <section>
                    <h1 class="text-4xl font-bold tracking-tight sm:text-5xl dark:text-slate-200">
                        Rolv Apneseth
                    </h1>
                    <h2 class="mt-3 text-lg font-medium tracking-tight sm:text-xl dark:text-slate-200">
                        Software Developer
                    </h2>
                    <p class="mt-4 max-w-xs leading-normal">
                        A proactive learner with a strong work ethic, seeking to apply my knowledge and skills
                    </p>
                    <Nav sections=sections />
                </section>
                <Contact />
            </header>

            <main class="flex flex-col gap-16 pt-24 lg:py-24 lg:w-1/2">
                {sections_views} <Footer />
            </main>
        </div>
    }
}
