pub mod global;
mod frontend;
mod backend;

use std::time::Duration;

use tokio::{runtime::LocalOptions, task};
use topcoat::router::tower::TowerRoute;
use topcoat::router::{StripPrefixLayer};
use topcoat::{asset::{AssetBundle, RouterBuilderAssetExt}, cookie::RouterBuilderCookieExt, router::{Compression, CompressionLevel, Router, RouterBuilderDiscoverExt}, runtime::{PrefetchMode, RouterBuilderRuntimeExt}};
use tower_http::services::ServeDir;

use crate::{backend::{camera_interface::CAMERA_INTERFACE, fahrtenbuch::{FAHRTENBUCH, FinishedFahrt, GpsPosition}}, global::Global}; 

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

    FAHRTENBUCH.modify(|state| {
        for i in 0..20 {
            state.add_entry(
                FinishedFahrt {
                    start_time: format!("2024-06-{}", i + 1),
                    dauer: Duration::from_mins(i),
                    strecke_km: (i * 10) as f64,
                    schläge: (i * 5) as u32,
                    position_history: vec![
                        GpsPosition {
                            latitude: 49.4400657,
                            longitude: 7.7491265 + i as f64 / 50.0
                        },
                        GpsPosition {
                            latitude: 49.4401657,
                            longitude: 7.7411265 + i as f64 / 50.0
                        },
                        GpsPosition {
                            latitude: 49.4402657,
                            longitude: 7.7431265 + i as f64 / 50.0
                        },
                        GpsPosition {
                            latitude: 49.4403657,
                            longitude: 7.7451265 + i as f64 / 50.0
                        },
                    ]
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
                    .compression(
                        Compression::new()
                            .brotli(false)
                            .level(CompressionLevel::Balanced)
                            .min_size(1024)
                    )
                    .runtime()
                    .prefetch(PrefetchMode::Never)
                    // Tower for precompressed static files
                    .layer(StripPrefixLayer::new("/static"))
                    .route(TowerRoute::any("/static/{*file}", ServeDir::new("static/public").precompressed_gzip().precompressed_br()))
                    // Tower for serving plugin files
                    .layer(StripPrefixLayer::new("/plugins"))
                    .route(TowerRoute::any("/plugins/{*file}", ServeDir::new("plugins")))
                    // Tower for serving map files as range requests (topcoat does not support range requests in v0.10.0)
                    .layer(StripPrefixLayer::new("/maps"))
                    .route(TowerRoute::any("/maps/{*file}", ServeDir::new("maps")))
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
        tokio::time::sleep(std::time::Duration::from_millis(700)).await;
    }
}