use topcoat::{Result, router::{Slot, page}, view::{Child, View, component, view}};

use crate::frontend::layout::nav_layout;

#[page("/settings")]
pub async fn settings() -> Result<impl View> {
    Ok(view! { nav_layout(slot: Slot::new(view! { 
        <h2> "Settings Page" </h2>
        
        settings_wrapper (
            <a href="/settings/camera"> "Kamera Einstellungen" </a>
        )
        settings_wrapper (
            <a href="/settings/anzeige"> "Anzeige Einstellungen" </a>
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
    