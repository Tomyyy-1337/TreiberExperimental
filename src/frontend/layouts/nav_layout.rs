use topcoat::{Result, context::Cx, router::{Slot, href, layout, request::uri}, runtime::{PrefetchMode, link}, view::{View, attributes, class, component, view}};

use crate::frontend::{fahrtenbuch_page::fahrtenbuch_page, settings_page::settings_page, stream_page::camera_page};

#[layout]
pub async fn nav_layout(slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <div class="container">
            <section>
                <h1> "Ruder Cam Beta" </h1>
            </section>
            
            <section>
                navbar()
            </section>

            (slot)
        </div>
    })
}


#[component]
pub async fn navbar(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <nav class="navbar"> 
            let camera_link = href!(camera_page);
            let attributes = attributes!(class=(class!(
                "navbar-link",
                "active" if camera_link.is_current(cx)
            )));
            link(href: camera_link, attrs: attributes, "Home")

            let fahrtenbuch_link = href!(fahrtenbuch_page);
            let attributes = attributes!(class=(class!(
                "navbar-link",
                "active" if fahrtenbuch_link.is_current(cx)
            )));
            link(href: fahrtenbuch_link, attrs: attributes, "Fahrtenbuch")

            let settings_link = href!(settings_page);
            let attributes = attributes!(class=(class!(
                "navbar-link",
                "active" if settings_link.is_current(cx)
            )));
            link(href: settings_link, attrs: attributes, "Settings")
        </nav>
    })
}