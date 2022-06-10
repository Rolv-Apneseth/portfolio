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
            "Create and maintain the front end code for school websites, and ensure sites are always backed up using git.",
            "Help implement unique designs and features for schools/governing bodies who are looking for something a little more interesting or aesthetically pleasing.",
            "Create page structures on the company's CMS software for new clients' websites using a Selenium (browser automation) script I created.",
        ]
    ),
]
