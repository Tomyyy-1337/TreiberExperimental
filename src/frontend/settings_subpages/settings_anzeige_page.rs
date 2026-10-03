use topcoat::{Result, context::Cx, cookie::{Cookies, cookies}, router::{Slot, page}, runtime::signal, view::{View, component, view}};

use crate::{frontend::{layouts::back_button_layout::back_button_layout}};

#[page("/settings/anzeige")]
pub async fn settings_anzeige() -> Result<impl View> {
    Ok(view! { back_button_layout(slot: Slot::new(view! {
        <h2> "Anzeige Einstellungen" </h2>

        theme_settings()
    }))})
}

#[component]
pub async fn theme_settings(cx: &Cx) -> Result<impl View> {   
    let cookies = cookies(cx);
    let current_theme_value = match cookies.get("theme") {
        Some(theme) => theme.value().to_string(),
        None => "dark".to_string(),
    };

    let current_theme = signal(cx, || current_theme_value);

    Ok(view! {
        <h3> "Theme" </h3>
        <p> "Current theme: " $(current_theme.get()) </p>

        <button @click=$(async |_event| { 
            let new_theme = if current_theme.get() == "dark" { "light".to_owned() } else { "dark".to_owned() };
            current_theme.set(new_theme);

            // Change the theme in the browser and set the cookie accordingly
            raw!("
                document.documentElement.setAttribute('data-theme', ${new_theme}.toString());
                document.cookie = 'theme=' + ${new_theme}.toString() + '; path=/; expires=' + new Date(Date.now() + 356 * 24 * 60 * 60 * 1000).toUTCString();                
            ");
        })>
            $(if current_theme.get() == "dark" { "Switch to Light Theme" } else { "Switch to Dark Theme" })
        </button>
    })
}