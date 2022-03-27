import React from "react"
import { Helmet } from "react-helmet"
import { throttle } from "underscore"

import Footer from "./Footer"
import Topbar from "./Topbar"

import enableScrollAnimations from "../utils/animations"
import "../styles/global.css"

const Layout = props => {
    const [topbarActive, setTopbarActive] = React.useState(false)
    const [scrollingUp, setScrollingUp] = React.useState(true)
    const [atStartingPosition, setAtStartingPosition] = React.useState(true)

    React.useEffect(() => {
        // Navbar appears as user scrolls up -------------------------
        let previousPosition = document.documentElement.scrollTop

        window.onscroll = throttle(() => {
            let position = document.documentElement.scrollTop

            setScrollingUp(position <= previousPosition)
            setAtStartingPosition(position === 0)

            previousPosition = position
        }, 50)

        // Initiate animations observer ------------------------------
        enableScrollAnimations()
        // -----------------------------------------------------------
    }, [])

    return (
        <React.Fragment>
            <Helmet>
                <title>Rolv Apneseth | Developer Portfolio</title>

                <meta
                    property="og:title"
                    content="Rolv Apneseth | Developer Portfolio"
                />
                <meta
                    name="description"
                    content="Rolv Apneseth Developer Portfolio"
                />
                <meta
                    property="og:description"
                    content="Rolv Apneseth Developer Portfolio"
                />

                <link rel="shortcut icon" href="./favicon.svg" />
            </Helmet>

            <Topbar
                active={topbarActive}
                setActive={setTopbarActive}
                scrollingUp={scrollingUp}
                atStartingPosition={atStartingPosition}
            />

            <main className={topbarActive ? "blur" : ""}>{props.children}</main>

            <Footer />
        </React.Fragment>
    )
}

export default Layout
