use leptos::*;
use std::fmt;

#[derive(Clone, Copy)]
pub enum Tech {
    Html,
    Css,
    Scss,
    Js,
    Ts,
    React,
    Redux,
    Bs,
    Py,
    PyGame,
    PyTest,
    PyQtGraph,
    Qt,
    Sqlite,
    Nltk,
    Az,
    Jest,
    Gatsby,
    Netlify,
    Curses,
    Jquery,
    Flask,
    SqlAlchemy,
    Rust,
    Axum,
    Serde,
    Tracing,
    Nom,
    Criterion,
    Lua,
    Neovim,
    Docker,
}

impl fmt::Display for Tech {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Tech::Rust => "Rust",
                Tech::Axum => "Axum",
                Tech::Serde => "Serde",
                Tech::Criterion => "Criterion",
                Tech::Tracing => "Tracing",
                Tech::Nom => "Nom",
                Tech::Lua => "Lua",
                Tech::Neovim => "Neovim",
                Tech::Html => "HTML",
                Tech::Css => "CSS",
                Tech::Scss => "SCSS",
                Tech::Js => "JavaScript",
                Tech::Ts => "Typescript",
                Tech::React => "React",
                Tech::Redux => "Redux",
                Tech::Bs => "Bootstrap",
                Tech::Py => "Python",
                Tech::PyGame => "PyGame",
                Tech::PyTest => "PyTest",
                Tech::PyQtGraph => "PyQtGraph",
                Tech::Qt => "Qt5",
                Tech::Az => "Azure DevOps",
                Tech::Jest => "Jest",
                Tech::Gatsby => "Gatsby",
                Tech::Netlify => "Netlify",
                Tech::Sqlite => "SQLite",
                Tech::Nltk => "NLTK",
                Tech::Curses => "Curses",
                Tech::Jquery => "JQuery",
                Tech::Flask => "Flask",
                Tech::SqlAlchemy => "SQLAlchemy",
                Tech::Docker => "Docker",
            }
        )
    }
}

#[component]
pub fn Tag(#[prop()] tech: Tech) -> impl IntoView {
    view! {
        <li class="flex items-center py-1 px-3 text-xs font-medium leading-5 text-violet-900 rounded-full dark:text-purple-300 bg-violet-400/20 dark:bg-purple-200/10">
            {tech.to_string()}
        </li>
    }
}

#[component]
pub fn Tags(#[prop()] techs: Vec<Tech>) -> impl IntoView {
    view! {
        <ul class="flex flex-wrap gap-1 w-100">
            {techs.into_iter().map(|t| view! { <Tag tech=t/> }).collect_view()}
        </ul>
    }
}
