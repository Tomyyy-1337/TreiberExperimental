use std::str::FromStr;

use crate::{
    backend::camera_interface::{CameraInterface, FocusMode, Metering}, frontend::settings_subpages::settings_wrapper::settings_wrapper,
};
use tokio::sync::watch::{Receiver, Sender};
use topcoat::{
    Result, context::{Cx, app_context}, router::page, runtime::{Event, Signal, procedure, signal}, view::{Attributes, View, attributes, component, view},
};

#[page("/settings/camera")]
pub async fn settings_camera_page() -> Result<impl View> {
    Ok(view! {
        settings_wrapper(
            title: "Kamera Einstellungen",
            description: "Einstellungen für die Kamera",

            settings_camera_component()
        )
    })
}

#[component]
pub async fn settings_camera_component(cx: &Cx) -> Result<impl View> {
    let camera_interface = app_context::<Receiver<CameraInterface>>(cx);
    let hdr_enabled: Signal<bool> = signal(cx, || camera_interface.borrow().hdr_enabled);

    Ok(view! {
        <div class="grid gap-3 grid-cols-2">
            focal_length(camera_interface: camera_interface)
            metering_mode_settings(camera_interface: camera_interface)
            focus_mode_settings(camera_interface: camera_interface)
            bitrate_settings(camera_interface: camera_interface)
            hdr_settings(hdr_enabled: &hdr_enabled)
            autolevel_settings()
            <div class="col-span-2">
                exposure_settings(hdr_enabled: &hdr_enabled, camera_interface: camera_interface)
            </div>
        </div>
    })
}

#[component]
async fn select_widget<T: ToString + Send + Sync + PartialEq + 'static>(
    title: &str,
    possible_values: &[(T, &str)],
    selected: T,
    on_select: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <label class="grid min-w-0 gap-1 rounded-lg border border-border bg-card p-2.5 transition-colors has-focus-visible:border-ring">
            <span class="block text-[0.66rem] font-bold uppercase tracking-widest text-muted-foreground"> (title) </span>
            <select
                (on_select)
                name=(format!("{}_select", title.split_whitespace().map(|s| s.to_lowercase()).collect::<Vec<_>>().join("_")))
                class="select-arrow h-10 w-full cursor-pointer rounded-md border border-border bg-card px-2.5 pr-8 text-sm text-card-foreground outline-none transition-colors focus:border-ring focus:ring-2 focus:ring-ring/20"
            >
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
async fn focal_length(camera_interface: &Receiver<CameraInterface>) -> Result<impl View> {
    const OPTIONS: [(u8, &str); 3] = [(28, "28 mm"), (35, "35 mm"), (42, "42 mm")];

    let on_select = attributes! {
        @change=$(async |event: Event| {
            set_focal_length(event.target.value).await;
        })
    };

    Ok(view! {
        select_widget(
            title: "Brennweite",
            possible_values: &OPTIONS,
            selected: camera_interface.borrow().focal_length,
            on_select: on_select,
        )
    })
}

#[procedure("/api/set_focal_length")]
async fn set_focal_length(cx: &Cx, value: String) -> Result<()> {
    let camera_interface = app_context::<Sender<CameraInterface>>(cx);
    let value = value.parse::<u8>().unwrap_or(0);
    camera_interface.send_modify(|s| s.focal_length = value);
    println!("Focal length set to: {}", camera_interface.borrow().focal_length);
    Ok(())
}

#[component]
async fn metering_mode_settings(camera_interface: &Receiver<CameraInterface>) -> Result<impl View> {
    const OPTIONS: [(Metering, &str); 2] =
        [(Metering::Average, "Average"), (Metering::Center, "Center")];

    let on_select = attributes! {
        @change=$(async |event: Event| {
            set_metering_mode(event.target.value).await;
        })
    };

    Ok(view! {
        select_widget(
            title: "Belichtungsmessung",
            possible_values: &OPTIONS,
            selected: camera_interface.borrow().metering_mode,
            on_select: on_select,
        )
    })
}

#[procedure("/api/set_metering_mode")]
async fn set_metering_mode(cx: &Cx, value: String) -> Result<()> {
    let camera_interface = app_context::<Sender<CameraInterface>>(cx);
    let metering_mode = Metering::from_str(&value).unwrap_or(Metering::Average);
    camera_interface.send_modify(|s| s.metering_mode = metering_mode);
    println!(
        "Metering mode set to: {}",
        camera_interface.borrow().metering_mode.to_string()
    );
    Ok(())
}

#[component]
async fn focus_mode_settings(camera_interface: &Receiver<CameraInterface>) -> Result<impl View> {
    const OPTIONS: [(FocusMode, &str); 2] =
        [(FocusMode::Auto, "Auto"), (FocusMode::Fixed, "Manual")];

    let on_select = attributes! {
        @change=$(async |event: Event| {
            set_focus_mode(event.target.value).await;
        })
    };

    Ok(view! {
        select_widget(
            title: "Fokusmodus",
            possible_values: &OPTIONS,
            selected: camera_interface.borrow().focus_mode,
            on_select: on_select,
        )
    })
}

#[procedure("/api/set_focus_mode")]
async fn set_focus_mode(cx: &Cx, value: String) -> Result<()> {
    let camera_interface = app_context::<Sender<CameraInterface>>(cx);
    let focus_mode = FocusMode::from_str(&value).unwrap_or(FocusMode::Fixed);
    camera_interface.send_modify(|s| s.focus_mode = focus_mode);
    println!(
        "Focus mode set to: {}",
        camera_interface.borrow().focus_mode.to_string()
    );
    Ok(())
}

#[component]
async fn bitrate_settings(camera_interface: &Receiver<CameraInterface>) -> Result<impl View> {
    const OPTIONS: [(u32, &str); 5] = [
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
            possible_values: &OPTIONS,
            selected: camera_interface.borrow().bitrate,
            on_select: on_select,
        )
    })
}

#[procedure("/api/set_bitrate")]
async fn set_bitrate(cx: &Cx, value: String) -> Result<()> {
    let camera_interface = app_context::<Sender<CameraInterface>>(cx);
    let bitrate = value.parse::<u32>().unwrap_or(3200000);
    camera_interface.send_modify(|s| s.bitrate = bitrate);
    println!("Bitrate set to: {}", camera_interface.borrow().bitrate);
    Ok(())
}

#[component]
async fn hdr_settings(hdr_enabled: &Signal<bool>) -> Result<impl View> {
    Ok(view! {
        <label class="flex min-h-18 cursor-pointer items-center justify-between gap-3 rounded-lg border border-border bg-card px-2.5 py-2.5 transition-colors">
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
            <span class="relative h-5 w-9 shrink-0 rounded-full bg-muted-foreground transition-colors after:absolute after:left-1 after:top-1 after:h-3 after:w-3 after:rounded-full after:bg-background after:transition-transform peer-checked:bg-primary peer-checked:after:translate-x-4 peer-focus-visible:outline peer-focus-visible:outline-ring" aria-hidden="true"></span>
        </label>
    })
}

#[procedure("/api/toggle_hdr")]
async fn toggle_hdr(cx: &Cx) -> Result<bool> {
    let camera_interface = app_context::<Sender<CameraInterface>>(cx);

    camera_interface.send_modify(|camera_interface| {
        camera_interface.hdr_enabled = !camera_interface.hdr_enabled
    });
    println!("HDR set to: {}", camera_interface.borrow().hdr_enabled);
    Ok(camera_interface.borrow().hdr_enabled)
}

#[component]
async fn autolevel_settings(cx: &Cx) -> Result<impl View> {
    let auto_level_enabled: Signal<bool> = signal(cx, || false);

    Ok(view! {
        <label class="flex min-h-18 cursor-pointer items-center justify-between gap-3 rounded-lg border border-border bg-card px-2.5 py-2.5 transition-colors">
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
            <span class="relative h-5 w-9 shrink-0 rounded-full bg-muted-foreground transition-colors after:absolute after:left-1 after:top-1 after:h-3 after:w-3 after:rounded-full after:bg-background after:transition-transform peer-checked:bg-primary peer-checked:after:translate-x-4 peer-focus-visible:outline peer-focus-visible:outline-ring" aria-hidden="true"></span>
        </label>
    })
}

#[component]
async fn exposure_settings(cx: &Cx, hdr_enabled: &Signal<bool>, camera_interface: &Receiver<CameraInterface>) -> Result<impl View> {
    let exposure_signal: Signal<String> = signal(cx, || {
        format!("{:.1}", camera_interface.borrow().exposure_compenstion)
    });

    const EXPOSURE_LABELS: [&str; 13] = [
        "-3", "", "-2", "", "-1", "", "0", "", "+1", "", "+2", "", "+3",
    ];

    Ok(view! {
        <div :hidden=$(hdr_enabled.get())>
            <section class="my-0 grid h-24 content-start gap-2 rounded-lg border border-border bg-card px-3 py-2">
                <div class="flex items-center justify-between">
                    <span class="text-[0.66rem] font-bold uppercase tracking-widest text-muted-foreground">"Belichtung"</span>
                    <output for="camera-exposure-compensation" class="text-sm font-semibold tabular-nums text-card-foreground">
                        $(if exposure_signal.read().starts_with("-") {""} else {"+"})
                        format_float(signal: &exposure_signal, digits: 1)
                        " EV"
                    </output>
                </div>
                <div class="grid gap-0.5">
                    <input
                        name="camera-exposure-compensation"
                        class="mx-2 h-6 w-[calc(100%-1rem)] cursor-pointer accent-primary"
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
                    <div class="grid grid-cols-13 text-[0.7rem] tabular-nums text-muted-foreground" aria-hidden="true">
                        for (index, label) in EXPOSURE_LABELS.iter().enumerate() {
                            if index % 2 == 0 {
                                <span class="flex flex-col items-center"><i class="h-2.5 w-px bg-muted-foreground"></i><b class="font-normal">(label)</b></span>
                            } else {
                                <span class="flex justify-center"><i class="h-2 w-px bg-border"></i></span>
                            }
                        }
                    </div>
                </div>
            </section>
        </div>

        <div :hidden=$(!hdr_enabled.get())>
            <p class="my-0 flex h-24 items-center justify-center rounded-lg border border-dashed border-border bg-card px-3 py-2 text-center text-xs text-muted-foreground">"Belichtung ist bei HDR nicht verfügbar."</p>
        </div>
    })
}

#[procedure("/api/set_exposure")]
pub async fn set_exposure(cx: &Cx, value: String) -> Result<()> {
    let camera_interface = app_context::<Sender<CameraInterface>>(cx);
    let value = value.parse::<f32>().unwrap_or(0.0);
    camera_interface.send_modify(|s| s.exposure_compenstion = value);
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
