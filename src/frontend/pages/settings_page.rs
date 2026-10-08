use topcoat::{Result, router::{Href, HrefTarget, href, page}, runtime::link, view::{View, attributes, class, component, view}};

use crate::frontend::{layouts::nav_layout::nav_layout, settings_subpages::{settings_anzeige_page::settings_anzeige_page, settings_camera_page::settings_camera_page}};

#[page("/settings")]
pub async fn settings_page() -> Result<impl View> {
    Ok(view! { 
        nav_layout(
            <div class="grid gap-3">
                settings_link(
                    title: "Kamera Einstellungen",
                    description: "Eintellungen für Belichtung, Fokus und andere Kameraoptionen",
                    href: href!(settings_camera_page)
                )
                settings_link(
                    title: "Anzeige Einstellungen",
                    description: "Einstellungen für Theme, Overlay und andere Anzeigeoptionen",
                    href: href!(settings_anzeige_page)
                )
            </div>
        )
    })
}

#[component]
pub async fn settings_link<T: HrefTarget + Send>(title: &str, description: &str, href: Href<T, (), (), &str>) -> Result<impl View> {
    Ok(view! {
        link(
            href: href,
            attrs: attributes! {
                class=(class!("grid gap-1 rounded-2xl border border-border bg-card px-4 py-3 text-base text-foreground no-underline"))
            },

            <h3 class="text-base font-semibold leading-6 text-foreground">(title)</h3>
            <p class="text-sm leading-5 text-muted-foreground">(description)</p>
        )
    })
}
    