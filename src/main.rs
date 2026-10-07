mod global;
mod frontend;
mod backend;

use std::time::Duration;

use tokio::{runtime::LocalOptions, task};
use topcoat::{asset::{AssetBundle, RouterBuilderAssetExt}, cookie::RouterBuilderCookieExt, router::{Compression, CompressionLevel, Router, RouterBuilderDiscoverExt}, runtime::{PrefetchMode, RouterBuilderRuntimeExt}};

use crate::{backend::{camera_interface::CAMERA_INTERFACE, fahrtenbuch::{FAHRTENBUCH, Fahrt}}, global::Global}; 

struct BatteryState {
    pub battery_percentage: u8,
}

static BATTERY_STATE: Global<BatteryState> = Global::new(
    BatteryState {
        battery_percentage: 0,
    }
);

struct GpsState {
    pub satellite_count: u8,
}

static GPS_STATE: Global<GpsState> = Global::new(
    GpsState {
        satellite_count: 0,
    }
);

fn main() {
    // Add fake Fahrtenbuch entries
    FAHRTENBUCH.modify(|state| {
        for i in 0..20 {
            state.add_entry(
                Fahrt {
                    start_time: format!("2024-06-{}", i + 1),
                    dauer: Duration::from_mins(i),
                    strecke_km: (i * 10) as f64,
                    schläge: (i * 5) as u32,
                }
            );
        }
    });

    tokio::runtime::Builder::new_current_thread()
        .max_blocking_threads(4)
        .enable_all()
        .build_local(LocalOptions::default())
        .unwrap()
        .block_on(async {
            // Sent initial Camera configuration to the camera
            tokio::task::spawn_local(CAMERA_INTERFACE.send_to_camera());
            
            // Spawn backend Tasks
            task::spawn_local(increment_counter());

            // Start the web server
            topcoat::start(
                Router::builder()
                    .discover()
                    .cookies()
                    .assets(AssetBundle::load().unwrap())
                    .compression(Compression::new().brotli(false).level(CompressionLevel::Balanced))
                    .runtime()
                    .prefetch(PrefetchMode::Never)
                    // .origin_policy(OriginPolicy::dangerous_disable())
                    .build()
            )
            .await
            .unwrap();
        });
}

async fn increment_counter() {
    loop {
        BATTERY_STATE.modify(|state| {
            state.battery_percentage = (state.battery_percentage + 1) % 101;
        });
        GPS_STATE.modify(|state| {
            state.satellite_count = (state.satellite_count + 1) % 13;
        });
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }
}