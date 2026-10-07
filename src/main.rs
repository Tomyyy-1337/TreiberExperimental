mod global;
mod frontend;
mod backend;

use tokio::{runtime::LocalOptions, task};
use topcoat::{asset::{AssetBundle, RouterBuilderAssetExt}, cookie::RouterBuilderCookieExt, router::{Compression, CompressionLevel, Router, RouterBuilderDiscoverExt}, runtime::{PrefetchMode, RouterBuilderRuntimeExt, record}, view::component};

use crate::{backend::camera_interface::CAMERA_INTERFACE, global::Global}; 

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

struct FahrtenbuchState {
    pub next_id: u32,
    pub entries: Vec<FahrtenbuchEntry>,
}

impl FahrtenbuchState {
    pub fn add_entry(&mut self, gesamtstrecke: f64) {
        let entry = FahrtenbuchEntry {
            id: self.next_id,
            gesamtstrecke,
        };
        self.next_id += 1;
        self.entries.push(entry);
    }
}

#[record]
#[derive(Clone)]
struct FahrtenbuchEntry {
    id: u32,
    gesamtstrecke: f64,
}

static FAHRTENBUCH_STATE: Global<FahrtenbuchState> = Global::new(
    FahrtenbuchState {
        next_id: 0,
        entries: Vec::new(),
    }
);

fn main() {
    FAHRTENBUCH_STATE.modify(|state| {
        for i in 0..20 {
            state.add_entry((i * 10) as f64);
        }
    });


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