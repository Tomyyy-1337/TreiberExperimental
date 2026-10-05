use topcoat::{Result, router::{Slot, href, page}, runtime::link, view::{Child, View, component, view}};

use crate::frontend::{layouts::nav_layout::nav_layout, settings_subpages::{settings_anzeige_page::settings_anzeige_page, settings_camera_page::settings_camera_page}};

#[page("/settings")]
pub async fn settings_page() -> Result<impl View> {
    Ok(view! { nav_layout(slot: Slot::new(view! { 
        <h2> "Settings Page" </h2>
        
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
    }))})
}

#[component]
pub async fn settings_wrapper(#[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <div style="padding: 10px;">
            (child)
        </div>
    })
}
    