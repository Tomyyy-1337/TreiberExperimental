use topcoat::{Result, context::Cx, router::href, runtime::link, view::{Child, View, attributes, class, component, view}};

use crate::frontend::{icons::{battery_status::battery_status, satelite_status::satelite_status}, pages::{fahrtenbuch_page::fahrtenbuch_page, settings_page::settings_page, stream_page::camera_page}};

#[component]
pub async fn nav_layout(child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <section class="grid grid-cols-3 items-center rounded-2xl border border-border bg-card p-4">
            <div class="justify-self-start">
                battery_status()
            </div>
            <h1 id="main-title" class="whitespace-nowrap text-center text-xl font-semibold tracking-tight"> "Ruder Cam Beta" </h1>
            <div class="justify-self-end">
                satelite_status()
            </div>
        </section>
        
        <section class="my-4 rounded-2xl border border-border bg-card p-2">
            navbar()
        </section>

        (child)
    })
}


#[component]
async fn navbar(cx: &Cx) -> Result<impl View> {
    const LINK_STYLING: &str = "z-10 flex h-11 min-w-[120px] items-center justify-center whitespace-nowrap rounded-xl px-4 text-center text-base font-semibold text-foreground no-underline transition-colors duration-300 ease-in-out";
    
    Ok(view! {
        let camera_link = href!(camera_page);
        let camera_active = camera_link.is_current(cx);
        let fahrtenbuch_link = href!(fahrtenbuch_page);
        let fahrtenbuch_active = fahrtenbuch_link.is_current(cx);
        let settings_link = href!(settings_page);
        let settings_active = settings_link.is_current(cx);

        <nav class="relative grid grid-cols-3 gap-0">            
            link(
                href: camera_link, 
                attrs: attributes!(class=(class!(LINK_STYLING, "text-primary-foreground" if camera_active))),
                "Home"
            )

            link(
                href: fahrtenbuch_link, 
                attrs: attributes!(class=(class!(LINK_STYLING, "text-primary-foreground" if fahrtenbuch_active))),
                "Fahrtenbuch"
            )

            link(
                href: settings_link, 
                attrs: attributes!(class=(class!(LINK_STYLING, "text-primary-foreground" if settings_active))),
                "Settings"
            )

            <div class=(class!(
                "pointer-events-none absolute inset-y-0 left-0 w-1/3 rounded-xl bg-primary transition-transform duration-300 ease-in-out",
                "translate-x-0" if camera_active,
                "translate-x-full" if fahrtenbuch_active,
                "translate-x-[200%]" if settings_active
            )) aria-hidden="true"></div>
        </nav>
    })
}