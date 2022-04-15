import React from "react"
import { throttle } from "underscore"

import Head from "./Head"
import Topbar from "./Topbar"
import Footer from "./Footer"

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
            <Head />

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
