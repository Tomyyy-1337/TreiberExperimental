use topcoat::{Result, asset::{Asset, asset}, context::Cx, cookie::{Cookies, cookies}, router::{Slot, layout}, view::{View, view}};

pub const STREAM_SCRIPT: Asset = asset!("../../../static/stream.js");
pub const PMTILES: Asset = asset!("https://unpkg.com/pmtiles@4.5.0/dist/pmtiles.js");
pub const LOAD_MAP: Asset = asset!("../../../static/load_map.js");

#[layout("/")]
pub async fn main_layout(slot: Slot<'_>, cx: &Cx) -> Result<impl View> {
    let cookies = cookies(cx);
    let theme_cookie = cookies.get("theme");

    Ok(view! {
        <!DOCTYPE html>
        <head>
            <title>"Ruder Cam Beta"</title>
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            
            <script src=(PMTILES)></script>
            <script type="module" src=(LOAD_MAP)></script>
            <script type="module" src=(STREAM_SCRIPT)></script>
            topcoat::runtime::script()
            <link rel="stylesheet" href=(topcoat::tailwind::stylesheet!())>
        </head>
        <html class=(theme_cookie.as_ref().map(|c| c.value()).unwrap_or("dark"))>
            <body class="min-h-screen bg-background text-foreground antialiased">
                (slot)
            </body>
        </html>
    })
}
