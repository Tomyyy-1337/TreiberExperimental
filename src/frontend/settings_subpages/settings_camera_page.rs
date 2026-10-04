use topcoat::{Result, context::Cx, router::{Slot, page}, runtime::{Event, Signal, procedure, shard, signal}, view::{View, component, view}};
use crate::{backend::camera_interface::CAMERA_INTERFACE, frontend::layouts::back_button_layout::back_button_layout};

#[page("/settings/camera")]
pub async fn settings_camera() -> Result<impl View> {
    Ok(view! { back_button_layout(slot: Slot::new(view! {
        <h2> "Kamera Einstellungen" </h2>

        settings_camera_wrapper()
    }))})
}

#[component]
pub async fn settings_camera_wrapper(cx: &Cx) -> Result<impl View> {
    let hdr_enabled: Signal<bool> = signal(cx, || CAMERA_INTERFACE.hdr_enabled);
    Ok(view! {
        hdr_settings(hdr_enabled: hdr_enabled.clone())
        exposure_settings(hdr_enabled: hdr_enabled)
    })
}

#[component]
pub async fn exposure_settings(cx: &Cx, hdr_enabled: Signal<bool>) -> Result<impl View> {
    let exposure_signal: Signal<String> = signal(cx, || format!("{:.1}", CAMERA_INTERFACE.exposure_compenstion));

    Ok(view! {
        <h3> "Exposure Compensation" </h3>
        
        <div :hidden=$(hdr_enabled.get())> 
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
        </div>
        <div :hidden=$(!hdr_enabled.get())> 
            <p> "HDR is enabled, exposure compensation is not available." </p>
        </div>
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
pub async fn hdr_settings(hdr_enabled: Signal<bool>) -> Result<impl View> {
    Ok(view! {
        <h3> "HDR" </h3>
        <p> "HDR is " $(if hdr_enabled.get() { "enabled" } else { "disabled" }) </p>
        <button 
            @click=$(async |_event| { 
                let new_value = toggle_hdr().await; 
                hdr_enabled.set(new_value);
            })>
            $(if hdr_enabled.get() { "Disable HDR" } else { "Enable HDR" })
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