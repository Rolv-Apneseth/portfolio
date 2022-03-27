const ANIMATIONS = ["fade-in", "slide-left", "slide-right", "grow"]

const enableScrollAnimations = () => {
    // Intersection observer
    function getIntersectionObserver(options) {
        return new IntersectionObserver((entries, observer) => {
            entries.forEach(entry => {
                if (!entry.isIntersecting) {
                    entry.target.classList.remove("animate")
                } else {
                    entry.target.classList.add("animate")
                }
            })
        }, options)
    }

    const observer = getIntersectionObserver({
        threshold: 0.7,
        rootMargin: "-5% 50%",
    })

    // Apply observer to elements
    ANIMATIONS.forEach(className => {
        document.querySelectorAll(`.${className}`).forEach(element => {
            observer.observe(element)
        })
    })
}

export default enableScrollAnimations
