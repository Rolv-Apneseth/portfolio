import { uniqueId } from "underscore"

class Project {
    constructor(name, imgName, description, codeURL, demoURL, tags) {
        this.name = name
        this.imgName = imgName
        this.description = description
        this.codeURL = codeURL
        this.demoURL = demoURL
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
    webscraping: "Webscraping",
}

export const projects = [
    new Project(
        "Just-A-Tracker",
        "just-a-tracker.png",
        [
            "A web application which allows users to track issues for different projects, as well as add other users to workspaces and comment on specific issues for better communication with a team.",
            "Please note that the live demo is hosted on Heroku and may take a minute to start up on the first load.",
        ],
        "https://github.com/Rolv-Apneseth/just-a-tracker",
        "https://just-a-tracker.herokuapp.com/login",
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
        "PS-Typer",
        "PS-Typer.png",
        [
            "A GUI application made using the PyQt5 library for Python. With it, a user can practice getting faster at typing with almost endless sources of text to type out, and even keep track of their progress.",
            "I use this program daily to improve my typing speed, and it has helped me go from 40 wpm to around 70 wpm.",
        ],
        "https://github.com/Rolv-Apneseth/speed-typer",
        "",
        [tags.python, tags.pyqt, tags.pyqtgraph, tags.nltk]
    ),
    new Project(
        "daily_hn",
        "daily_hn.png",
        [
            'A command line tool for displaying and opening links to the current best stories from <a target="_blank" rel="noreferrer" href="news.ycombinator.com">news.ycombinator.com</a> (Hacker News). It works by scraping the best stories page and displaying it for the user using a curses library UI.',
            'This handy tool is automatically tested using Pytest and GitHub Actions, and is available as a PyPi package <a target="_blank" rel="noreferrer" href="https://pypi.org/project/daily-hn/">here!</a>',
        ],
        "https://github.com/Rolv-Apneseth/daily_hn",
        "",
        [tags.python, tags.webscraping, tags.curses, tags.pytest]
    ),
    new Project(
        "Pathfind Visualiser",
        "pathfind-visualiser.png",
        [
            "A pathfinding algorithm visualiser, written in Python and visualised with the help of the Pygame library.",
            "With this program you can see different pathfinding algorithms in action! It also has algorithms for generating different kinds of mazes so the user can see how the algorithms behave under different conditions.",
        ],
        "https://github.com/Rolv-Apneseth/pathfind-visualiser",
        "",
        [tags.python, tags.algorithms, tags.pygame]
    ),
    new Project(
        "Auto Folder Sorter",
        "auto-folder-sort.png",
        [
            "A Python script which can sort (and keep sorted) any given number of folders, either by file type or date of modification.",
            "This script can be easily run in the background and keep, for example, your Downloads folder neatly sorted by file type so that whenever you download a new file, it is immediately sorted into the folder matching it's corresponding file type (same can be done with date).",
        ],
        "https://github.com/Rolv-Apneseth/auto-folder-sort",
        "",
        [tags.python, tags.automation, tags.unittest]
    ),
]
