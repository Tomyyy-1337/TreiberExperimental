use topcoat::{Result, context::Cx, router::{error::RouterErrorExt, href, page, path_param}, runtime::{Event, PrefetchMode, link, procedure}, view::{View, attributes, view}};
use crate::{backend::fahrtenbuch::FAHRTENBUCH, frontend::pages::fahrtenbuch_page::fahrtenbuch_page};

path_param!(pub post_id: u32);

#[page("/fahrtenbuch/{post_id}")]
pub async fn fahrtenbuch_eintrag(cx: &Cx) -> Result<impl View> {
    let post_id = *path_param::<PostId>(cx).ok_or_not_found()?;
    let entry = FAHRTENBUCH.get(post_id).ok_or_not_found()?;

    let next_id = FAHRTENBUCH.get_id_of_next(post_id);
    let previous_id = FAHRTENBUCH.get_id_of_previous(post_id);

    Ok(view! {
        <section class="my-4 mx-2 grid gap-4 rounded-2xl border border-border bg-card p-4 shadow-xs">
            <div class="flex items-start justify-between gap-4">
                <div class="grid gap-1">
                    <span class="text-[0.66rem] font-bold uppercase tracking-[0.1em] text-muted-foreground">"Fahrt am"</span>
                    <h2 class="text-xl font-semibold tracking-tight text-card-foreground">( &entry.start_time )</h2>
                </div>
                link(
                    href: href!(fahrtenbuch_page),
                    attrs: attributes!{
                        class= "inline-flex h-12 w-12 shrink-0 items-center justify-center rounded-full border border-border bg-card text-3xl font-medium leading-none text-muted-foreground"
                        aria-label= "Zurück zum Fahrtenbuch"
                        title= "Zurück"
                    }
                    "×"
                )
            </div>

            if next_id.is_some() || previous_id.is_some() {
                <div class="flex items-center justify-between border-t border-border pt-3">
                    if let Some(previous_id) = previous_id {
                        link(
                            href: href!(fahrtenbuch_eintrag, PostId(previous_id)),
                            attrs: attributes!(class="rounded-md px-2 py-1 text-sm font-semibold text-primary transition-colors hover:bg-background"),
                            prefetch: PrefetchMode::Viewport
                            "Vorherige Fahrt"
                        )
                    } else {
                        <span></span>
                    }

                    if let Some(next_id) = next_id {
                        link(
                            href: href!(fahrtenbuch_eintrag, PostId(next_id)),
                            attrs: attributes!(class="rounded-md px-2 py-1 text-sm font-semibold text-primary transition-colors hover:bg-background"),
                            prefetch: PrefetchMode::Viewport
                            "Nächste Fahrt"
                        )
                    }
                </div>
            }
        </section>

        <section class="my-4 mx-2 grid grid-cols-2 gap-3 rounded-2xl border border-border bg-card p-4 shadow-xs">
            <div class="col-span-2 grid gap-1 rounded-xl border border-border bg-background p-4">
                <span class="text-xs font-semibold uppercase tracking-[0.08em] text-muted-foreground">"Gesamtstrecke"</span>
                <div class="flex items-baseline gap-1 text-2xl font-semibold tracking-tight text-card-foreground">
                    <span> ( &entry.strecke_km ) </span>
                    <span class="text-sm font-medium text-muted-foreground">" km"</span>
                </div>
            </div>

            <div class="grid gap-1 rounded-xl border border-border bg-background p-3">
                <span class="text-xs font-semibold text-muted-foreground">"Dauer"</span>
                <span class="font-semibold text-card-foreground">"10:30" <span class="text-sm font-medium text-muted-foreground">" h"</span></span>
            </div>
            <div class="grid gap-1 rounded-xl border border-border bg-background p-3">
                <span class="text-xs font-semibold text-muted-foreground">"Ø Splittime"</span>
                <span class="font-semibold text-card-foreground">"10:30" <span class="text-sm font-medium text-muted-foreground">" / 500 m"</span></span>
            </div>
            <div class="grid gap-1 rounded-xl border border-border bg-background p-3">
                <span class="text-xs font-semibold text-muted-foreground">"Ø Geschwindigkeit"</span>
                <span class="font-semibold text-card-foreground">"50" <span class="text-sm font-medium text-muted-foreground">" km/h"</span></span>
            </div>
            <div class="grid gap-1 rounded-xl border border-border bg-background p-3">
                <span class="text-xs font-semibold text-muted-foreground">"Ø Schlagzahl"</span>
                <span class="font-semibold text-card-foreground">"30" <span class="text-sm font-medium text-muted-foreground">" bpm"</span></span>
            </div>
        </section>

        <section class="my-4 mx-2 grid gap-3 rounded-2xl border border-destructive/40 bg-card p-4">
            <div class="grid gap-1">
                <h3 class="text-sm font-semibold text-card-foreground">"Eintrag verwalten"</h3>
                <p class="text-sm text-muted-foreground">"Diesen Fahrtenbucheintrag dauerhaft löschen."</p>
            </div>
            link(
                href: href!(fahrtenbuch_page),
                attrs: attributes!(
                    class="justify-self-start rounded-md bg-destructive px-3 py-2 text-sm font-semibold text-destructive-foreground transition-opacity hover:opacity-90"
                    @click=$(async |_event: Event| {
                        delete_fahrtenbuch_entry(post_id).await;
                    })
                )
                "Eintrag löschen"
            )
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