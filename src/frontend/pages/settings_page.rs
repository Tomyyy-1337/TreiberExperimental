use topcoat::{Result, router::{href, page}, runtime::link, view::{Child, View, component, view}};

use crate::frontend::{layouts::nav_layout::nav_layout, settings_subpages::{settings_anzeige_page::settings_anzeige_page, settings_camera_page::settings_camera_page}};

#[page("/settings")]
pub async fn settings_page() -> Result<impl View> {
    Ok(view! { 
        nav_layout(
            <h2 class="text-xl font-semibold tracking-tight"> "Settings Page" </h2>
            
            settings_wrapper (
                link(
                    href: href!(settings_camera_page), 
                    "Kamera Einstellungen"
                )
            )
            settings_wrapper (
                link(
                    href: href!(settings_anzeige_page), 
                    "Anzeige Einstellungen"
                )
            )
        )
    })
}

#[component]
async fn settings_wrapper(#[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <div class="p-2.5">
            (child)
        </div>
    })
}
    