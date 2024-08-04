use leptos::*;
use leptos_icons::Icon;

use crate::components::tag::{
    Tags,
    Tech,
};

pub struct ExperienceData {
    workplace: String,
    workplace_href: String,
    position: String,
    description: Vec<String>,
    range: [String; 2],
    techs: Vec<Tech>,
}
impl ExperienceData {
    pub fn new(
        workplace: &str,
        workplace_href: &str,
        position: &str,
        description: &[&str],
        range: [&str; 2],
        techs: impl IntoIterator<Item = Tech>,
    ) -> Self {
        Self {
            workplace: workplace.to_string(),
            workplace_href: workplace_href.to_string(),
            position: position.to_string(),
            description: description.iter().map(|s| s.to_string()).collect(),
            range: [range[0].to_string(), range[1].to_string()],
            techs: techs.into_iter().collect(),
        }
    }
}

#[component]
pub fn Experience(#[prop()] data: ExperienceData) -> impl IntoView {
    view! {
        <li class="relative">

            <a
                href=data.workplace_href
                target="_blank"
                rel="noreferrer noopener"
                class="grid gap-1 py-6 px-3 rounded-md duration-300 sm:grid-cols-8 motion-reduce:transition-none lg:hover:!opacity-100 lg:group-hover/list:opacity-50 lg:hover:bg-slate-300/50 lg:dark:hover:bg-slate-800/50 lg:hover:shadow[inset_0_1px_0_0_rgba(148,163,184,0.1)] lg:hover:drop-shadow-lg"
            >
                <header class="z-10 m-0 text-xs font-semibold tracking-wide uppercase sm:col-span-2 text-slate-500 dark:text-slate-500">
                    {format!("{} — {}", data.range[0], data.range[1])}
                </header>

                <div class="flex z-10 flex-col gap-3 sm:col-span-6">
                    <h4 class="text-sm font-medium leading-snug text-slate-800 dark:text-slate-200">
                        <span class="">{data.position}</span>
                        {" · "}
                        <span class="">{data.workplace}</span>
                    </h4>

                    <ul class="flex flex-col gap-2 list-disc ps-4">
                        {data
                            .description
                            .iter()
                            .map(|d| {
                                view! { <li class="text-sm leading-normal">{d}</li> }
                            })
                            .collect::<Vec<_>>()}
                    </ul>

                    <Tags techs=data.techs />
                </div>
            </a>
        </li>
    }
}

#[component]
pub fn ExperienceSection() -> impl IntoView {
    view! {
        <ol class="flex flex-col gap-4 mt-3 mb-6 text-xs group/list">
            <Experience data=ExperienceData::new(
                "Kinesense Ltd.",
                "https://www.kinesense-vca.com/",
                "Junior Frontend Developer",
                &[
                    "Develop the front end for re-creating the company’s flagship desktop application (a complex video analysis tool) as a web-based platform which will service clients from around the world.",
                    "Migrated all old class-based React components to more maintainable and extendable functional compo- nents, also fixing critical bugs along the way.",
                    "Leverage Redux to manage intricate client-side state and RTKQuery for querying and caching API responses from the back end, allowing for easy extension of required state, and simpler debugging of related issues.",
                    "Create comprehensive unit tests using Jest, and maintain Azure DevOps CI/CD workflows, ensuring the reliability of the web application.",
                ],
                ["2022", "Present"],
                [
                    Tech::Ts,
                    Tech::React,
                    Tech::Redux,
                    Tech::RTKQuery,
                    Tech::Az,
                    Tech::Jest,
                    Tech::Bs,
                    Tech::Scss,
                ],
            ) />
            <Experience data=ExperienceData::new(
                "Bottletop Media",
                "https://iclass-cms.com/",
                "Junior Frontend Developer",
                &[
                    "Updated and maintained websites for over 1000 existing clients using modern HTML, CSS, and JavaScript.",
                    "Implemented unique designs and features upon request, leading to increased customer satisfaction.",
                    "Developed a custom Selenium (browser automation) script in Python to automate the creation of page structures on the company’s proprietary CMS software for new clients’ websites, significantly speeding up the initial development stages.",
                ],
                ["2021", "2022"],
                [Tech::Js, Tech::Html, Tech::Css, Tech::Py],
            ) />
        </ol>

        <a
            class="inline-block overflow-hidden relative py-3 px-8 font-medium font-semibold leading-tight rounded border border-violet-400 duration-300 dark:border-purple-500 focus:outline-none group before:w-0 before:absolute before:left-0 before:inset-y-0 before:bg-violet-800 before:dark:bg-purple-600 before:motion-safe:transition-all after:duration-300 hover:border-violet-400/20 hover:dark:border-purple-600 hover:before:w-full"
            href="/public/rolvApnesethCV.pdf"
            target="_blank"
            rel="noreferrer noopener"
        >
            <span class="flex relative gap-3 items-center text-sm text-violet-800 transition-colors duration-300 dark:text-purple-500 group-hover:text-slate-100">
                <Icon icon=icondata::ImProfile />
                View CV / Résumé
            </span>
        </a>
    }
}
