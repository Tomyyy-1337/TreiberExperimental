use topcoat::{Result, context::Cx, router::{Slot, page}, runtime::{Event, Signal, procedure, shard, signal}, view::{View, attributes, component, view}};
use crate::{backend::camera_interface::{CAMERA_INTERFACE, Metering}, frontend::layouts::back_button_layout::back_button_layout};

#[page("/settings/camera")]
pub async fn settings_camera() -> Result<impl View> {
    Ok(view! {back_button_layout(slot: Slot::new(
        view! {
            <h2>"Kamera Einstellungen"</h2>

            settings_camera_wrapper()
        },
    ))})
}

#[component]
pub async fn settings_camera_wrapper(cx: &Cx) -> Result<impl View> {
    let hdr_enabled: Signal<bool> = signal(cx, || CAMERA_INTERFACE.hdr_enabled);
    Ok(view! {
        hdr_settings(hdr_enabled: hdr_enabled.clone())
        exposure_settings(hdr_enabled: hdr_enabled)
        metering_mode_settings()
    })
}

#[component]
pub async fn exposure_settings(cx: &Cx, hdr_enabled: Signal<bool>) -> Result<impl View> {
    let exposure_signal: Signal<String> = signal(cx, || format!("{:.1}", CAMERA_INTERFACE.exposure_compenstion));
    
    let input_attributes = attributes! {
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
    };
    
    Ok(view! {
        <h3>"Exposure Compensation"</h3>

        <div :hidden=$(hdr_enabled.get())>
            <p>
                "Current exposure compensation: "
                format_float(signal: exposure_signal, digits: 1)
            </p>
            <input (input_attributes) />
        </div>

        <div :hidden=$(!hdr_enabled.get())>
            <p>"HDR is enabled, exposure compensation is not available."</p>
        </div>
    })
}

#[component]
pub async fn format_float(signal: Signal<String>, digits: usize) -> Result<impl View> {
    Ok(view! {
        $(raw! {
            "Number(${signal}.get()).toFixed(${digits})",
            signal.get()
        })
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
    let toggle_hdr_attributes = attributes! {
        @click=$(async |_event: Event| {
            let new_value = toggle_hdr().await;
            hdr_enabled.set(new_value);
        })
    };

    Ok(view! {
        <h3>"HDR"</h3>
        <p>
            "HDR is "
            $(if hdr_enabled.get() { "enabled" } else { "disabled" })
        </p>
        <button (toggle_hdr_attributes)>
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

#[component]
pub async fn metering_mode_settings(cx: &Cx) -> Result<impl View> {
    let metering_mode_signal: Signal<String> = signal(cx, || CAMERA_INTERFACE.metering_mode.to_name().to_owned());

    let options = [
        (Metering::Average, "Average"),
        (Metering::Center, "Center"),
    ];

    let select_atributes = attributes! {
        @change=$(async |event: Event| {
            metering_mode_signal.set(event.target.value.clone());
            set_metering_mode(event.target.value).await;
        })
    };

    Ok(view! {
        <h3>"Metering Mode"</h3>
        <p>
            "Current metering mode: "
            $(metering_mode_signal.get())
            " "
        </p>

        <select (select_atributes) id="metering_mode_select">
            for (mode, label) in options.iter() {
                let attributes = attributes! {
                    value=(mode.to_name())
                    selected=(metering_mode_signal.get_untracked() == mode.to_name())
                };
                <option (attributes)>(label)</option>
            }
        </select>
    })
}

#[procedure]
pub async fn set_metering_mode(value: String) -> Result<()> {
    let metering_mode = Metering::from_name(&value);
    CAMERA_INTERFACE.modify_and_send(|s| s.metering_mode = metering_mode);
    println!("Metering mode set to: {}", CAMERA_INTERFACE.metering_mode.to_name());
    Ok(())
}

