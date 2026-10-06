use topcoat::{Result, router::href, runtime::link, view::{Child, View, attributes, component, view}};

use crate::frontend::pages::settings_page::settings_page;

#[component]
pub async fn settings_wrapper(
    title: &str,
    #[default] description: &str,
    child: Child<'_>
) -> Result<impl View> {
    Ok(view! {
        <div class="mx-auto grid w-full max-w-4xl gap-6 px-4 py-6">
            <div class="flex items-start justify-between gap-4">
                <header class="grid gap-1">
                    <h1 class="text-2xl font-semibold tracking-tight text-foreground"> (title) </h1>
                    if description != "" {
                        <p class="max-w-2xl text-sm text-muted-foreground"> (description) </p>
                    }
                </header>
                link(
                    href: href!(settings_page),
                    attrs: attributes!{
                        class= "inline-flex h-12 w-12 shrink-0 items-center justify-center rounded-full border border-border bg-card text-3xl font-medium leading-none text-muted-foreground"
                        aria-label= "Zurück zu den Einstellungen"
                        title= "Zurück"
                    }
                    "×"
                )
            </div>
            <div class="border-t border-border" aria-hidden="true"></div>
            <div class="grid gap-4">
                (child)
            </div>
        </div>
    })
}

#[component]
pub async fn settings_container(
    title: &str,
    #[default] description: &str,
    child: Child<'_>
) -> Result<impl View> {
    Ok(view! {
        <article class="grid gap-3 rounded-xl border border-border bg-card p-4 text-card-foreground shadow-sm">
            <div class="grid gap-1">
                <h3 class="text-base font-semibold"> (title) </h3>
                if description != "" {
                    <p class="text-sm text-muted-foreground"> (description) </p>
                }
            </div>
            <div class="border-t border-border" aria-hidden="true"></div>
            <div class="grid gap-1.5">
                (child)
            </div>
        </article>
    })
}