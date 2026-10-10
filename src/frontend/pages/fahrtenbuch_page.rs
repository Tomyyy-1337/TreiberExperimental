use tokio::sync::watch::Receiver;
use topcoat::{Result, context::{Cx, app_context}, router::{href, page}, runtime::{PrefetchMode, link}, view::{View, attributes, component, view}};
use crate::{backend::{fahrtenbuch::{Fahrtenbuch, Indexed}}, frontend::{layouts::nav_layout::nav_layout, pages::fahrtenbuch_eintrag_page::{Id, fahrtenbuch_eintrag}}};

#[page("/fahrtenbuch")]
pub async fn fahrtenbuch_page(cx: &Cx) -> Result<impl View> { 
    let fahrtenbuch = app_context::<Receiver<Fahrtenbuch>>(cx);

    let fahrtenbuch = fahrtenbuch.borrow();
    let is_empty = fahrtenbuch.entries.is_empty();

    let entries = fahrtenbuch.entries
        .iter()
        .rev()
        .map(|Indexed { id, entry }| {(
            *id,
            entry.start_time.clone(),
            entry.formated_distance_km(2),
            entry.formated_duration_mm_ss(),
        )})
        .collect::<Vec<_>>();

    drop(fahrtenbuch);

    Ok(view! { 
        nav_layout(
            <div class="my-4 grid gap-3">
                if is_empty {
                    <p class="text-center text-muted-foreground p-8 border border-border rounded-2xl bg-card">
                        "Keine Fahrten vorhanden." 
                    </p>
                } else {
                    #[key(id)]
                    for (id, start_time, distance, duration) in entries {
                        fahrtenbuch_eintragen(
                            id: id,
                            start_time: start_time.clone(),
                            distance: distance.clone(),
                            duration: duration.clone(),
                        )
                    }
                }
            </div>
        )
    })
}

#[component]
async fn fahrtenbuch_eintragen(id: u32, start_time: String, distance: String, duration: String) -> Result<impl View> {
    Ok(view! {
        link(
            href: href!(fahrtenbuch_eintrag, Id(id)),
            attrs: attributes!(class="group block text-inherit no-underline"),
            prefetch: PrefetchMode::Never,
            
            <article class="grid gap-2 rounded-2xl border border-border bg-card p-3 transition-colors group-active:bg-background">
                <div class="flex items-start justify-between gap-3">
                    <h3 class="text-base font-semibold text-card-foreground"> "Fahrt am " (start_time)</h3>
                    <span class="shrink-0 rounded-md px-2 text-sm font-semibold text-muted-foreground"> "Mehr" </span>
                </div>

                <div class="flex flex-wrap items-center gap-2">
                    <span class="py-1 rounded-full bg-background px-3 text-sm font-medium text-muted-foreground">(distance) " km"</span>
                    <span class="py-1 rounded-full bg-background px-3 text-sm font-medium text-muted-foreground"> (duration) " min"</span>
                </div>
            </article>
        )
    })
}