import { uniqueId } from "underscore"

class Project {
    constructor(name, imgName, description, codeURL, demoURL, pypiURL, tags) {
        this.name = name
        this.imgName = imgName
        this.description = description
        this.codeURL = codeURL
        this.demoURL = demoURL
        this.pypiURL = pypiURL
        this.tags = tags
        this.key = uniqueId("project")
    }
}

export const tags = {
    // Languages
    python: "Python",
    js: "JavaScript",
    scss: "SASS",
    // Libraries/frameworks
    sqlite: "SQLite",
    pyqt: "PyQt5",
    pygame: "Pygame",
    flask: "Flask",
    sqlalchemy: "SQLAlchemy",
    pyqtgraph: "PyQtGraph",
    nltk: "NLTK",
    curses: "Curses",
    pytest: "Pytest",
    jquery: "JQuery",
    bootstrap: "Bootstrap",
    // Concepts/skills
    automation: "Automation",
    algorithms: "Algorithms",
    unittest: "Unit Tests",
    webscraping: "Web Scraping",
}

export const projects = [
    new Project(
        "PS-Typer",
        "PS-Typer.png",
        [
            "A GUI application made using the PyQt5 library for Python. With it, a user can practice getting faster at typing with almost endless sources of text to type out, and even keep track of their progress. User w.p.m. scores are stored using an SQLite database accessed via the sqlite3 Python module.",
            "I use this program daily to improve my typing speed, and it has helped me progress with ease as I can keep typing as long as I want and still not repeat any text, which was the main goal of this application.",
        ],
        "https://github.com/Rolv-Apneseth/ps-typer",
        "",
        "https://pypi.org/project/ps-typer/",
        [tags.python, tags.pyqt, tags.pyqtgraph, tags.sqlite, tags.nltk]
    ),
    new Project(
        "daily_hn",
        "daily_hn.webp",
        [
            'A command line tool for displaying and opening links to the current best stories from <a target="_blank" rel="noreferrer" href="https://news.ycombinator.com">news.ycombinator.com</a> (Hacker News). It works by scraping the best stories page and displaying it for the user using a curses library UI.',
            "This handy tool is automatically tested and published to PyPI using Pytest and GitHub Actions",
        ],
        "https://github.com/Rolv-Apneseth/daily_hn",
        "",
        "https://pypi.org/project/daily-hn/",
        [tags.python, tags.webscraping, tags.curses, tags.pytest]
    ),
    new Project(
        "Just-A-Tracker",
        "just-a-tracker.webp",
        [
            "A web application which allows users to track issues for different projects, as well as add other users to workspaces and comment on specific issues for better communication with a team.",
            "Please note that the live demo is hosted on Heroku and may take a minute to start up on the first load.",
        ],
        "https://github.com/Rolv-Apneseth/just-a-tracker",
        "https://just-a-tracker.herokuapp.com/login",
        "",
        [
            tags.js,
            tags.python,
            tags.scss,
            tags.jquery,
            tags.bootstrap,
            tags.flask,
            tags.sqlalchemy,
        ]
    ),
    new Project(
        "Pathfind Visualiser",
        "pathfind-visualiser.webp",
        [
            "A pathfinding algorithm visualiser, written in Python and visualised with the help of the Pygame library.",
            "With this program you can see different pathfinding algorithms in action! It also has algorithms for generating different kinds of mazes so the user can see how the algorithms behave under different conditions.",
        ],
        "https://github.com/Rolv-Apneseth/pathfind-visualiser",
        "",
        "https://pypi.org/project/pathfind-visualiser/",
        [tags.python, tags.algorithms, tags.pygame]
    ),
    new Project(
        "Auto Folder Sorter",
        "auto-folder-sort.webp",
        [
            "A Python script which can sort (and keep sorted) any given number of folders, either by file type or date of modification.",
            "This script can be easily run in the background and keep, for example, your Downloads folder neatly sorted by file type so that whenever you download a new file, it is immediately sorted into the folder matching it's corresponding file type (same can be done with date).",
        ],
        "https://github.com/Rolv-Apneseth/auto-folder-sort",
        "",
        "",
        [tags.python, tags.automation, tags.unittest]
    ),
]
