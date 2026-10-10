use tokio::sync::watch::{self, Receiver};
use topcoat::{Result, context::{Cx, app_context}, router::page, runtime::{Event, connected, procedure, shard}, view::{Child, EmitToken, View, class, component, emit, live, view}};

use crate::{backend::{fahrt::{self, FahrtStatus}, fahrtenbuch::FAHRTENBUCH}, frontend::{layouts::nav_layout::nav_layout, settings_subpages::settings_camera_page::settings_camera_component}};

#[page("/")]
pub async fn camera_page() -> Result<impl View> {
    Ok(view! { 
        nav_layout(
            stream_player()

            collapsable_section(
                title: "Kamera Einstellungen",
                settings_camera_component()
            )

            fahrt_info()
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

#[shard]
async fn fahrt_info(cx: &Cx) -> Result<impl View> {
    let mut fahrt_state = app_context::<Receiver<fahrt::FahrtStatus>>(cx).clone();
    fahrt_state.mark_changed();

    Ok(live! {
        while let Ok(()) = fahrt_state.changed().await {
            let (is_active, dauer, distanz) = match &*fahrt_state.borrow() {
                FahrtStatus::Active(a) => (true, a.formated_duration_mm_ss(), a.formated_distance_km(2)),
                _ => (false, "--:--".to_string(), "--.--".to_string())
            };

            let token = emit!{
                <section class="my-4 rounded-2xl border border-border bg-card p-2">
                    fahrt_button(is_active: is_active)

                    <section class="my-4 grid grid-cols-2 gap-3 rounded-2xl border border-border bg-card p-2">
                        <div class="grid gap-1 rounded-xl border border-border bg-background p-3">
                            <span class="text-xs font-semibold text-muted-foreground">"Dauer"</span>
                            <span class="text-2xl font-semibold tabular-nums tracking-tight text-card-foreground">(dauer)</span>
                        </div>
                        <div class="grid gap-1 rounded-xl border border-border bg-background p-3">
                            <span class="text-xs font-semibold text-muted-foreground">"Distanz"</span>
                            <span class="text-2xl font-semibold tabular-nums tracking-tight text-card-foreground">(distanz)</span>
                        </div>
                    </section>
                </section>
            }?;

            if !connected(cx) {
                return Ok(token);
            }
        }
        Ok(EmitToken)
    })
}

#[component]
async fn fahrt_button(is_active: bool) -> Result<impl View> {
    Ok(view! {
        <button
            class=(class!("bg-destructive" if is_active else "bg-primary" ,"flex h-12 w-full items-center justify-center rounded-xl px-4 text-base font-semibold text-destructive-foreground active:scale-[0.99]"))
            @click=$(async |_event: Event| {
                if is_active {
                    stop_fahrt().await;
                } else {
                    let current_time = raw!("new Date().toLocaleString('de-DE', { dateStyle: 'short', timeStyle: 'short' })", String::new());
                    start_fahrt(current_time).await;
                }
            })
        > 
            if is_active { "Fahrt beenden" } else { "Fahrt starten" }
        </button>
    })
}

#[procedure("/api/start_fahrt")]
async fn start_fahrt(cx: &Cx, current_time: String) -> Result<()> {
    let fahrt_status = app_context::<watch::Sender<FahrtStatus>>(&cx);
    fahrt_status.send(FahrtStatus::new_active(current_time)).unwrap();
    Ok(())
}

#[procedure("/api/stop_fahrt")]
async fn stop_fahrt(cx: &Cx) -> Result<()> {
    let fahrt_status = app_context::<watch::Sender<FahrtStatus>>(&cx);

    let fahrt = fahrt_status.send_replace(FahrtStatus::Inactive);
    if let FahrtStatus::Active(a) = fahrt {
        let finished_fahrt = a.finish();
        FAHRTENBUCH.modify(|f| f.add_entry(finished_fahrt));
    }
    Ok(())
}