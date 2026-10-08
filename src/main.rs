mod global;
mod frontend;
mod backend;

use std::path::PathBuf;
use std::time::Duration;

use tokio::{io::{AsyncReadExt, AsyncSeekExt}, runtime::LocalOptions, task};
use topcoat::router::{RouterBuilderDirectoryExt, StripPrefixLayer, tower::{TowerLayer, TowerRoute}};
use topcoat::router::response::Response;
use topcoat::{asset::{AssetBundle, RouterBuilderAssetExt}, context::Cx, cookie::RouterBuilderCookieExt, router::{header::{ACCEPT_RANGES, CONTENT_LENGTH, CONTENT_RANGE, CONTENT_TYPE, RANGE}, Body, Compression, CompressionLevel, Method, RouteFn, RouteFuture, Router, RouterBuilderDiscoverExt, StatusCode}, runtime::{PrefetchMode, RouterBuilderRuntimeExt}};
use tower_http::services::ServeDir;

use crate::{backend::{camera_interface::CAMERA_INTERFACE, fahrtenbuch::{FAHRTENBUCH, Fahrt, GpsPosition}}, global::Global}; 

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
                    .serve_dir("/static/{*file}", "static/public")
                    // Tower for serving map files as range requests (topcoat does not support range requests in v0.10.0)
                    .layer(StripPrefixLayer::new("/maps"))
                    .route(TowerRoute::any("/maps/{*file}", ServeDir::new("maps")))
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