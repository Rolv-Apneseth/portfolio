use leptos::*;
use leptos_darkmode::Darkmode;
use leptos_icons::Icon;

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let mut darkmode = expect_context::<Darkmode>();

    let get_icon = || {
        let darkmode = expect_context::<Darkmode>();
        if darkmode.is_dark() {
            icondata::BiMoonSolid
        } else {
            icondata::BiSunSolid
        }
    };

    let (icon, set_icon) = create_signal(icondata::BiMoonSolid);
    create_effect(move |_| set_icon(get_icon()));

    view! {
        <button
            title="Toggle theme"
            aria-label="Toggles between light and dark themes"
            class="absolute top-3 right-3 py-2 px-3 w-max font-semibold leading-5 text-gray-100 bg-violet-700 rounded-md duration-200 pointer-events-auto dark:bg-purple-600 hover:bg-violet-800 transition-bg-color dark:hover:bg-purple-500"
            on:click=move |_| darkmode.toggle()
        >
            <Icon class="text-2xl" icon=icon/>
        </button>
    }
}
