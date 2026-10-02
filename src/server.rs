use topcoat::{Result, asset::{Asset, asset}, context::Cx, router::{Slot, layout, page, request::uri}, runtime::{connected, procedure, shard, signal}, view::{View, attributes, class, component, emit, live, view}};

use crate::{SETTINGS, SHARED_STATE};

const STYLESHEET: Asset = asset!("../static/style.css");
const MEDIAMTX_READER: Asset = asset!("../static/mediamtx-reader.js");
const MEDIAMTX_WEBRTC_URL: &str = "http://localhost:8889/camera/whep";

#[layout("/")]
async fn main_layout(slot: Slot<'_>) -> Result<impl View> {
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
async fn nav_layout(slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <h1> "Ruder Cam Beta" </h1>
        nav()
        (slot)
    })
}

#[layout]
async fn back_button_layout(slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <a href="javascript:history.back()"> "Go Back" </a>
        (slot)
    })
}

#[component]
async fn nav(cx: &Cx) -> Result<impl View> {
    let pages = vec![
        ("/", "Home"),
        ("/fahrtenbuch", "Fahrtenbuch"),
        ("/settings", "Settings"),
    ];

    Ok(view! {
        <nav class="navbar"> 
            for (path, label) in pages {
                let attributes = attributes!(
                    class=(class!(
                        "navbar-link",
                        "active" if uri(cx).path() == path 
                    ))
                    href=(path)
                );

                <a (attributes)> (label) </a>
            }
        </nav>
    })
}

#[page("/")]
async fn camera() -> Result<impl View> {
    Ok(view! { nav_layout(slot: Slot::new(view! {
        <p> "Camera Page" </p>

        stream_player()

        live_data()
    }))})
}

#[component]
async fn stream_player() -> Result<impl View> {
    Ok(view! {
        <div
            class="live-video-shell"
            data-live-config=""
            data-url=(MEDIAMTX_WEBRTC_URL)
        >
            <video data-live-video="" autoplay="" playsinline="" muted=""></video>
        </div>
        <script type="module" src=(MEDIAMTX_READER)></script>
    })
}

#[component]
async fn live_data(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <p> "Counter: " 
            (live! {
                let mut timer = tokio::time::interval(std::time::Duration::from_millis(1000));
                loop {
                    let token = emit! {( SHARED_STATE.counter )}?;

                    // Create websocket connection if not already connected
                    if !connected(cx) {
                        break Ok(token);
                    }

                    timer.tick().await;
                }
            })
        </p>
    })
}

#[page("/fahrtenbuch")]
async fn fahrtenbuch() -> Result<impl View> {
    Ok(view! { nav_layout(slot: Slot::new(view! { 
        <p> "Fahrtenbuch Page" </p>
    }))})
}


#[page("/settings")]
async fn settings() -> Result<impl View> {
    Ok(view! { nav_layout(slot: Slot::new(view! { 
        <p> "Settings Page" </p>
        <a href="/settings/camera"> "Camera Settings" </a>
    }))})
}

#[page("/settings/camera")]
async fn settings_camera(cx: &Cx) -> Result<impl View> {
    let hdr_signal = signal(cx, || SETTINGS.hdr);

    Ok(view! { back_button_layout(slot: Slot::new(view! {
        <h2> "Settings Camera Page" </h2>

        <h3> "HDR" </h3>

        <p> "HDR is " $(if hdr_signal.get() { "enabled" } else { "disabled" }) </p>

        <button 
            @click=$(async |_event| { 
                let new_value = toggle_hdr().await; 
                hdr_signal.set(new_value);
            })>
            $(if hdr_signal.get() { "Disable HDR" } else { "Enable HDR" })
        </button>
    }))})
}

#[procedure]
async fn toggle_hdr() -> Result<bool> {
    let current = SETTINGS.hdr;
    SETTINGS.modify(|s| s.hdr = !current);
    println!("HDR toggled, new value: {}", !current);
    Ok(!current)
}
