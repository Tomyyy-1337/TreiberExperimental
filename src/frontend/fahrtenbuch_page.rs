use topcoat::{Result, router::{Slot, page}, view::{View, view}};
use crate::frontend::layouts::nav_layout::nav_layout;

#[page("/fahrtenbuch")]
pub async fn fahrtenbuch_page() -> Result<impl View> {
    Ok(view! { nav_layout(slot: Slot::new(view! { 
        <h2> "Fahrtenbuch Page" </h2>
    }))})
}