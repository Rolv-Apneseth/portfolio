import React from "react"
import { Helmet } from "react-helmet"
import preview from "../../static/images/preview.png"
import favicon from "../../static/favicon.svg"

const Head = () => {
    return (
        <Helmet
            htmlAttributes={{
                lang: "en",
            }}
        >
            <meta
                name="viewport"
                content="width=device-width, initial-scale=1.0"
            />

            <title>Rolv Apneseth | Developer Portfolio</title>
            <meta
                property="og:title"
                content="Rolv Apneseth | Developer Portfolio"
            />
            <meta
                name="description"
                content="Rolv Apneseth Developer Portfolio, displaying work experience
                and previews of and links to all notable personal projects. Also linked
                are GitHub, StackOverflow and LinkedIn pages."
            />
            <meta
                property="og:description"
                content="Rolv Apneseth Developer Portfolio, displaying work experience
                and previews of and links to all notable personal projects. Also linked
                are GitHub, StackOverflow and LinkedIn pages."
            />
            <link rel="canonical" href="https://rolvapneseth.com/" />
            <meta property="og:url" content="https://rolvapneseth.com/" />
            <meta property="og:image" content={preview} />
            <meta property="og:locale" content="en_GB" />
            <meta property="og:type" content="website" />
            <meta name="twitter:card" content="summary_large_image" />
            <meta name="theme-color" content="#0a182e" />

            <meta
                name="keywords"
                content="HTML, CSS, JavaScript, Python, React, Gatsby, Developer,
                Portfolio, Rolv, Apneseth, Software, Frontend, Backend, Code, Programmer"
            />
            <meta name="author" content="Rolv Apneseth" />

            <link rel="shortcut icon" href={favicon} type="image/svg+xml" />

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
