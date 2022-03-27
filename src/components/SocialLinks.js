import React from "react"
import { uniqueId } from "underscore"
import "../styles/components/socialLinks.css"

class Link {
    constructor(title, url) {
        this.title = title
        this.url = url
        this.imgUrl = title.toLowerCase()
        this.key = uniqueId("socialLink")
    }
}

const urls = [
    new Link("Github", "https://github.com/Rolv-Apneseth"),
    new Link("Email", "mailto:rolv.apneseth@gmail.com"),
    new Link("LinkedIn", "https://www.linkedin.com/in/rolv-apneseth-6b47401b5"),
    new Link(
        "StackOverflow",
        "https://stackoverflow.com/users/14316282/rolv-apneseth"
    ),
]

const SocialLinks = () => {
    return (
        <nav className="social-links slide-right delay-7">
            <ul>
                {urls.map(item => (
                    <li key={item.key}>
                        <a href={item.url} target="_blank" rel="noreferrer">
                            <img
                                src={`./images/${item.imgUrl}.svg`}
                                alt={item.title}
                            />
                        </a>
                    </li>
                ))}
            </ul>
        </nav>
    )
}

export default SocialLinks
