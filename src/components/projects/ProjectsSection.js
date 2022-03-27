import React from "react"
import ProjectObject from "./ProjectObject"
import { projects } from "./projects"

const ProjectsSection = () => {
    return (
        <section id="projects">
            <h3 className="slide-left">Projects</h3>
            <ul>
                {projects.map(project => (
                    <ProjectObject project={project} key={project.key} />
                ))}
            </ul>
        </section>
    )
}

export default ProjectsSection
