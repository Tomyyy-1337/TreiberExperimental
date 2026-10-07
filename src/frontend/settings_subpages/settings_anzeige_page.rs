use topcoat::{Result, context::Cx, cookie::{Cookies, cookies}, router::page, runtime::{Event, signal}, view::{Attributes, View, attributes, component, view}};

use crate::frontend::settings_subpages::settings_wrapper::{settings_container, settings_wrapper};

#[page("/settings/anzeige")]
pub async fn settings_anzeige_page() -> Result<impl View> {
    Ok(view! {
        settings_wrapper(
            title: "Anzeige Einstellungen",
            description: "Passe die Anzeigeeinstellungen an",

            theme_settings()
            overlay_settings()
        )
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
        <label class="flex flex-col gap-2 text-sm font-semibold tracking-wide text-foreground"> (title) 
            <select 
                (on_select) 
                name=(format!("{}_select", title.split_whitespace().map(|s| s.to_lowercase()).collect::<Vec<_>>().join("_"))) 
                class="select-arrow h-10 w-full cursor-pointer rounded-md border border-border bg-card px-3 pr-8 text-sm text-foreground outline-none"
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
    let theme_cookies = cookies(cx);
    let current_theme = theme_cookies.get("theme").map(|c| c.value().to_string()).unwrap_or_else(|| "dark".to_string());

    const THEME_OPTIONS: [(&str, &str); 2] = [
        ("light", "Light"),
        ("dark", "Dark"),
    ];

    Ok(view! {
        settings_container(
            title: "Theme Einstellungen",

            select_widget(
                title: "Theme Toggle",
                possible_values: &THEME_OPTIONS,
                selected: current_theme,
                on_select: attributes! {
                    @change=$(async |event: Event| {
                        let _new_theme = event.target.value;
                        raw!(
                        "document.documentElement.className = ${_new_theme};
                            document.cookie = 'theme=' + ${_new_theme} + '; path=/; expires=' + new Date(Date.now() + 356 * 24 * 60 * 60 * 1000).toUTCString();"
                        );
                    })
                }
            )
        )
    })
}

struct OverlayElement {
    label: &'static str,
    cookie_name: &'static str,
}

const OVERLAY_ELEMENTS: [OverlayElement; 7] = [
    OverlayElement { label: "Geschwindigkeit (km/h)", cookie_name: "show_speed" },
    OverlayElement { label: "500m Split", cookie_name: "show_500m_split" },
    OverlayElement { label: "Schlagzahl", cookie_name: "show_stroke_rate" },
    OverlayElement { label: "Distanz pro Schlag", cookie_name: "show_distance_per_stroke" },
    OverlayElement { label: "Distanz", cookie_name: "show_distance" },
    OverlayElement { label: "Fahrtzeit", cookie_name: "show_time" },
    OverlayElement { label: "Schläge", cookie_name: "show_strokes" },
];

#[component]
async fn overlay_settings(cx: &Cx) -> Result<impl View> {
    let cookies = cookies(cx);

    let overlay_enabled_cookie = cookies.get("overlay_enabled");
    let overlay_enabled = overlay_enabled_cookie.map(|c| c.value().to_string()).unwrap_or_else(|| true.to_string());
    let overlay_enabled_signal = signal(cx, || if overlay_enabled == "true" { true } else { false });

    let overlay_position_cookie = cookies.get("overlay_position");
    let overlay_position = overlay_position_cookie.map(|c| c.value().to_string()).unwrap_or_else(|| "top".to_string());

    Ok(view! {
        settings_container(
            title: "Overlay Einstellungen", 

            select_widget(
                title: "Overlay Aktivieren/Deaktivieren",
                possible_values: &[("true", "Aktiviert"), ("false", "Deaktiviert")],
                selected: overlay_enabled,
                on_select: attributes! {
                    @change=$(async |event: Event| {
                        let new_value = event.target.value;
                        overlay_enabled_signal.set(new_value == "true");
                        raw!(
                            "document.cookie = 'overlay_enabled=' + ${new_value} + '; path=/; expires=' + new Date(Date.now() + 356 * 24 * 60 * 60 * 1000).toUTCString();"
                        );
                    })
                },
            )

            <div :hidden=$(!overlay_enabled_signal.get())>
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

                <div class="grid gap-2">
                    <h4 class="pt-2 pb-1 text-sm font-semibold tracking-wide text-foreground">
                        "Eingeblendete Daten im Overlay"
                    </h4>
                    <div class="flex flex-col gap-1">
                    for element in OVERLAY_ELEMENTS.iter() {
                        overlay_element_toggle(
                            label: element.label,
                            cookie_name: element.cookie_name
                        )
                    }
                    </div>
                </div>
            </div>

            <div :hidden=$(overlay_enabled_signal.get())>
                overlay_disabled_hint()
            </div>
        )
    })
}

#[component]
async fn overlay_element_toggle(
    cx: &Cx,
    label: &str,
    cookie_name: &str
) -> Result<impl View> {
    let cookies = cookies(cx);
    
    let cookie = cookies.get(cookie_name);
    let cookie_value = cookie.map(|c| c.value() == "true").unwrap_or(true);

    Ok(view! {
        <label class="flex min-h-12 cursor-pointer items-center justify-between gap-2 rounded-lg border-2 border-border bg-card px-2 py-2 text-foreground has-[:checked]:border-primary">
            <span class="text-sm font-medium">(label)</span>
            <input
                class="peer sr-only"
                name="overlay-show-speed"
                type="checkbox"
                checked=(cookie_value)
                @change=$(async |_event: Event| {
                    let _new_value = _event.target.checked;
                    raw!(
                        "document.cookie = ${cookie_name} + '=' + ${_new_value} + '; path=/; expires=' + new Date(Date.now() + 356 * 24 * 60 * 60 * 1000).toUTCString();"
                    );
                })
            />
            <span class="relative h-5 w-9 shrink-0 rounded-full bg-muted-foreground after:absolute after:left-1 after:top-1 after:h-3 after:w-3 after:rounded-full after:bg-background peer-checked:bg-primary peer-checked:after:translate-x-4 peer-focus-visible:outline peer-focus-visible:outline-2 peer-focus-visible:outline-ring" aria-hidden="true"></span>
        </label>
    })
}

#[component]
async fn overlay_disabled_hint() -> Result<impl View> {
    Ok(view! {
        <div class="flex items-start gap-3 rounded-lg border border-border bg-muted/40 px-3 py-3 text-muted-foreground">
            <span class="mt-0.5 flex h-5 w-5 shrink-0 items-center justify-center rounded-full border border-current text-xs font-semibold" aria-hidden="true">"i"</span>
            <div class="grid gap-1">
                <h4 class="text-sm font-semibold leading-5 text-foreground">
                    "Overlay ist deaktiviert."
                </h4>
                <p class="text-sm leading-5 text-muted-foreground">
                    "Aktiviere das Overlay, um die Einstellungen anzupassen."
                </p>
            </div>
        </div>
    })
}