use leptos::{
    error::Error,
    prelude::*,
};
use leptos_icons::Icon;
use serde::{
    Deserialize,
    Serialize,
};

use crate::components::external_link::ExternalLink;

const PRS_URL: &str =
"https://api.github.com/search/issues?q=+author%3ARolv-Apneseth+type%3Apr+-user%3Arolv-apneseth&sort=created&order=desc&per_page=100";

const REPOS: [[&str; 2]; 8] = [
    ["sxyazi/yazi", "Yazi - A modern terminal file manager"],
    [
        "jeertmans/languagetool-rust",
        "LanguageTool-Rust - Rust bindings to connect with LanguageTool server API",
    ],
    [
        "avencera/rustywind",
        "rustywind - Formatting tool for sorting Tailwind classes",
    ],
    [
        "SabrinaJewson/rofi-mode.rs",
        "rofi-mode.rs - Rust library for creating Rofi plugins",
    ],
    [
        "Macchina-CLI/libmacchina",
        "libmacchina - Rust library for fetching system information",
    ],
    [
        "Macchina-CLI/macchina",
        "Macchina - CLI tool for displaying system information",
    ],
    [
        "danyspin97/wpaperd",
        "wpaperd - Minimal wallpaper daemon for Wayland",
    ],
    [
        "tldr-pages/tldr",
        "TLDR - A collection of help pages for command-line tools",
    ],
];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct GithubResponse {
    items: Vec<GithubPr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct GithubPr {
    state: String,
    html_url: String,
    repository_url: String,
    title: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContributionsData {
    url: String,
    title: String,
    prs: Vec<GithubPr>,
}

impl ContributionsData {
    fn new(base_prs: &[GithubPr], repo: &str, title: &str) -> Self {
        let url = format!("https://github.com/{repo}");
        let repo_api_url = format!("https://api.github.com/repos/{repo}");

        let prs = base_prs
            .iter()
            .filter(|pr| pr.repository_url == repo_api_url)
            .cloned()
            .collect::<Vec<GithubPr>>();

        Self {
            url,
            prs,
            title: title.to_string(),
        }
    }
}

/// Fetch pull requests from GitHub
async fn fetch_prs() -> Result<Vec<GithubPr>, Error> {
    let json = reqwasm::http::Request::get(PRS_URL)
        .send()
        .await?
        .json::<GithubResponse>()
        .await?;

    Ok(json.items)
}

#[component]
pub fn Contributions(#[prop()] data: ContributionsData) -> impl IntoView {
    let pr_views = data
        .prs
        .iter()
        .cloned()
        .map(|pr| {
            let fill = match pr.state.as_str() {
                "closed" => "fill-violet-700 dark:fill-purple-500",
                "open" => "fill-green-600 dark:fill-green-500",
                _ => "",
            };

            view! {
                <li>
                    <a
                        class="flex gap-2 items-center text-sm duration-200 transition-text-color text-slate-700 dark:text-slate-400 hover:text-slate-950 hover:dark:text-slate-200"
                        href=pr.html_url.to_owned()
                    >
                        <span>
                            <Icon
                                icon=icondata::IoGitPullRequest
                                attr:class=format!("h-6 w-6 {fill}")
                            />
                        </span>
                        <span class="flex-grow-0 flex-shrink-1">{pr.title.to_owned()}</span>
                    </a>
                </li>
            }
        })
        .collect_view();

    view! {
        <li class="py-4 rounded-md transition-all duration-300 ps-2 motion-reduce:transition-none lg:hover:!opacity-100 lg:group-hover/list:opacity-50 lg:hover:bg-slate-300/20 lg:dark:hover:bg-slate-800/50 lg:hover:shadow[inset_0_1px_0_0_rgba(148,163,184,0.1)] lg:hover:drop-shadow-lg">
            <a
                class="text-slate-700 dark:text-slate-200"
                href=data.url
                target="_blank"
                rel="noreferrer noopener"
            >
                <h4 class="flex gap-1 items-center mb-2 font-medium leading-snug text-slate-900 dark:text-slate-200">
                    <Icon icon=icondata::IoCaretForward />
                    {data.title}
                </h4>
            </a>
            <ul class="flex flex-col gap-2 ps-5">{pr_views}</ul>
        </li>
    }
}

#[component]
pub fn ContributionsSection() -> impl IntoView {
    let resource = LocalResource::new(fetch_prs);

    let fallback_loading = move || {
        view! { <div attr:aria-role="status">"Loading..."</div> }
    };

    let fallback_error = move |_errors| {
        view! {
            <section class="ps-5">
                <h6 class="text-red-300">Error - Failed to fetch data from the GitHub API</h6>
                <p class="text-xs">
                    Please wait before refreshing as this is likely due to a rate-limit
                </p>
            </section>
        }
    };

    let contribution_views = move || {
        resource.and_then(|prs| {
            REPOS
                .iter()
                .map(|[repo, title]| ContributionsData::new(prs, repo, title))
                .map(|d| view! { <Contributions data=d /> })
                .collect_view()
        })
    };

    view! {
        <Transition fallback=fallback_loading>
            <ErrorBoundary fallback=fallback_error>
                <aside class="mb-3 text-sm italic">
                    Open source contributions fetched using the
                    <ExternalLink
                        href="https://docs.github.com/en/rest?apiVersion=2022-11-28".to_string()
                        content="GitHub API".to_string()
                    />
                </aside>
                <ul class="flex flex-col gap-1 group/list">{contribution_views}</ul>
            </ErrorBoundary>
        </Transition>
    }
}
