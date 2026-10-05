use topcoat::{Result, router::page, view::{View, view}};
use crate::frontend::layouts::nav_layout::nav_layout;

#[page("/fahrtenbuch")]
pub async fn fahrtenbuch_page() -> Result<impl View> {
    Ok(view! { 
        nav_layout(
            <h2> "Fahrtenbuch Page" </h2>
        )
    })
}