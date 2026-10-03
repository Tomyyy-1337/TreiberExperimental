use topcoat::{Result, asset::{Asset, asset}, context::Cx, cookie::{Cookies, cookies}, router::{Slot, page}, runtime::{connected, signal}, view::{View, class, component, emit, live, view}};

use crate::{SHARED_STATE, frontend::{icons::battery_status::battery_status, layouts::nav_layout::nav_layout}};

pub const MEDIAMTX_READER: Asset = asset!("../../static/mediamtx-reader.js");
const MEDIAMTX_WEBRTC_URL: &str = "http://localhost:8889/camera/whep";

#[page("/")]
async fn camera() -> Result<impl View> {
    Ok(view! { nav_layout(slot: Slot::new(view! {
        stream_player()

        battery_status()
        
        live_data()
    }))})
}

#[component]
pub async fn stream_player() -> Result<impl View> {
    Ok(view! {
        <section>
            <div
                class="stream_player-live-video-shell"
                data-live-config=""
                data-url=(MEDIAMTX_WEBRTC_URL)
            >
                <video data-live-video="" autoplay="" playsinline="" muted=""></video>
            </div>
        </section>

        <script type="module" src=(MEDIAMTX_READER)></script>
    })
}

#[component]
pub async fn live_data(cx: &Cx) -> Result<impl View> {
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
