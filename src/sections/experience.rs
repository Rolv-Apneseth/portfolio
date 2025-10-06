use leptos::prelude::*;
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
            workplace: workplace.to_owned(),
            workplace_href: workplace_href.to_owned(),
            position: position.to_owned(),
            description: description
                .iter()
                .map(std::string::ToString::to_string)
                .collect(),
            range: [range[0].to_owned(), range[1].to_owned()],
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
                class="grid gap-1 py-6 px-3 rounded-md duration-300 sm:grid-cols-8 motion-reduce:transition-none dark:hover:bg-slate-800/50 group-hover/list:opacity-50 hover:!opacity-100 hover:bg-slate-300/20 hover:shadow[inset_0_1px_0_0_rgba(148,163,184,0.1)] hover:drop-shadow-lg"
            >
                <header class="z-10 m-0 text-xs font-semibold tracking-wide uppercase sm:col-span-2 text-slate-600 dark:text-slate-500">
                    {format!("{} — {}", data.range[0], data.range[1])}
                </header>

                <div class="flex z-10 flex-col gap-3 sm:col-span-6">
                    <h4 class="text-sm font-semibold leading-snug text-slate-800 dark:text-slate-200">
                        <span class="">{data.position}</span>
                        {" · "}
                        <span class="">{data.workplace}</span>
                    </h4>

                    <ul class="flex flex-col gap-2 list-disc ps-4">
                        {data
                            .description
                            .iter()
                            .map(|d| {
                                view! { <li class="text-sm leading-normal">{d.to_owned()}</li> }
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
                    "Primary developer for the front-end  of a web-based video investigation application built using React, adapting the company's flagship desktop application to serve law enforcement and security clients globally in a more accessible format.",
                    "Collaborated closely with the senior developer in charge of the project on architectural decisions, ensuring seamless integration between front-end and back-end systems. In addition, conducted thorough code reviews for back-end changes, both to identify potential bugs and to remain up-to-date on relevant functionality.",
                    "Built maintainable custom components (e.g. interactive video timeline with events, video overlays for motion detection, progress indicators for video import and analysis) using modern React and CSS features to minimise complexity while still enhancing user experience.",
                    "Leveraged Redux for state management and RTKQuery for efficient API handling, ensuring scalability and robust debugging capabilities.",
                    "Developed comprehensive unit tests with Jest and maintain Azure DevOps CI/CD pipelines, ensuring high code quality and reliability.",
                ],
                ["Nov 2022", "Apr 2025"],
                [
                    Tech::Ts,
                    Tech::React,
                    Tech::Redux,
                    Tech::RTKQuery,
                    Tech::Html,
                    Tech::Css,
                    Tech::Scss,
                    Tech::Bs,
                    Tech::Az,
                    Tech::Jest,
                ],
            ) />
            <Experience data=ExperienceData::new(
                "Bottletop Media",
                "https://iclass-cms.com/",
                "Junior Frontend Developer",
                &[
                    "Optimised and maintained websites for over 1000 existing clients using modern HTML, CSS, and JavaScript.",
                    "Implemented custom styling and interactive features for clients, leading to increased customer satisfaction.",
                    "Using modern CSS and a touch of VB.NET, created demo websites with unique, custom layouts for big potential clients, greatly increasing their interest in the company's product.",
                    "Developed a custom Selenium (browser automation) script in Python to automate the creation of page structures on the company's proprietary CMS software for new clients' websites, speeding up the initial setup stage by over 200%.",
                ],
                ["Sep 2021", "Nov 2022"],
                [Tech::Js, Tech::Html, Tech::Css, Tech::Py, Tech::Selenium],
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
