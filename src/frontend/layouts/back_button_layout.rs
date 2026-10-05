use topcoat::{Result, router::{Slot, href, layout}, runtime::link, view::{View, view}};

use crate::frontend::settings_page::settings_page;

#[layout]
pub async fn back_button_layout(slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        link(href: href!(settings_page), "Go Back")
        (slot)
    })
}