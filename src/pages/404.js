import React from "react"
import Layout from "../components/Layout"

const PageNotFound = () => {
    return (
        <>
            <Layout>
                <section style={{ textAlign: "center" }} id="intro">
                    <h2>Uh oh!</h2>
                    <h1>The page you are looking for could not be found.</h1>
                    <p>
                        If you believe this is an error please open an
                        issue&nbsp;
                        <a
                            href="https://github.com/Rolv-Apneseth/portfolio/issues"
                            target="_blank"
                            rel="noreferrer"
                        >
                            here
                        </a>
                    </p>
                </section>
            </Layout>
        </>
    )
}

export default PageNotFound
