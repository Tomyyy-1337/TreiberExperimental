use topcoat::{Result, router::page, view::{Child, View, component, view}};

use crate::frontend::{layouts::nav_layout::nav_layout, settings_subpages::settings_camera_page::settings_camera_component};

#[page("/")]
pub async fn camera_page() -> Result<impl View> {
    Ok(view! { 
        nav_layout(
            stream_player()

            collapsable_section(
                title: "Kamera Einstellungen",

                settings_camera_component()
            )
        )
    })
}

const MEDIAMTX_WEBRTC_URL: &str = "http://localhost:8889/camera/whep";

#[component]
async fn stream_player() -> Result<impl View> {
    Ok(view! {
        <section class="my-4 rounded-2xl border border-border bg-card p-2">
            <div
                class="aspect-video w-full max-w-[960px] overflow-hidden rounded-lg bg-black"
                data-live-config=""
                data-url=(MEDIAMTX_WEBRTC_URL)
            >
                <video class="block h-full w-full object-contain" data-live-video="" autoplay="" playsinline="" muted=""></video>
            </div>
        </section>
    })
}

#[component]
async fn collapsable_section(title: &str, child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <details class="my-4 rounded-2xl border-1 border-border bg-card [interpolate-size:allow-keywords] [&::details-content]:[block-size:0] [&::details-content]:overflow-hidden [&::details-content]:opacity-0 [&::details-content]:transition-all [&::details-content]:duration-300 [&::details-content]:ease-in-out [&::details-content]:[transition-behavior:allow-discrete] open:[&::details-content]:[block-size:auto] open:[&::details-content]:opacity-100">
            <summary class="cursor-pointer p-4 text-lg font-semibold"> 
                (title) 
            </summary>
            <div class="min-h-0 overflow-hidden rounded-b-2xl border-t border-border bg-background p-2">
                (child)
            </div>
        </details>
    })
}