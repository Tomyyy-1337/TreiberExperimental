mod global;
mod frontend;
mod backend;

use tokio::{runtime::LocalOptions, task};
use topcoat::{asset::{AssetBundle, RouterBuilderAssetExt}, cookie::RouterBuilderCookieExt, router::{Compression, CompressionLevel, Router, RouterBuilderDiscoverExt}, runtime::{PrefetchMode, RouterBuilderRuntimeExt}};

use crate::{backend::camera_interface::CAMERA_INTERFACE, global::Global}; 

struct BatteryState {
    battery_percentage: u8,
}

static BATTERY_STATE: Global<BatteryState> = Global::new(
    BatteryState {
        battery_percentage: 0,
    }
);

struct GpsState {
    satellite_count: u8,
}

static GPS_STATE: Global<GpsState> = Global::new(
    GpsState {
        satellite_count: 0,
    }
);

fn main() {
    tokio::runtime::Builder::new_current_thread()
        .max_blocking_threads(4)
        .enable_all()
        .build_local(LocalOptions::default())
        .unwrap()
        .block_on(async {
            CAMERA_INTERFACE.send_to_camera();

            task::spawn_local(increment_counter());

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