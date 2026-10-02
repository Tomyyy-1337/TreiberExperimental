use topcoat::{Result, asset::{Asset, asset}, context::Cx, cookie::{Cookies, cookies}, router::{Slot, layout}, view::{View, view}};

use crate::frontend::navbar::navbar;

pub const STYLESHEET: Asset = asset!("../../static/style.css");
pub const SCRIPT: Asset = asset!("../../static/script.js");

#[layout("/")]
pub async fn main_layout(slot: Slot<'_>, cx: &Cx) -> Result<impl View> {
    let cookies = cookies(cx);
    let theme_cookie = cookies.get("theme");

    Ok(view! {
        <!DOCTYPE html>
        <head>
            <title>"Ruder Cam Beta"</title>
            topcoat::runtime::script()
            // topcoat::dev::script()
            <link rel="stylesheet" type="text/css" href=(STYLESHEET)>
            <script type="module" src=(SCRIPT)></script>
        </head>
        <html data-theme=(theme_cookie.as_ref().map(|c| c.value()).unwrap_or("light"))>
            <body>
                (slot)
            </body>
        </html>
    })
}


#[layout]
pub async fn nav_layout(slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <div class="container">
            <section>
                <h1> "Ruder Cam Beta" </h1>
            </section>
            
            <section>
                navbar()
            </section>

            (slot)
        </div>
    })
}

#[layout]
pub async fn back_button_layout(slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <a href="/settings"> "Go Back" </a>
        (slot)
    })
}