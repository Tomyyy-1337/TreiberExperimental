use topcoat::{Result, context::Cx, router::request::uri, view::{View, attributes, class, component, view}};

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