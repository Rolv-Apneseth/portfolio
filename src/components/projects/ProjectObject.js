import React from "react"
import { uniqueId } from "underscore"

const ProjectObject = props => {
    const img_url = `/images/project_demos/${props.project.imgName}`

    return (
        <li className={`project ${props.animationClass}`}>
            <img alt={`${props.project.name} Demo`} src={img_url} />

            <section>
                <h4>{props.project.name}</h4>

                <ul>
                    {props.project.tags.map(tag => (
                        <li key={uniqueId("projectObjectTag")}>{tag}</li>
                    ))}
                </ul>

                {props.project.description.map(paragraph => (
                    <p
                        key={uniqueId("projectObjectDescription")}
                        dangerouslySetInnerHTML={{ __html: paragraph }}
                    />
                ))}

                <div>
                    <a
                        className="button"
                        type="button"
                        href={props.project.codeURL}
                        target="_blank"
                        rel="noreferrer"
                    >
                        Code
                    </a>

                    {props.project.demoURL && (
                        <a
                            className="button"
                            type="button"
                            href={props.project.demoURL}
                            target="_blank"
                            rel="noreferrer"
                        >
                            Demo
                        </a>
                    )}
                    {props.project.pypiURL && (
                        <a
                            className="button"
                            type="button"
                            href={props.project.pypiURL}
                            target="_blank"
                            rel="noreferrer"
                        >
                            PyPI
                        </a>
                    )}
                </div>
            </section>
        </li>
    )
}

export default ProjectObject
