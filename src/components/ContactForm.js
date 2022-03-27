import React from "react"

// Auto expand text area
function autoExpand(event) {
    // Minimum height set to the same as in _contact.scss
    let minHeight = "calc(var(--space-m) + 0.5rem)"

    event.target.style.height = minHeight // Reset so textarea also shrinks
    event.target.style.height = event.target.scrollHeight + "px"
}

const ContactForm = () => {
    React.useState()
    return (
        <form action="https://formspree.io/f/xvodqzpa" method="POST">
            <div className="form-group slide-left">
                <label htmlFor="name">Name</label>
                <input id="name" type="text" name="name" required />
            </div>

            <div className="form-group slide-left">
                <label htmlFor="email">Email</label>
                <input id="email" type="email" name="email" required />
            </div>

            <div className="form-group slide-left">
                <label htmlFor="subject">Subject</label>
                <input id="subject" type="text" name="subject" required />
            </div>

            <div className="form-group slide-left">
                <label htmlFor="message">Message</label>
                <textarea
                    id="message"
                    name="message"
                    onChange={autoExpand}
                    required
                ></textarea>
            </div>

            <button className="button slide-left" type="submit">
                Submit
            </button>
        </form>
    )
}

export default ContactForm
