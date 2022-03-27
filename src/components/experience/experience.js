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
        '<a target="_blank" rel="noreferrer" href="http://bottletopmedia.com/portfolio/iclasscms/">Bottletop Media</a>',
        "September 2021 - Present",
        [
            "Work with creating and maintaining the frontend code for hundreds of schools' websites, both by myself and in cooperation with the Lead Developer.",
            "Help implement unique designs and features for schools/governing bodies who are looking for something a little more interesting or aesthetically pleasing.",
            "Create page structures on the company's CMS software for new clients' websites using a Selenium (browser automation) script I created.",
        ]
    ),
]
