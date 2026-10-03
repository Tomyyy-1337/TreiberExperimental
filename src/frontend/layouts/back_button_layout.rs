use topcoat::{Result, router::{Slot, layout}, view::{View, view}};

#[layout]
pub async fn back_button_layout(slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <a href="/settings"> "Go Back" </a>
        (slot)
    })
}