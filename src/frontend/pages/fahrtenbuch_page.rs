use topcoat::{Result, context::Cx, router::page, runtime::{Signal, procedure, shard, signal}, view::{View, component, view}};
use crate::{FAHRTENBUCH_STATE, FahrtenbuchEntry, frontend::layouts::nav_layout::nav_layout};

#[page("/fahrtenbuch")]
pub async fn fahrtenbuch_page() -> Result<impl View> { 
    Ok(view! { 
        nav_layout(
            <h2> "Fahrtenbuch Page" </h2>

            #[key(entry.id)]
            for entry in FAHRTENBUCH_STATE.entries.iter() {
                fahrtenbuch_eintragen(entry: entry)
            }
        )
    })
}

#[component]
async fn fahrtenbuch_eintragen(cx: &Cx, entry: &FahrtenbuchEntry) -> Result<impl View> {
    let shown = signal(cx, || true);
    let id = entry.id;

    Ok(view! {
        <div :hidden=$(!shown.get())>  
            <p> ( format!("Gesamtstrecke: {}", entry.gesamtstrecke) ) </p>
            
            <button 
            @click=$(async |_event| {
                delete_fahrtenbuch_entry(id).await;
                shown.set(false);
            })
            > "Delete" </button>
        </div>
    })
}

#[procedure("/api/delete_fahrtenbuch_entry")]
async fn delete_fahrtenbuch_entry(entry_id: u32) -> Result<()> {
    FAHRTENBUCH_STATE.modify(|state| {
        state.entries.retain(|entry| entry.id != entry_id);
    });
    Ok(())
}