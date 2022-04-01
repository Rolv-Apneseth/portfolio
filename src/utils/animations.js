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

    const observer = getIntersectionObserver({
        threshold: 0.7,
        rootMargin: "-2% 50%",
    })

    // Apply observer to elements
    ANIMATIONS.forEach(className => {
        document.querySelectorAll(`.${className}`).forEach(element => {
            observer.observe(element)
        })
    })
}

export default enableScrollAnimations
