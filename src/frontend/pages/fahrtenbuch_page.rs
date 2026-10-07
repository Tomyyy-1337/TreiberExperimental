use topcoat::{Result, context::Cx, router::page, runtime::{procedure, signal}, view::{View, component, view}};
use crate::{backend::fahrtenbuch::{FAHRTENBUCH_STATE, Fahrt, IndexedFahrt}, frontend::layouts::nav_layout::nav_layout};

#[page("/fahrtenbuch")]
pub async fn fahrtenbuch_page() -> Result<impl View> { 
    Ok(view! { 
        nav_layout(
            <h2> "Fahrtenbuch Page" </h2>

            #[key(id)]
            for IndexedFahrt { id, entry } in FAHRTENBUCH_STATE.entries.iter() {
                fahrtenbuch_eintragen(id: *id, entry: entry)
            }
        )
    })
}

#[component]
async fn fahrtenbuch_eintragen(cx: &Cx, id: u32, entry: &Fahrt) -> Result<impl View> {
    let shown = signal(cx, || true);

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