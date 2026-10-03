use topcoat::{Result, context::Cx, router::{Slot, page}, runtime::{Event, procedure, signal, Signal}, view::{View, component, view}};

use crate::{backend::camera_interface::CAMERA_INTERFACE, frontend::layouts::back_button_layout::back_button_layout};

#[page("/settings/camera")]
pub async fn settings_camera() -> Result<impl View> {
    Ok(view! { back_button_layout(slot: Slot::new(view! {
        <h2> "Kamera Einstellungen" </h2>

        hdr_settings()
        exposure_settings()
    }))})
}

#[component]
pub async fn exposure_settings(cx: &Cx) -> Result<impl View> {
    let exposure_signal: Signal<String> = signal(cx, || CAMERA_INTERFACE.exposure_compenstion.to_string());

    Ok(view! {
        <h3> "Exposure Compensation" </h3>
        <p> "Current exposure compensation: " $(exposure_signal.get()) </p>
        <input
            type="range"
            min=(-3.0)
            max=(3.0)
            step=(0.5)  
            value=(CAMERA_INTERFACE.exposure_compenstion)
            @input=$(async |event: Event| {
                exposure_signal.set(event.target.value);
            })
            @change=$(async |event: Event| {
                set_exposure(event.target.value).await;
            })
        />
    })
}   

#[procedure]
pub async fn set_exposure(value: String) -> Result<()> {
    CAMERA_INTERFACE.modify(|s| s.exposure_compenstion = value.parse().unwrap_or(0.0));
    CAMERA_INTERFACE.send_to_camera();
    println!("Exposure compensation set to: {}", value);
    Ok(())
}

#[component]
pub async fn hdr_settings(cx: &Cx) -> Result<impl View> {
    let hdr_signal = signal(cx, || CAMERA_INTERFACE.hdr_enabled);

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
    let current = CAMERA_INTERFACE.hdr_enabled;
    CAMERA_INTERFACE.modify(|s| s.hdr_enabled = !current);
    CAMERA_INTERFACE.send_to_camera();
    println!("HDR toggled, new value: {}", !current);
    Ok(!current )
}