use topcoat::{Result, context::Cx, router::{Slot, page}, runtime::{procedure, signal}, view::{View, component, view}};

use crate::{SETTINGS, frontend::layout::back_button_layout};

#[page("/settings/camera")]
pub async fn settings_camera() -> Result<impl View> {
    Ok(view! { back_button_layout(slot: Slot::new(view! {
        <h2> "Settings Camera Page" </h2>

        hdr_settings()
    }))})
}

#[component]
pub async fn hdr_settings(cx: &Cx) -> Result<impl View> {
    let hdr_signal = signal(cx, || SETTINGS.hdr);

    Ok(view! {
        <h3> "HDR" </h3>
        <p> "HDR is " $(if hdr_signal.get() { "enabled" } else { "disabled" }) </p>
        <button 
            @click=$(async |_event| { 
                let new_value = toggle_hdr().await; 
                hdr_signal.set(new_value);
            })>
            $(if hdr_signal.get() { "Disable HDR" } else { "Enable HDR" })
        </button>
    })
}

#[procedure]
pub async fn toggle_hdr() -> Result<bool> {
    let current = SETTINGS.hdr;
    SETTINGS.modify(|s| s.hdr = !current);
    println!("HDR toggled, new value: {}", !current);
    Ok(!current)
}