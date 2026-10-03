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
    let exposure_signal: Signal<String> = signal(cx, || format!("{:.1}", CAMERA_INTERFACE.exposure_compenstion));

    Ok(view! {
        <h3> "Exposure Compensation" </h3>
        <p> "Current exposure compensation: " $({
            let ev = exposure_signal.get();
            let ev_formated = raw!{
                "Number(${ev}).toFixed(1)",
                ev
            };
            ev_formated
        }) </p>
        <input
            type="range"
            min=(-3.0)
            max=(3.0)
            step=(0.5)  
            value=(exposure_signal.get_untracked())
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
    let value = value.parse::<f32>().unwrap_or(0.0);
    CAMERA_INTERFACE.modify_and_send(|s| s.exposure_compenstion = value);
    println!("Exposure compensation set to: {}", value);
    Ok(())
}

#[component]
pub async fn hdr_settings(cx: &Cx) -> Result<impl View> {
    let hdr_signal: Signal<bool> = signal(cx, || CAMERA_INTERFACE.hdr_enabled);

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
    CAMERA_INTERFACE.modify_and_send(|s| s.hdr_enabled = !current);
    println!("HDR toggled, new value: {}", !current);
    Ok(!current)
}