import React from "react"
import ProjectObject from "./ProjectObject"
import { projects } from "./projects"

const ProjectsSection = props => {
    return (
        <section id="projects">
            <h3 className={props.headerAnimation}>Projects</h3>
            <ul>
                {projects.map((project, index) => (
                    <ProjectObject
                        project={project}
                        key={project.key}
                        // Alternate animation
                        animationClass={`slide-${index % 2 ? "right" : "left"}`}
                    />
                ))}
            </ul>
        </section>
    )
}

export default ProjectsSection
