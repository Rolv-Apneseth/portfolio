import React from "react"
import "../styles/components/socialLinks.css"

const SocialLinks = () => {
    return (
        <nav className="social-links slide-right delay-7">
            <ul>
                <li>
                    <a
                        href="https://github.com/Rolv-Apneseth"
                        target="_blank"
                        rel="noreferrer"
                    >
                        <img src="./images/github.svg" alt="GitHub" />
                    </a>
                </li>
                <li>
                    <a
                        href="https://github.com/Rolv-Apneseth"
                        target="_blank"
                        rel="noreferrer"
                    >
                        <img src="./images/email.svg" alt="Email" />
                    </a>
                </li>
                <li>
                    <a
                        href="https://github.com/Rolv-Apneseth"
                        target="_blank"
                        rel="noreferrer"
                    >
                        <img src="./images/linkedin.svg" alt="LinkedIn" />
                    </a>
                </li>
                <li>
                    <a
                        href="https://stackoverflow.com/users/14316282/rolv-apneseth"
                        target="_blank"
                        rel="noreferrer"
                    >
                        <img
                            src="./images/stackoverflow.svg"
                            alt="StackOverflow"
                        />
                    </a>
                </li>
            </ul>
        </nav>
    )
}

export default SocialLinks
