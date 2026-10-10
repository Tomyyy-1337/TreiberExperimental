use topcoat::{Result, router::{href, page}, runtime::{PrefetchMode, link}, view::{View, attributes, component, view}};
use crate::{backend::fahrtenbuch::{FAHRTENBUCH, Indexed}, backend::fahrt::{Fahrt, Finished}, frontend::{layouts::nav_layout::nav_layout, pages::fahrtenbuch_eintrag_page::{Id, fahrtenbuch_eintrag}}};

#[page("/fahrtenbuch")]
pub async fn fahrtenbuch_page() -> Result<impl View> { 
    Ok(view! { 
        nav_layout(
            <div class="my-4 grid gap-3">
                if FAHRTENBUCH.entries.is_empty() {
                    <p class="text-center text-muted-foreground p-8 border border-border rounded-2xl bg-card">
                        "Keine Fahrten vorhanden." 
                    </p>
                } else {
                    #[key(id)]
                    for Indexed::<Fahrt<Finished>> { id, entry } in FAHRTENBUCH.entries.iter().rev() {
                        fahrtenbuch_eintragen(id: *id, entry: entry)
                    } 
                }
            </div>
        )
    })
}

#[component]
async fn fahrtenbuch_eintragen(id: u32, entry: &Fahrt<Finished>) -> Result<impl View> {
    Ok(view! {
        link(
            href: href!(fahrtenbuch_eintrag, Id(id)),
            attrs: attributes!(class="group block text-inherit no-underline"),
            prefetch: PrefetchMode::Never,
            
            <article class="grid gap-2 rounded-2xl border border-border bg-card p-3 transition-colors group-active:bg-background">
                <div class="flex items-start justify-between gap-3">
                    <h3 class="text-base font-semibold text-card-foreground"> "Fahrt am " (&entry.start_time)</h3>
                    <span class="shrink-0 rounded-md px-2 text-sm font-semibold text-muted-foreground"> "Mehr" </span>
                </div>

                <div class="flex flex-wrap items-center gap-2">
                    <span class="py-1 rounded-full bg-background px-3 text-sm font-medium text-muted-foreground">(entry.formated_distance_km(2)) " km"</span>
                    <span class="py-1 rounded-full bg-background px-3 text-sm font-medium text-muted-foreground"> (entry.formated_duration_mm_ss()) " min"</span>
                </div>
            </article>
        )
    })
}