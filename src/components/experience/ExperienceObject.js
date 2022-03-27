import React from "react"
import { uniqueId } from "underscore"

const ExperienceObject = props => {
    return (
        <li className="experience-object">
            <header className="slide-left">
                <h4
                    dangerouslySetInnerHTML={{
                        __html: `${props.experience.position} <span>@</span> <span>${props.experience.name}</span>`,
                    }}
                />
                <h5>{props.experience.timeRange}</h5>
            </header>
            <ul>
                {props.experience.description.map(item => (
                    <li
                        className="slide-left"
                        key={uniqueId("experienceObjectDescription")}
                    >
                        <p>{item}</p>
                    </li>
                ))}
            </ul>
        </li>
    )
}

export default ExperienceObject
