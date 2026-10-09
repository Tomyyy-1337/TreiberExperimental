use topcoat::{Result, router::{href, page}, runtime::{PrefetchMode, link}, view::{View, attributes, component, view}};
use crate::{backend::fahrtenbuch::{FAHRTENBUCH, FinishedFahrt, Indexed}, frontend::{layouts::nav_layout::nav_layout, pages::fahrtenbuch_eintrag_page::{Id, fahrtenbuch_eintrag}}};

#[page("/fahrtenbuch")]
pub async fn fahrtenbuch_page() -> Result<impl View> { 
    Ok(view! { 
        nav_layout(
            <div class="my-4 grid gap-3">
                #[key(id)]
                for Indexed::<FinishedFahrt> { id, entry } in FAHRTENBUCH.entries.iter().rev() {
                    fahrtenbuch_eintragen(id: *id, entry: entry)
                }
            </div>
        )
    })
}

#[component]
async fn fahrtenbuch_eintragen(id: u32, entry: &FinishedFahrt) -> Result<impl View> {
    let time_secs = entry.dauer.as_secs();
    let time_mins = time_secs / 60;
    
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
                    <span class="py-1 rounded-full bg-background px-3 text-sm font-medium text-muted-foreground">(&entry.strecke_km) " km"</span>
                    <span class="py-1 rounded-full bg-background px-3 text-sm font-medium text-muted-foreground"> (time_mins) ":" ( format!("{:02}", time_secs % 60)) " min"</span>
                </div>
            </article>
        )
    })
}