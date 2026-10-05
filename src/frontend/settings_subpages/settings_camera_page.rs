use std::str::FromStr;

use topcoat::{Result, context::Cx, router::{Slot, page}, runtime::{Event, Signal, procedure, signal}, view::{Attributes, View, attributes, component, view}};
use crate::{backend::camera_interface::{CAMERA_INTERFACE, FocusMode, Metering}, frontend::layouts::back_button_layout::back_button_layout};

#[page("/settings/camera")]
pub async fn settings_camera_page() -> Result<impl View> {
    Ok(view! {back_button_layout(slot: Slot::new(
        view! {
            <h2 class="mb-4 text-xl font-semibold tracking-tight">"Kamera Einstellungen"</h2>

            // settings_camera_wrapper()

            settings_camera_component()
        },
    ))})
}

#[component]
pub async fn settings_camera_component(cx: &Cx) -> Result<impl View> {
    let hdr_enabled: Signal<bool> = signal(cx, || CAMERA_INTERFACE.hdr_enabled);

    Ok(view! {
        <div class="grid gap-2 rounded-xl border border-border bg-card p-3 text-card-foreground shadow-sm">
            <div class="grid grid-cols-2 gap-2">
                focal_length()
                metering_mode_settings()
                focus_mode_settings()
                bitrate_settings()
                hdr_settings(hdr_enabled: &hdr_enabled)
                autolevel_settings()
            </div>
            exposure_settings(hdr_enabled: &hdr_enabled)
        </div>
    })
}

#[component]
async fn select_widget<T: ToString + Send + Sync + PartialEq + 'static>(
    title: &str,
    possible_values: &[(T, &str)],
    selected: T,
    on_select: Attributes
) -> Result<impl View> {
    Ok(view! {
        <label class="min-w-0 rounded-lg border border-border bg-background p-1.5">
            <span class="mb-1 block text-[0.66rem] font-bold uppercase tracking-[0.1em] text-muted-foreground"> (title) </span>
            <select (on_select) class="mt-0.5 h-9 w-full cursor-pointer rounded-md border border-border bg-card px-2 text-sm text-card-foreground outline-none">
                for (value, label) in possible_values.iter() {
                    let attributes = attributes! {
                        value=(value.to_string())
                        selected=(selected == *value)
                    };
                    <option (attributes)>(label)</option>
                }
            </select>
        </label>
    })
}

#[component]
async fn focal_length() -> Result<impl View> {
    let option: [(u8, &str); 3] = [
        (28, "28 mm"),
        (35, "35 mm"),
        (42, "42 mm"),
    ];

    let on_select = attributes! {
        @change=$(async |event: Event| {
            set_focal_length(event.target.value).await;
        })
    };

    Ok(view! {
        select_widget(
            title: "Brennweite",
            possible_values: &option,
            selected: CAMERA_INTERFACE.focal_length,
            on_select: on_select,
        )   
    })
}

#[procedure("/api/set_focal_length")]
async fn set_focal_length(value: String) -> Result<()> {
    let value = value.parse::<u8>().unwrap_or(0);
    CAMERA_INTERFACE.modify_and_send(|s| s.focal_length = value);
    println!("Focal length set to: {}", CAMERA_INTERFACE.focal_length);
    Ok(())
}


#[component]
async fn metering_mode_settings() -> Result<impl View> {
    let options = [
        (Metering::Average, "Average"),
        (Metering::Center, "Center"),
    ];

    let on_select = attributes! {
        @change=$(async |event: Event| {
            set_metering_mode(event.target.value).await;
        })
    };

    Ok(view! {
        select_widget(
            title: "Belichtungsmessung",
            possible_values: &options,
            selected: CAMERA_INTERFACE.metering_mode,
            on_select: on_select,
        )
    })
}

#[procedure("/api/set_metering_mode")]
async fn set_metering_mode(value: String) -> Result<()> {
    let metering_mode = Metering::from_str(&value).unwrap_or(Metering::Average);
    CAMERA_INTERFACE.modify_and_send(|s| s.metering_mode = metering_mode);
    println!("Metering mode set to: {}", CAMERA_INTERFACE.metering_mode.to_string());
    Ok(())
}

#[component]
async fn focus_mode_settings() -> Result<impl View> {
    let options = [
        (FocusMode::Auto, "Auto"),
        (FocusMode::Fixed, "Manual"),
    ];

    let on_select = attributes! {
        @change=$(async |event: Event| {
            set_focus_mode(event.target.value).await;
        })
    };

    Ok(view! {
        select_widget(
            title: "Fokusmodus",
            possible_values: &options,
            selected: CAMERA_INTERFACE.focus_mode,
            on_select: on_select,
        )
    })
}

#[procedure("/api/set_focus_mode")]
async fn set_focus_mode(value: String) -> Result<()> {
    let focus_mode = FocusMode::from_str(&value).unwrap_or(FocusMode::Fixed);
    CAMERA_INTERFACE.modify_and_send(|s| s.focus_mode = focus_mode);
    println!("Focus mode set to: {}", CAMERA_INTERFACE.focus_mode.to_string());
    Ok(())
}


#[component]
async fn bitrate_settings() -> Result<impl View> {
    let options = [
        (1600000, "1.6 MB/s"),
        (2400000, "2.4 MB/s"),
        (3200000, "3.2 MB/s"),
        (4000000, "4.0 MB/s"),
        (8000000, "8.0 MB/s"),
    ];

    let on_select = attributes! {
        @change=$(async |event: Event| {
            set_bitrate(event.target.value).await;
        })
    };

    Ok(view! {
        select_widget(
            title: "Bitrate",
            possible_values: &options,
            selected: CAMERA_INTERFACE.bitrate,
            on_select: on_select,
        )
    })
}

#[procedure("/api/set_bitrate")]
async fn set_bitrate(value: String) -> Result<()> {
    let bitrate = value.parse::<u32>().unwrap_or(3200000);
    CAMERA_INTERFACE.modify_and_send(|s| s.bitrate = bitrate);
    println!("Bitrate set to: {}", CAMERA_INTERFACE.bitrate);
    Ok(())
}

#[component]
async fn hdr_settings(cx: &Cx, hdr_enabled: &Signal<bool>) -> Result<impl View> {
    Ok(view! {
        <label class="flex min-h-12 cursor-pointer items-center justify-between gap-2 rounded-lg border border-border bg-background px-2 py-2">
            <span class="grid min-w-0 gap-1">
                <span class="text-sm font-medium">"HDR"</span>
                <small class="text-[0.68rem] leading-none text-muted-foreground">$(if hdr_enabled.get() {"Aktiv"} else {"Aus"})</small>
            </span>
            <input
                class="peer sr-only"
                name="camera-hdr"
                type="checkbox"
                checked=(hdr_enabled.get_untracked())
                @click=$(async |_event: Event| {
                    let new_hdr_enabled = toggle_hdr().await;
                    hdr_enabled.set(new_hdr_enabled);
                })
            />
            <span class="relative h-5 w-9 shrink-0 rounded-full bg-muted-foreground transition-colors after:absolute after:left-1 after:top-1 after:h-3 after:w-3 after:rounded-full after:bg-background after:transition-transform peer-checked:bg-primary peer-checked:after:translate-x-4 peer-focus-visible:outline peer-focus-visible:outline-2 peer-focus-visible:outline-ring" aria-hidden="true"></span>
        </label> 
    })
}

#[procedure("/api/toggle_hdr")]
async fn toggle_hdr() -> Result<bool> {
    CAMERA_INTERFACE.modify_and_send(|camera_interface| camera_interface.hdr_enabled = !camera_interface.hdr_enabled);
    println!("HDR set to: {}", CAMERA_INTERFACE.hdr_enabled);
    Ok(CAMERA_INTERFACE.hdr_enabled)
}

#[component]
async fn autolevel_settings(cx: &Cx) -> Result<impl View> {
    let auto_level_enabled: Signal<bool> = signal(cx, || false);

    Ok(view! {
        <label class="flex min-h-12 cursor-pointer items-center justify-between gap-2 rounded-lg border border-border bg-background px-2 py-2">
            <span class="grid min-w-0 gap-1">
                <span class="text-sm font-medium">"Auto Level"</span>
                <small class="text-[0.68rem] leading-none text-muted-foreground">$(if auto_level_enabled.get() {"Aktiv"} else {"Aus"})</small>
            </span>
            <input 
                class="peer sr-only"
                name="camera-auto-level"
                type="checkbox"
                checked=(auto_level_enabled.get_untracked())
                @click=$(async |_event: Event| {
                    auto_level_enabled.set(!auto_level_enabled.get());
                })
            />  
            <span class="relative h-5 w-9 shrink-0 rounded-full bg-muted-foreground transition-colors after:absolute after:left-1 after:top-1 after:h-3 after:w-3 after:rounded-full after:bg-background after:transition-transform peer-checked:bg-primary peer-checked:after:translate-x-4 peer-focus-visible:outline peer-focus-visible:outline-2 peer-focus-visible:outline-ring" aria-hidden="true"></span>
        </label>
    })
}

#[component]
async fn exposure_settings(cx: &Cx, hdr_enabled: &Signal<bool>) -> Result<impl View> {
    let exposure_signal: Signal<String> = signal(cx, || format!("{:.1}", CAMERA_INTERFACE.exposure_compenstion));

    Ok(view! {
        <div :hidden=$(hdr_enabled.get())>
            <section class="my-0 grid h-[4.9rem] content-start gap-2 rounded-lg border border-border bg-background p-2">
                <div class="flex items-center justify-between">
                    <span class="text-[0.66rem] font-bold uppercase tracking-[0.1em] text-muted-foreground">"Belichtung"</span>
                    <output for="camera-exposure-compensation">
                        $(if exposure_signal.read().starts_with("-") {""} else {"+"})
                        format_float(signal: &exposure_signal, digits: 1)
                        " EV"
                    </output>
                </div>
                <div class="relative">
                    <input
                        name="camera-exposure-compensation"
                        class="h-6 w-full cursor-pointer accent-primary"
                        type="range"
                        min="-3"
                        max="3"
                        step="0.5"
                        value=(exposure_signal.read_untracked())
                        @input=$(async |event: Event| {
                            exposure_signal.set(event.target.value);
                        })
                        @change=$(async |event: Event| {
                            set_exposure(event.target.value).await;
                        })
                    />
                </div>
            </section>
        </div>

        <div :hidden=$(!hdr_enabled.get())>
            <p class="my-0 flex min-h-[4.9rem] items-center rounded-lg border border-dashed border-border px-2 text-[0.6rem] tracking-[0.06em] text-muted-foreground">"Belichtung ist bei HDR nicht verfügbar."</p>
        </div>
    })
}

#[procedure("/api/set_exposure")]
pub async fn set_exposure(value: String) -> Result<()> {
    let value = value.parse::<f32>().unwrap_or(0.0);
    CAMERA_INTERFACE.modify_and_send(|s| s.exposure_compenstion = value);
    println!("Exposure compensation set to: {}", value);
    Ok(())
}

#[component]
pub async fn format_float(signal: &Signal<String>, digits: usize) -> Result<impl View> {
    Ok(view! {
        $(raw! {
            "Number(${signal}.get()).toFixed(${digits})",
            signal.read()
        })
    })
}