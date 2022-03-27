import { Link } from "gatsby"
import React from "react"
import "../styles/components/topbar.css"

const Topbar = props => {
    const deactivate = () => {
        props.setActive(false)
    }

    return (
        <header
            className={`topbar${props.active ? " active" : ""}${
                props.scrollingUp ? " appear" : ""
            }${props.atStartingPosition ? " initial-state" : ""}`}
        >
            <nav>
                <ul>
                    <li>
                        <Link onClick={deactivate} to="/">
                            Home
                        </Link>
                    </li>
                    <li>
                        <Link onClick={deactivate} to="/#projects">
                            Projects
                        </Link>
                    </li>
                    <li>
                        <Link onClick={deactivate} to="/#experience">
                            Experience
                        </Link>
                    </li>
                    <li>
                        <Link onClick={deactivate} to="/#contact">
                            Contact
                        </Link>
                    </li>
                    <li>
                        <Link
                            onClick={deactivate}
                            to="/rolvApnesethCV.pdf"
                            target="_blank"
                            rel="noopener"
                        >
                            CV
                        </Link>
                    </li>
                </ul>
            </nav>

            <button
                className="topbar-toggle"
                aria-label="Navbar visibility toggle (hamburger menu)"
                onClick={() => props.setActive(!props.active)}
            >
                <span className="hamburger"></span>
                <span className="hamburger"></span>
                <span className="hamburger"></span>
            </button>
        </header>
    )
}

export default Topbar
