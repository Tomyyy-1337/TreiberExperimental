use topcoat::{Result, router::{Slot, page}, view::{View, view}};
use crate::frontend::layout::nav_layout;

#[page("/fahrtenbuch")]
async fn fahrtenbuch() -> Result<impl View> {
    Ok(view! { nav_layout(slot: Slot::new(view! { 
        <p> "Fahrtenbuch Page" </p>
    }))})
}