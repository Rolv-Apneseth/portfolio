import React from "react"
import ContactForm from "../components/ContactForm"
import Layout from "../components/Layout"
import ProjectsSection from "../components/projects/ProjectsSection"
import SocialLinks from "../components/SocialLinks"
import ExperienceSection from "../components/experience/ExperienceSection"

const Index = () => {
    const introAnimation = "slide-up"
    const sectionHeaderAnimation = "slide-up"

    return (
        <>
            <Layout>
                <section id="intro">
                    <p className={`${introAnimation} delay-1`}>Hey, I'm</p>
                    <h2 className={`${introAnimation} delay-2`}>
                        Rolv Apneseth
                    </h2>
                    <h1 className={`${introAnimation} delay-3`}>
                        Software Developer
                    </h1>
                    <div className={`${introAnimation} delay-4`}>
                        <a
                            className="button"
                            href="https://github.com/Rolv-Apneseth"
                            target="_blank"
                            rel="noreferrer"
                        >
                            Check out my GitHub!
                        </a>
                    </div>
                </section>

                {<ProjectsSection headerAnimation={sectionHeaderAnimation} />}

                {<ExperienceSection headerAnimation={sectionHeaderAnimation} />}

                <section id="contact">
                    <h3 className={sectionHeaderAnimation}>Contact</h3>
                    {<ContactForm />}
                </section>

                <SocialLinks />
            </Layout>
        </>
    )
}

export default Index
