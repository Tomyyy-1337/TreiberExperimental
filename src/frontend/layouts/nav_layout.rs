use topcoat::{Result, router::{Slot, layout, request::uri}, view::{View, attributes, class, component, view}, context::Cx};

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
    let pages = vec![
        ("/", "Home"),
        ("/fahrtenbuch", "Fahrtenbuch"),
        ("/settings", "Settings"),
    ];

    Ok(view! {
        <nav class="navbar"> 
            for (path, label) in pages {
                let attributes = attributes!(
                    class=(class!(
                        "navbar-link",
                        "active" if uri(cx).path() == path 
                    ))
                    href=(path)
                );

                <a (attributes)> (label) </a>
            }
        </nav>
    })
}