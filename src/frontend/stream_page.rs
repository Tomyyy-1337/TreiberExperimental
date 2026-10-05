use topcoat::{Result, context::Cx, router::page, runtime::connected, view::{View, component, emit, live, view}};

use crate::{SHARED_STATE, frontend::{layouts::nav_layout::nav_layout}};

const MEDIAMTX_WEBRTC_URL: &str = "http://localhost:8889/camera/whep";

#[page("/")]
pub async fn camera_page() -> Result<impl View> {
    Ok(view! { 
        nav_layout(
            stream_player()
            
            live_data()
        )
    })
}

#[component]
pub async fn stream_player() -> Result<impl View> {
    Ok(view! {
        <section>
            <div
                class="aspect-video w-full max-w-[960px] bg-background"
                data-live-config=""
                data-url=(MEDIAMTX_WEBRTC_URL)
            >
                <video class="block h-full w-full object-contain" data-live-video="" autoplay="" playsinline="" muted=""></video>
            </div>
        </section>
    })
}

#[component]
pub async fn live_data(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <p class="my-4 text-sm text-muted-foreground"> "Counter: "
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
