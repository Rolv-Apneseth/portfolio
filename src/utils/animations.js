const ANIMATIONS = [
    "fade-in",
    "slide-left",
    "slide-right",
    "slide-up",
    "slide-down",
    "grow",
]

const enableScrollAnimations = () => {
    // Intersection observer
    function getIntersectionObserver(options) {
        return new IntersectionObserver((entries, observer) => {
            entries.forEach(entry => {
                if (!entry.isIntersecting) {
                    return
                }

                entry.target.classList.add("animate")
                observer.unobserve(entry.target)
            })
        }, options)
    }

    // Generate observers
    const baseObserver = getIntersectionObserver({
        threshold: 0.7,
        rootMargin: "-5% 50%",
    })
    const projectsObserver = getIntersectionObserver({
        threshold: 0.5,
        rootMargin: "-2% 50%",
    })

    // Apply observers to elements
    ANIMATIONS.forEach(className => {
        document.querySelectorAll(`.${className}`).forEach(element => {
            if (element.classList.contains("project")) {
                projectsObserver.observe(element)
            } else {
                baseObserver.observe(element)
            }
        })
    })
}

export default enableScrollAnimations
