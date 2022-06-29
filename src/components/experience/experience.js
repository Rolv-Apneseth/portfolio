import { uniqueId } from "underscore"

class Experience {
    constructor(position, name, timeRange, description) {
        this.position = position
        this.name = name
        this.timeRange = timeRange
        this.description = description
        this.key = uniqueId("workplace")
    }
}

export const experiences = [
    new Experience(
        "Junior Developer",
        '<a target="_blank" rel="noreferrer" href="https://iclass-cms.com/">Bottletop Media (iClassCMS)</a>',
        "September 2021 - Present",
        [
            "Maintain and make changes to websites for existing clients, using HTML, CSS, JavaScript and VB.Net",
            "Implement unique designs/features outside of the base templates using modern CSS and JavaScript functionality",
            "Create page structures on the company's custom CMS software for new clients' websites using a Selenium (browser automation) script I created using Python",
            "Maintain version control for the websites using Git",
        ]
    ),
]
