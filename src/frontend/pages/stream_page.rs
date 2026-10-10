use topcoat::{Result, context::Cx, router::page, runtime::{Event, connected, procedure, signal}, view::{Child, EmitToken, View, component, emit, live, view}};

use crate::{backend::{fahrt::{FAHRT_STATE, FahrtStatus, FahrtDurationTrait}, fahrtenbuch::FAHRTENBUCH}, frontend::{layouts::nav_layout::nav_layout, settings_subpages::settings_camera_page::settings_camera_component}};

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

#[component]
async fn fahrt_info(cx: &Cx) -> Result<impl View> {
    Ok(live! {
        let mut timer = tokio::time::interval(std::time::Duration::from_secs(1));

        for i in 0u64.. {
            timer.tick().await;

            let (dauer, distanz) = match &*FAHRT_STATE {
                FahrtStatus::Active(a) => (a.formated_duration_mm_ss(), a.formated_distance_km(2)),
                _ => ("--:--".to_string(), "--.--".to_string()),
            };
            let token = emit!{
                <section class="my-4 rounded-2xl border border-border bg-card p-2">
                    fahrt_button(key: i)

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
async fn fahrt_button(cx: &Cx, key: u64) -> Result<impl View> {
    let active = signal(&cx.keyed(key), || FAHRT_STATE.is_active());
    Ok(view! {
        <div :hidden=$(!active.get())>
            <button
                class="flex h-12 w-full items-center justify-center rounded-xl bg-destructive px-4 text-base font-semibold text-destructive-foreground shadow-xs transition-colors hover:bg-destructive/90 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ring active:scale-[0.99]"
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
                class="flex h-12 w-full items-center justify-center rounded-xl bg-primary px-4 text-base font-semibold text-primary-foreground shadow-xs transition-colors hover:bg-primary/90 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ring active:scale-[0.99]"
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