pub mod global;
mod frontend;
mod backend;

use tokio::{runtime::LocalOptions, task};
use topcoat::router::tower::TowerRoute;
use topcoat::router::{Compression, StripPrefixLayer};
use topcoat::{asset::{AssetBundle, RouterBuilderAssetExt}, cookie::RouterBuilderCookieExt, router::{Router, RouterBuilderDiscoverExt}, runtime::{PrefetchMode, RouterBuilderRuntimeExt}};
use tower_http::compression::{predicate::SizeAbove, CompressionLayer};
use tower_http::services::ServeDir;
use tower::ServiceBuilder;

use crate::backend::fahrt;
use crate::backend::fahrtenbuch::FAHRTENBUCH;
use crate::backend::gps_interface::GpsPosition;
use crate::backend::plugins::PLUGINS;
use crate::{backend::{camera_interface::CAMERA_INTERFACE}, global::Global}; 

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
    // Initial plugin load
    PLUGINS.modify(|plugins| plugins.load() );

    FAHRTENBUCH.modify(|fb| {
        // add fake entries 
        let fahrt = fahrt::Fahrt::new("Now".to_string()).finish();
        fb.add_entry(fahrt);

        let mut fahrt2 = fahrt::Fahrt::new("Later".to_string()).finish();
        fahrt2.position_history.push(GpsPosition { latitude: 49.0, longitude: 8.0 });
        fahrt2.position_history.push(GpsPosition { latitude: 50.0, longitude: 9.0 });
        fahrt2.position_history.push(GpsPosition { latitude: 51.0, longitude: 10.0 });
        fb.add_entry(fahrt2);
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
                            .level(topcoat::router::CompressionLevel::Balanced)
                            .min_size(1024)
                    )
                    .runtime()
                    .prefetch(PrefetchMode::Never)
                    // Tower for precompressed static files
                    .layer(StripPrefixLayer::new("/static"))
                    .route(TowerRoute::any("/static/{*file}", ServeDir::new("static/public").precompressed_gzip().precompressed_br()))
                    // Tower for serving plugin files
                    .layer(StripPrefixLayer::new("/plugins"))
                    .route(TowerRoute::any("/plugins/{*file}",
                        ServiceBuilder::new()
                            .layer(CompressionLayer::new()
                                .gzip(true) 
                                .br(false)
                                .quality(tower_http::CompressionLevel::Default)
                                .compress_when(SizeAbove::new(1024))
                            )
                            .service(ServeDir::new("plugins")),
                    ))
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