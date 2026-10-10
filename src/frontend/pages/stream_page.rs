use topcoat::{Result, context::Cx, router::page, runtime::{Event, connected, procedure, signal}, view::{Child, View, component, emit, live, view}};

use crate::{backend::{fahrt::{FAHRT_STATE, FahrtStatus}, fahrtenbuch::FAHRTENBUCH}, frontend::{layouts::nav_layout::nav_layout, settings_subpages::settings_camera_page::settings_camera_component}};

#[page("/")]
pub async fn camera_page() -> Result<impl View> {
    Ok(view! { 
        nav_layout(
            stream_player()

            collapsable_section(
                title: "Kamera Einstellungen",

                settings_camera_component()
            )

            live_fahrt_button()
        )
    })
}

const MEDIAMTX_WEBRTC_URL: &str = "http://192.168.50.1:8889/stream/whep";

#[component]
async fn stream_player() -> Result<impl View> {
    Ok(view! {
        <section class="my-4 rounded-2xl border border-border bg-card p-2">
            <div
                class="aspect-video w-full overflow-hidden rounded-lg bg-black"
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
        <details class="my-4 rounded-2xl border border-border bg-card [interpolate-size:allow-keywords] details-content:[block-size:0] details-content:overflow-hidden details-content:opacity-0 details-content:transition-all details-content:duration-300 details-content:ease-in-out details-content:transition-discrete open:details-content:[block-size:auto] open:details-content:opacity-100">
            <summary class="cursor-pointer p-4 text-lg font-semibold"> 
                (title) 
            </summary>
            <div class="min-h-0 overflow-hidden rounded-b-2xl border-t border-border bg-background p-2">
                (child)
            </div>
        </details>
    })
}

#[component]
async fn fahrt_steuerung() -> Result<impl View> {    
    Ok(view! {
        <section class="my-4 rounded-2xl border border-border bg-card p-2">
            live_fahrt_button()

            <div>
                <span> "Dauer" </span>
                <span> "00:00:00" </span>
                <span> "km" </span>
            </div>
            <div>
                <span> "Distanz" </span>
                <span> "0" </span>
                <span> "km/h" </span>
            </div>
        </section>
    })
}

#[component]
async fn live_fahrt_button(cx: &Cx) -> Result<impl View> {    
    Ok(live! {
        let mut timer = tokio::time::interval(std::time::Duration::from_secs(1));
        let mut i = 0u64;
        let mut previous_state: Option<bool> = None;
        loop {
            timer.tick().await;

            if previous_state != Some(FAHRT_STATE.is_active()) || previous_state.is_none() {
                let token = emit!{
                    fahrt_button(key: i)
                }?;
                i += 1;
                if !connected(cx) {
                    break Ok(token);
                }
            }
            previous_state = Some(FAHRT_STATE.is_active());
        }    
    })
}

#[component]
async fn fahrt_button(cx: &Cx, key: u64) -> Result<impl View> {
    let active = signal(&cx.keyed(key), || FAHRT_STATE.is_active());
    Ok(view! {
        <div :hidden=$(!active.get())>
            <button 
                @click=$(async |_event: Event| {
                    active.set(false);
                    stop_fahrt().await;
                })
            >
                "Fahrt beenden"
            </button>
        </div>
        <div :hidden=$(active.get())>
            <button 
                @click=$(async |_event: Event| {
                    active.set(true);
                    let current_time = raw!(
                        "new Date().toLocaleString('de-DE', { dateStyle: 'short', timeStyle: 'short' })",
                        "Current time".to_string()
                    );
                    start_fahrt(current_time).await;
                })
            >   
                "Fahrt starten"
            </button>
        </div>
    })
}

#[procedure("/api/start_fahrt")]
async fn start_fahrt(current_time: String) -> Result<()> {
    FAHRT_STATE.set(FahrtStatus::new_active(current_time));
    Ok(())
}

#[procedure("/api/stop_fahrt")]
async fn stop_fahrt() -> Result<()> {
    match FAHRT_STATE.replace(FahrtStatus::Inactive) {
        FahrtStatus::Active(a) => {
            let finished_fahrt = a.finish();
            FAHRTENBUCH.modify(|f| f.add_entry(finished_fahrt));
        },
        FahrtStatus::Inactive => {},
    }
    Ok(())
}