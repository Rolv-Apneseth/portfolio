import React from "react"
import { Helmet } from "react-helmet"

const Head = () => {
    return (
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

            {/* Preload fonts */}
            <link
                rel="preload"
                as="font"
                href="/fonts/Roboto-Regular-webfont.woff"
                type="font/woff2"
                crossorigin="crossorigin"
            />
            <link
                rel="preload"
                as="font"
                href="/fonts/Roboto-Italic-webfont.woff"
                type="font/woff2"
                crossorigin="crossorigin"
            />
            <link
                rel="preload"
                as="font"
                href="/fonts/Roboto-Bold-webfont.woff"
                type="font/woff2"
                crossorigin="crossorigin"
            />
            <link
                rel="preload"
                as="font"
                href="/fonts/Roboto-Black-webfont.woff"
                type="font/woff2"
                crossorigin="crossorigin"
            />
        </Helmet>
    )
}

export default Head
