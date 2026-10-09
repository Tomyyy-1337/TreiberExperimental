use tokio::fs;
use topcoat::{Result, asset::{Asset, asset}, context::Cx, cookie::{Cookies, cookies}, router::{Slot, layout}, view::{View, view}};

pub const STREAM_SCRIPT: Asset = asset!("static/stream.js");
pub const LOAD_MAP: Asset = asset!("static/load_map.js");

#[layout("/")]
pub async fn main_layout(slot: Slot<'_>, cx: &Cx) -> Result<impl View> {
    let cookies = cookies(cx);
    let theme_cookie = cookies.get("theme");

    let mut plugins_dir = fs::read_dir("plugins").await.unwrap();
    let mut plugins_list: Vec<String> = Vec::new();
    while let Ok(Some(entry)) = plugins_dir.next_entry().await {
        plugins_list.push(entry.file_name().to_string_lossy().to_string());
    }

    Ok(view! {
        <!DOCTYPE html>
        <head>
            <title>"Ruder Cam Beta"</title>
            <meta name="viewport" content="width=device-width, initial-scale=1.0">

            <script type="module" src=(LOAD_MAP)></script>
            <script type="module" src=(STREAM_SCRIPT)></script>
            topcoat::runtime::script()
            <link rel="stylesheet" href=(topcoat::tailwind::stylesheet!())>
            
            for plugin in &plugins_list {
                <script type="module" src=(format!("/plugins/{}", plugin))></script>
            }
        </head>
        <html class=(theme_cookie.as_ref().map(|c| c.value()).unwrap_or("dark"))>
            <body class="min-h-screen bg-background text-foreground antialiased">
                <main class="mx-auto w-full max-w-4xl px-2">
                    (slot)
                </main>
            </body>
        </html>
    })
}
