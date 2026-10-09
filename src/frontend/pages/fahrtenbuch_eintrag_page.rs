use crate::{
    backend::fahrtenbuch::{FAHRTENBUCH, FinishedFahrt},
    frontend::pages::fahrtenbuch_page::fahrtenbuch_page,
};
use topcoat::{
    Result, context::Cx, router::{error::RouterErrorExt, href, page, path_param}, runtime::{Event, PrefetchMode, link, procedure, signal}, view::{StaticClass, View, attributes, class, component, view},
};

path_param!(pub id: u32);

#[page("/fahrtenbuch/{id}")]
pub async fn fahrtenbuch_eintrag(cx: &Cx) -> Result<impl View> {
    let post_id = *path_param::<Id>(cx).ok_or_not_found()?;
    let entry = FAHRTENBUCH
        .get(post_id)
        .ok_or_redirect(href!(fahrtenbuch_page).resolve(cx))?;

    Ok(view! {
        title_card(post_id: post_id, entry: entry)
        
        map_component(id: post_id, entry: entry)

        fahrt_stats(fahrt: entry)

        delete_button(post_id: post_id)
    })
}

#[component]
async fn title_card(post_id: u32, entry: &FinishedFahrt) -> Result<impl View> {
    let next_id = FAHRTENBUCH.get_id_of_next(post_id);
    let previous_id = FAHRTENBUCH.get_id_of_previous(post_id);

    Ok(view! {
        <section class="grid gap-4 rounded-2xl border border-border bg-card p-4">
            <div class="flex items-start justify-between gap-4">
                <div class="grid gap-1">
                    <span class="text-[0.66rem] font-bold uppercase tracking-widest text-muted-foreground">"Fahrt am"</span>
                    <h2 class="text-xl font-semibold tracking-tight text-card-foreground">( &entry.start_time )</h2>
                </div>
                link(
                    href: href!(fahrtenbuch_page),
                    attrs: attributes!{
                        class= "inline-flex h-12 w-12 shrink-0 items-center justify-center rounded-full border border-border bg-card text-3xl font-medium leading-none text-muted-foreground"
                        aria-label= "Zurück zum Fahrtenbuch"
                        title= "Zurück"
                    },
                    "×"
                )
            </div>

            if next_id.is_some() || previous_id.is_some() {
                <div class="flex items-center border-t border-border pt-3">
                    if let Some(next_id) = next_id {
                        link(
                            href: href!(fahrtenbuch_eintrag, Id(next_id)),
                            attrs: attributes!(class="mr-auto rounded-md px-2 py-1 text-sm font-semibold text-primary transition-colors hover:bg-background"),
                            prefetch: PrefetchMode::Never,
                            "Nächste Fahrt"
                        )
                    }

                    if let Some(previous_id) = previous_id {
                        link(
                            href: href!(fahrtenbuch_eintrag, Id(previous_id)),
                            attrs: attributes!(class="ml-auto rounded-md px-2 py-1 text-sm font-semibold text-primary transition-colors hover:bg-background"),
                            prefetch: PrefetchMode::Never,
                            "Vorherige Fahrt"
                        )
                    }
                </div>
            }
        </section>
    })
}

#[component]
async fn fahrt_stats(fahrt: &FinishedFahrt) -> Result<impl View> {
    let duration = format!(
        "{:02}:{:02}:{:02}",
        fahrt.dauer.as_secs() / 3600,
        (fahrt.dauer.as_secs() % 3600) / 60,
        fahrt.dauer.as_secs() % 60
    );
    let avg_split_time = if fahrt.strecke_km > 0.0 {
        format!("{:.2}", fahrt.dauer.as_secs_f64() / (fahrt.strecke_km * 1000.0 / 500.0))
    } else {
        "N/A".to_string()
    };
    let avg_speed = if fahrt.dauer.as_secs_f64() > 0.0 {
        format!("{:.2}",fahrt.strecke_km / (fahrt.dauer.as_secs_f64() / 3600.0))
    } else {
        "N/A".to_string()
    };
    let avg_stroke_rate = if fahrt.dauer.as_secs_f64() > 0.0 {
        format!("{:.2}",fahrt.schläge as f64 / (fahrt.dauer.as_secs_f64() / 60.0))
    } else {
        "N/A".to_string()
    };

    Ok(view! {
        <section class="my-4 grid grid-cols-2 gap-3 rounded-2xl border border-border bg-card p-2">
            <div class="col-span-2 grid gap-1 rounded-xl border border-border bg-background p-4">
                <span class="text-xs font-semibold uppercase tracking-[0.08em] text-muted-foreground">"Gesamtstrecke"</span>
                <div class="flex items-baseline gap-1 text-2xl font-semibold tracking-tight text-card-foreground">
                    <span> ( &fahrt.strecke_km ) </span>
                    <span class="text-sm font-medium text-muted-foreground">" km"</span>
                </div>
            </div>

            stat_card(title: "Dauer", value: &duration)
            stat_card(title: "Ø Splittime", value: &avg_split_time, unit: " / 500 m")
            stat_card(title: "Ø Geschwindigkeit", value: &avg_speed, unit: "km/h")
            stat_card(title: "Ø Schlagzahl", value: &avg_stroke_rate, unit: "bpm")
        </section>
    })
}

#[component]
async fn stat_card(title: &str, value: &str, #[default] unit: &str) -> Result<impl View> {
    Ok(view! {
        <div class="grid gap-1 rounded-xl border border-border bg-background p-3">
            <span class="text-xs font-semibold text-muted-foreground">( title )</span>
            <span class="font-semibold text-card-foreground">( value ) <span class="text-sm font-medium text-muted-foreground">( unit )</span></span>
        </div>
    })
}

#[component]
async fn map_component(id: u32, entry: &FinishedFahrt) -> Result<impl View> {
    let gps_data_json = serde_json::to_string(&entry.position_history).unwrap();
    let available_maps = std::fs::read_dir("./maps")?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect::<Vec<String>>();
    let available_maps_json = serde_json::to_string(&available_maps).unwrap();

    Ok(view! {
        <section class="pointer-events-none my-4 overflow-hidden rounded-2xl bg-card [&_.maplibregl-canvas]:block border border-border">

            if !entry.position_history.is_empty() {
                <div
                    hidden=(entry.position_history.is_empty())
                    id=(format!("map{}", id))
                    class="h-[400px] w-[calc(100%+1px)]"
                    gps_data=(gps_data_json)
                    available_maps=(available_maps_json)
                ></div>
            } else {
                <div
                    class="h-[400px] w-[calc(100%+1px)] flex items-center justify-center text-muted-foreground"
                >
                    "Keine GPS-Daten verfügbar."
                </div>
            }
        </section>
    })
}

#[component]
async fn delete_button(cx: &Cx, post_id: u32) -> Result<impl View> {
    let löschen_bestätigen = signal(&cx.keyed(post_id), || false);

    Ok(view! {
        <section class="my-4 grid gap-3 rounded-2xl border border-destructive/40 bg-card p-4">
            <div class="grid gap-1">
                <h3 class="text-sm font-semibold text-card-foreground">"Eintrag verwalten"</h3>
                <p class="text-sm text-muted-foreground">"Diesen Fahrtenbucheintrag dauerhaft löschen."</p>
            </div>

            <div :hidden=$(löschen_bestätigen.get())>
                <p
                    class="bg-destructive block w-full rounded-md px-3 py-2 text-center text-sm font-semibold text-destructive-foreground"
                    @click=$(async |_event: Event| {
                        löschen_bestätigen.set(true);
                    })
                >
                    "Eintrag löschen"
                </p>
            </div>
            <div :hidden=$(!löschen_bestätigen.get())>
                link(
                    href: href!(fahrtenbuch_page),
                    attrs: attributes!(
                        class="bg-destructive-confirm block w-full rounded-md px-3 py-2 text-center text-sm font-semibold text-destructive-foreground"
                        @click=$(async |_event: Event| {
                            delete_fahrtenbuch_entry(post_id).await;
                        })
                    )
                    "Löschen bestätigen"
                )
            </div>
        </section>
    })
}

#[procedure("/api/delete_fahrtenbuch_entry")]
async fn delete_fahrtenbuch_entry(entry_id: u32) -> Result<()> {
    FAHRTENBUCH.modify(|state| {
        state.entries.retain(|entry| entry.id != entry_id);
    });
    Ok(())
}