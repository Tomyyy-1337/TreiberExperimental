use topcoat::{Result, asset::{Asset, asset}, router::{Slot, layout}, view::{View, view}};

use crate::frontend::nav::nav;

pub const STYLESHEET: Asset = asset!("../../static/style.css");

#[layout("/")]
pub async fn main_layout(slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <head>
            <title>"Ruder Cam Beta"</title>
            topcoat::runtime::script()
            // topcoat::dev::script()
            <link rel="stylesheet" type="text/css" href=(STYLESHEET)>
        </head>
        <html>
            <body>
                (slot)
            </body>
        </html>
    })
}

#[layout]
pub async fn nav_layout(slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <h1> "Ruder Cam Beta" </h1>
        nav()
        (slot)
    })
}

#[layout]
pub async fn back_button_layout(slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <a href="javascript:history.back()"> "Go Back" </a>
        (slot)
    })
}