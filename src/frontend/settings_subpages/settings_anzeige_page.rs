use topcoat::{Result, context::Cx, cookie::{Cookies, cookies}, router::{Slot, page}, runtime::Event, view::{Attributes, Child, View, attributes, component, view}};

use crate::frontend::layouts::{back_button_layout::back_button_layout};

#[page("/settings/anzeige")]
pub async fn settings_anzeige_page() -> Result<impl View> {
    Ok(view! { back_button_layout(slot: Slot::new(
        view! {
            <h2> "Anzeige Einstellungen" </h2>
            <div class="grid gap-4">
                theme_settings()
                overlay_settings()
            </div>
        }
    ))})
}

#[component]
async fn settings_container(
    title: &str,
    #[default] description: &str,
    child: Child<'_>
) -> Result<impl View> {
    Ok(view! {
        <article class="grid gap-3 rounded-xl border border-border bg-card p-4 text-card-foreground shadow-sm">
            <div class="grid gap-1">
                <h3 class="text-base font-semibold"> (title) </h3>
                if description != "" {
                    <p class="text-sm text-muted-foreground"> (description) </p>
                }
            </div>
            <div class="border-t border-border" aria-hidden="true"></div>
            <div class="grid gap-1.5">
                (child)
            </div>
        </article>
    })
}

#[component]
async fn select_widget(
    title: &str,
    possible_values: &[(&str, &str)],
    selected: String,
    on_select: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <label class="text-xs font-semibold uppercase tracking-[0.08em] text-muted-foreground"> (title) 
            <select 
                (on_select) 
                name=(format!("{}_select", title.split_whitespace().map(|s| s.to_lowercase()).collect::<Vec<_>>().join("_"))) 
                class="select-arrow h-10 w-full cursor-pointer rounded-md border border-border bg-background px-3 pr-8 text-sm text-foreground outline-none"
            >
                for (value, label) in possible_values {
                    <option value=(value) selected=(selected == *value)> (label) </option>
                }
            </select>
        </label>
    })
}

#[component]
async fn theme_settings(cx: &Cx) -> Result<impl View> {  
    let cookies = cookies(cx);
    let current_theme_value = match cookies.get("theme") {
        Some(theme) => theme.value().to_string(),
        None => "dark".to_string(),
    };

    let on_select = attributes! {
        @change=$(async |event: Event| {
            let _new_theme = event.target.value;
            raw!(
               "document.documentElement.className = ${_new_theme};
                document.cookie = 'theme=' + ${_new_theme} + '; path=/; expires=' + new Date(Date.now() + 356 * 24 * 60 * 60 * 1000).toUTCString();"
            );
        })
    };

    let options = &[("light", "Light"), ("dark", "Dark")];

    Ok(view! {
        settings_container(
            title: "Theme Einstellungen",

            select_widget(
                title: "Theme Toggle",
                possible_values: options,
                selected: current_theme_value,
                on_select: on_select,
            )
        )
    })
}

#[component]
async fn overlay_settings(cx: &Cx) -> Result<impl View> {
    let cookies = cookies(cx);

    let overlay_enabled = match cookies.get("overlay_enabled") {
        Some(value) => value.value().to_string() == "true",
        None => false,
    }; 

    let overlay_position = match cookies.get("overlay_position") {
        Some(value) => value.value().to_string(),
        None => "top".to_string(),
    };

    Ok(view! {
        settings_container(
            title: "Overlay Einstellungen",

            select_widget(
                title: "Overlay Aktivieren/Deaktivieren",
                possible_values: &[("true", "Aktiviert"), ("false", "Deaktiviert")],
                selected: overlay_enabled.to_string(),
                on_select: attributes! {
                    @change=$(async |event: Event| {
                        let _new_value = event.target.value;
                        raw!(
                            "document.cookie = 'overlay_enabled=' + ${_new_value} + '; path=/; expires=' + new Date(Date.now() + 356 * 24 * 60 * 60 * 1000).toUTCString();"
                        );
                    })
                },
            )
            
            select_widget(
                title: "Overlay Position",
                possible_values: &[("top", "Oben"), ("bottom", "Unten")],
                selected: overlay_position,
                on_select: attributes! {
                    @change=$(async |event: Event| {
                        let _new_value = event.target.value;
                        raw!(
                            "document.cookie = 'overlay_position=' + ${_new_value} + '; path=/; expires=' + new Date(Date.now() + 356 * 24 * 60 * 60 * 1000).toUTCString();"
                        );
                    })
                },
            )

        )
    })
}