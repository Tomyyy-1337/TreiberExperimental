use topcoat::{
    Result,
    asset::{Asset, asset},
    context::Cx,
    cookie::{Cookies, cookies},
    router::{Slot, layout},
    view::{View, view},
};

use crate::PLUGINS;

pub const STREAM_SCRIPT: Asset = asset!("static/stream.js");
pub const LOAD_MAP: Asset = asset!("static/load_map.js");

#[layout("/")]
pub async fn main_layout(slot: Slot<'_>, cx: &Cx) -> Result<impl View> {
    let cookies = cookies(cx);
    let theme_cookie = cookies.get("theme");

    Ok(view! {
        <!DOCTYPE html>
        <html class=(theme_cookie.as_ref().map(|c| c.value()).unwrap_or("dark"))>
            <head>
                <title>"Ruder Cam Beta"</title>
                <meta name="viewport" content="width=device-width, initial-scale=1.0">

                <script type="module" src=(LOAD_MAP)></script>
                <script type="module" src=(STREAM_SCRIPT)></script>
                topcoat::runtime::script()
                <link rel="stylesheet" href=(topcoat::tailwind::stylesheet!())>

                for path in &PLUGINS.paths {
                    <script type="module" src=(path)></script>
                }
            </head>
            <body class="min-h-screen bg-background text-foreground antialiased">
                <main class="mx-auto w-full max-w-4xl px-2 py-2">
                    (slot)
                </main>
            </body>
        </html>
    })
}
