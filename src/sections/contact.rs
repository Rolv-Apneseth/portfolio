use icondata::Icon as IconData;
use leptos::*;
use leptos_icons::Icon;

pub struct ContactLinkData {
    label: String,
    href: String,
    icon: IconData,
}
impl ContactLinkData {
    fn new(label: &str, href: &str, icon: IconData) -> Self {
        Self {
            label: label.to_string(),
            href: href.to_string(),
            icon,
        }
    }
}

#[component]
pub fn ContactLink(#[prop()] data: ContactLinkData) -> impl IntoView {
    view! {
        <li>
            <a
                class="duration-200 group transition-text-color motion-reduce:transition-none dark:hover:text-slate-200 hover:text-slate-950"
                aria-label=format!("Link to {} - opens in a new tab", data.label)
                title=data.label
                target="_blank"
                rel="noreferrer noopener"
                href=data.href
            >
                <Icon
                    class="w-7 h-7 transition-transform duration-200 group-hover:-translate-y-1 motion-reduce:transition-none"
                    icon=data.icon
                />
            </a>
        </li>
    }
}
#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <nav class="mt-6 lg:block" aria-label="Contact and social media links">
            <ul class="flex gap-4 text-xs">
                <ContactLink data=ContactLinkData::new(
                    "LinkedIn",
                    "https://www.linkedin.com/in/rolv-apneseth-6b47401b5",
                    icondata::BiLinkedin,
                )/>
                <ContactLink data=ContactLinkData::new(
                    "Email",
                    "mailto:rolv.apneseth@gmail.com",
                    icondata::IoMail,
                )/>
                <ContactLink data=ContactLinkData::new(
                    "GitHub",
                    "https://github.com/Rolv-Apneseth",
                    icondata::BiGithub,
                )/>
                <ContactLink data=ContactLinkData::new(
                    "StackOverflow",
                    "https://stackoverflow.com/users/14316282/rolv-apneseth",
                    icondata::BiStackOverflow,
                )/>
            </ul>
        </nav>
    }
}
