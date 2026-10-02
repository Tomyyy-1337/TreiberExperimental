use topcoat::{Result, router::{Slot, page}, view::{View, view}};

use crate::frontend::layout::nav_layout;

#[page("/settings")]
pub async fn settings() -> Result<impl View> {
    Ok(view! { nav_layout(slot: Slot::new(view! { 
        <p> "Settings Page" </p>
        
        <a href="/settings/camera"> "Camera Settings" </a>
    }))})
}