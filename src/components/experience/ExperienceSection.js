import React from "react"
import { experiences } from "./experience"
import ExperienceObject from "./ExperienceObject"

const ExperienceSection = props => {
    return (
        <section id="experience">
            <h3 className={props.headerAnimation}>Experience</h3>
            <ul>
                {experiences.map((experience, index) => (
                    <ExperienceObject
                        experience={experience}
                        key={experience.key}
                    />
                ))}
            </ul>
        </section>
    )
}

export default ExperienceSection
