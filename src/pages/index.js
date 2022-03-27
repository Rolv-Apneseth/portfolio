import React from "react"
import ContactForm from "../components/ContactForm"
import Layout from "../components/Layout"
import ProjectsSection from "../components/projects/ProjectsSection"
import SocialLinks from "../components/SocialLinks"
import ExperienceSection from "../components/experience/ExperienceSection"

const Index = () => {
    return (
        <>
            <Layout>
                <section id="intro">
                    <p className="slide-left delay-1">Hey, I'm</p>
                    <h2 className="slide-left delay-2">Rolv Apneseth</h2>
                    <h1 className="slide-left delay-3">Software Developer</h1>
                    <div className="slide-left delay-4">
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

                {/* <section id="about">
          <h3 className="slide-left">About</h3>
        </section> */}

                {<ProjectsSection />}

                {<ExperienceSection />}

                <section id="contact">
                    <h3 className="slide-left">Contact</h3>
                    {<ContactForm />}
                </section>

                <SocialLinks />
            </Layout>
        </>
    )
}

export default Index
