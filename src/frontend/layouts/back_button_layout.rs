use topcoat::{Result, router::{Slot, href, layout}, runtime::link, view::{View, attributes, view}};

use crate::frontend::settings_page::settings_page;

#[layout]
pub async fn back_button_layout(slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <div class="mx-auto w-full max-w-4xl px-2 py-4">
            link(
                href: href!(settings_page),
                attrs: attributes!{
                    class= "mb-4 inline-block text-primary underline-offset-4 hover:underline"
                }
                "Go Back"
            )
        (slot)
        </div>
    })
}