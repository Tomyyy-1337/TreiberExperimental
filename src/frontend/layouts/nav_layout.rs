use topcoat::{Result, context::Cx, router::href, runtime::link, view::{Child, View, attributes, class, component, view}};

use crate::frontend::{fahrtenbuch_page::fahrtenbuch_page, icons::battery_status::battery_status, settings_page::settings_page, stream_page::camera_page};

#[component]
pub async fn nav_layout(child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <div class="mx-auto w-full max-w-4xl px-2 py-2">
            <section class="my-4 grid grid-cols-3 items-center rounded-2xl border border-border bg-card p-4">
                <div class="justify-self-start">
                    battery_status()
                </div>
                <h1 class="whitespace-nowrap text-center text-xl font-semibold tracking-tight"> "Ruder Cam Beta" </h1>
                <div class="justify-self-end">
                    battery_status()
                </div>
            </section>
            
            <section class="my-4 rounded-2xl border border-border bg-card p-2">
                navbar()
            </section>

            (child)
        </div>
    })
}


#[component]
async fn navbar(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        let camera_link = href!(camera_page);
        let camera_active = camera_link.is_current(cx);
        let fahrtenbuch_link = href!(fahrtenbuch_page);
        let fahrtenbuch_active = fahrtenbuch_link.is_current(cx);
        let settings_link = href!(settings_page);
        let settings_active = settings_link.is_current(cx);

        <nav class="relative grid grid-cols-3 gap-0">            
            let attributes = attributes!(class=(class!(
                "z-10 flex h-11 min-w-[120px] items-center justify-center whitespace-nowrap rounded-xl px-4 py-2.5 text-center text-base font-semibold text-foreground no-underline transition-colors duration-300 ease-in-out",
                "text-primary-foreground" if camera_active
            )));
            link(href: camera_link, attrs: attributes, "Home")

            let attributes = attributes!(class=(class!(
                "z-10 flex h-11 min-w-[120px] items-center justify-center whitespace-nowrap rounded-xl px-4 py-2.5 text-center text-base font-semibold text-foreground no-underline transition-colors duration-300 ease-in-out",
                "text-primary-foreground" if fahrtenbuch_active
            )));
            link(href: fahrtenbuch_link, attrs: attributes, "Fahrtenbuch")

            let attributes = attributes!(class=(class!(
                "z-10 flex h-11 min-w-[120px] items-center justify-center whitespace-nowrap rounded-xl px-4 py-2.5 text-center text-base font-semibold text-foreground no-underline transition-colors duration-300 ease-in-out",
                "text-primary-foreground" if settings_active
            )));
            link(href: settings_link, attrs: attributes, "Settings")

            <div class=(class!(
                "pointer-events-none absolute inset-y-0 left-0 w-1/3 rounded-xl bg-primary transition-transform duration-300 ease-in-out",
                "translate-x-0" if camera_active,
                "translate-x-full" if fahrtenbuch_active,
                "translate-x-[200%]" if settings_active
            )) aria-hidden="true"></div>
        </nav>
    })
}