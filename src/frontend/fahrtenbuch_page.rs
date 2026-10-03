use topcoat::{Result, router::{Slot, page}, view::{View, view}};
use crate::frontend::layouts::nav_layout::nav_layout;

#[page("/fahrtenbuch")]
async fn fahrtenbuch() -> Result<impl View> {
    Ok(view! { nav_layout(slot: Slot::new(view! { 
        <h2> "Fahrtenbuch Page" </h2>
    }))})
}