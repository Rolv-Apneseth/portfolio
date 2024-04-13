use leptos::*;
use leptos_icons::Icon;

use crate::components::tag::{
    Tags,
    Tech,
};

type ProjectDataInput<'a> = (&'a str, &'a str, &'a [&'a str], &'a [&'a str], &'a [Tech]);
const PROJECTS: &[ProjectDataInput] = &[ 
    (
                "tfm.nvim · Terminal file manager integration for Neovim",
                "tfm-nvim.png",
                &["https://github.com/Rolv-Apneseth/tfm.nvim"],
                &[
                    "A plugin for Neovim which integrates several well-known terminal file managers directly into the editor. This provides the user with the ability to switch between these different file managers, and utilise them as alternatives to options such as file trees, or the builtin netrw, depending on their preferences.",
                    "This has become an essential part of my Neovim configuration, and pairs well with the Yazi terminal file manager to substantially improve my workflow.",
                ],
                &[Tech::Lua, Tech::Neovim],
),
    (

                "rofi-games · Launch games from different sources all from one place",
                "rofi-games.png",
                &[
                    "https://crates.io/crates/lib_game_detector",
                    "https://github.com/Rolv-Apneseth/rofi-games",
                ],
                &[
                    "A plugin for the Rofi application launcher on Linux, which adds a mode for
    launching games available on a user's system.",
                    "Games are parsed at runtime from various common sources such as Steam, Heroic Games, Lutris and Bottles. This allows a user to efficiently access their entire gaming collection on Linux by integrating all games into a single, light-weight launcher",
                    "The game parsing logic has also been extracted into a separate, standalone library, enabling other developers to leverage this functionality for their own purposes.",
                ],
                &[Tech::Rust, Tech::Criterion, Tech::Tracing, Tech::Nom],
),
    (

                "PS-Typer · Desktop application for practising typing",
                "PS-Typer.png",
                &[
                    "https://pypi.org/project/ps-typer/",
                    "https://github.com/Rolv-Apneseth/ps-typer",
                ],
                &[
                    "A GUI application made using the PyQt5 library for Python. With it, a user can practise getting their typing skills with almost endless sources of text to type out, and even keep track of their progress.",
                    "User W.P.M. scores are stored locally using an SQLite database accessed via the sqlite3 Python module, and a user's progress is visualised using PyQtGraph.",
                    "I personally use this program most days so that I can improve my typing speed
    and accuracy. It has helped me progress comfortably, as I can keep typing as long as I want and still not repeat any text, which was the main goal of this application.",
                ],
                &[Tech::Py, Tech::Qt, Tech::PyQtGraph, Tech::Sqlite, Tech::Nltk],
),
    (
                "pathfind-visualiser · Pathfinding algorithm visualiser",
                "pathfind-visualiser.webp",
                &[
                    "https://pypi.org/project/pathfind-visualiser/",
                    "https://github.com/Rolv-Apneseth/pathfind-visualiser",
                ],
                &[
                    "A pathfinding algorithm visualiser, written in Python and visualised with the help of the Pygame library.",
                    "With this program a user can see different pathfinding algorithms in action. It also has algorithms for generating different kinds of mazes, so the behaviour of these algorithms can also be observed under different conditions.",
                ],
                &[Tech::Py, Tech::PyGame],
)
];


const PROJECTS_ARCHIVED: &[ProjectDataInput] = &[
(
                "ua-explorer · Explore and compare cities based on their statistics",
                "ua_explorer.webp",
                &["https://github.com/Rolv-Apneseth/ua-explorer"],
                &[
                    "A website built to allow users to explore and compare statistics about different cities and urban areas from around the world.",
                    "Data about cities is fetched from the Teleport public API, and further links to that API and Wikepedia articles about each location is provided for further reading.",
                ],
                &[Tech::Ts, Tech::React, Tech::Scss, Tech::Gatsby, Tech::Netlify],
),

    (
                "Just-A-Tracker · Web application for bug/issue tracking",
                "just-a-tracker.webp",
                &["https://github.com/Rolv-Apneseth/just-a-tracker"],
                &[
                    "A web application which allows users to track issues for different projects, as well as add other users to workspaces and comment on specific issues for better communication with a team.",
                    "The backend is built using Python with Flask and SQLAlchemy, and the frontend uses Bootstrap as well as custom SCSS for styling.",
                ],
                &[
                    Tech::Js,
                    Tech::Jquery,
                    Tech::Scss,
                    Tech::Bs,
                    Tech::Py,
                    Tech::Flask,
                    Tech::SqlAlchemy,
                ],
),
    (

                "daily_hn · CLI for browsing the top stories on HN",
                "daily_hn.webp",
                &[
                    "https://pypi.org/project/daily-hn/",
                    "https://github.com/Rolv-Apneseth/daily_hn",
                ],
                &[
                    "A command line tool for displaying and opening links to the current best stories from news.ycombinator.com (Hacker News). It works by scraping the best stories page and displaying it for the user using a curses library UI.",
                    "Automatically tested and published to PyPI using Pytest and GitHub Actions",
                ],
                &[Tech::Py, Tech::Curses, Tech::PyTest],
)

];

pub struct ProjectData {
    title: String,
    path_image: String,
    urls: Vec<String>,
    description: Vec<String>,
    techs: Vec<Tech>,
}
impl ProjectData {
    pub fn new(input: ProjectDataInput) -> Self {
        Self {
            title: input.0.into(),
            path_image: format!("/public/project_demos/{}", input.1),
            urls: input.2.iter().map(|s| s.to_string()).collect(),
            description: input.3.iter().map(|s| s.to_string()).collect(),
            techs: input.4.iter().copied().collect(),
        }
    }
}

#[component]
pub fn Project(#[prop()] data: ProjectData) -> impl IntoView {
    let url_views = {
        data.urls.into_iter()
            .map(|u| match u.split('/').nth(2) {
                Some("github.com") => (u, icondata::BiGithub, "GitHub repository"),
                Some("pypi.org") => (u, icondata::BiPython, "pypi.org distribution"),
                Some("crates.io") => (u, icondata::LuBoxes, "crates.io distribution"),
                _ => (u, icondata::BiLinkRegular, "Live demo"),
            })
            .map(|(url, icon, desc)| {
                view! {
                    <li>
                        <a
                            class="duration-200 group motion-safe:transition-text-color dark:hover:text-slate-200 hover:text-slate-950"
                            href=url
                            title=desc
                            target="_blank"
                            rel="noreferrer noopener"
                        >
                            <Icon
                                class="w-6 h-6 duration-200 group-hover:-translate-y-1 motion-safe:transition-transform"
                                icon=icon
                            />
                        </a>
                    </li>
                }
            })
            .collect_view()
    };

    view! {
        <li class="flex flex-col gap-3 sm:grid sm:grid-cols-8 lg:flex lg:hover:!opacity-100
        lg:group-hover/list:opacity-50 rounded-md lg:hover:bg-slate-300/50
        lg:dark:hover:bg-slate-800/50 lg:hover:shadow[inset_0_1px_0_0_rgba(148,163,184,0.1)]
        lg:hover:drop-shadow-lg py-6 px-3 duration-300">
            <header class="flex gap-4 justify-between sm:col-span-8">
                <h4 class="text-sm font-medium leading-snug text-slate-800 dark:text-slate-200">
                    {data.title}
                </h4>

                <ul class="flex gap-2">{url_views}</ul>
            </header>

            <a
                class="sm:col-span-3 lg:px-2 h-max"
                href=&data.path_image
                title="Open image"
                target="_blank"
                rel="noreferrer noopener"
            >
                <img class="rounded-md shadow-lg" alt="Demo image" src=data.path_image/>
            </a>

            <div class="flex flex-col gap-3 sm:col-span-5">
                <ul class="flex flex-col gap-2 list-disc ps-4">
                    {data
                        .description
                        .iter()
                        .map(|d| {
                            view! { <li class="text-sm leading-normal">{d}</li> }
                        })
                        .collect::<Vec<_>>()}
                </ul>

                <Tags techs=data.techs/>
            </div>
        </li>
    }
}

#[component]
pub fn ProjectsSection() -> impl IntoView {
    let project_views = PROJECTS.iter().map(|input| {
        let data = ProjectData::new(*input);
        view! { <Project data=data/> }
    }).collect_view();

    view! { <ul class="group/list">{project_views}</ul> }
}

#[component]
pub fn ArchivedProjectsSection() -> impl IntoView {
    let archived_project_views = PROJECTS_ARCHIVED.iter().map(|input| {
        let data = ProjectData::new(*input);
        view! { <Project data=data/> }
    }).collect_view();
    view! {
        <aside class="mb-3 text-sm italic">
            These are past projects that have been broken (e.g. by an API being retired, as
            is the case for ua-explorer), rendering the project useless without a major rework
        </aside>
        {archived_project_views}
        <ul class="group/list"></ul>
    }
}
