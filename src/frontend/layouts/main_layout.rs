use topcoat::{Result, asset::{Asset, asset}, context::Cx, cookie::{Cookies, cookies}, router::{Slot, layout}, view::{View, view}};

pub const SCRIPT: Asset = asset!("../../../static/script.js");

#[layout("/")]
pub async fn main_layout(slot: Slot<'_>, cx: &Cx) -> Result<impl View> {
    let cookies = cookies(cx);
    let theme_cookie = cookies.get("theme");

    Ok(view! {
        <!DOCTYPE html>
        <head>
            <title>"Ruder Cam Beta"</title>
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            
            <link rel="stylesheet" href=(topcoat::tailwind::stylesheet!())>

            topcoat::runtime::script()
            <script type="module" src=(SCRIPT)></script>
        </head>
        <html class=(theme_cookie.as_ref().map(|c| c.value()).unwrap_or("dark"))>
            <body class="min-h-screen bg-background text-foreground antialiased">
                (slot)
            </body>
        </html>
    })
}
