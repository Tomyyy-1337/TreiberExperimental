pub mod global;
mod frontend;
mod backend;

use tokio::sync::watch::{self, Receiver, Sender};
use tokio::time::MissedTickBehavior;
use tokio::{runtime::LocalOptions, task};
use topcoat::router::tower::TowerRoute;
use topcoat::router::{Compression, StripPrefixLayer};
use topcoat::{asset::{AssetBundle, RouterBuilderAssetExt}, cookie::RouterBuilderCookieExt, router::{Router, RouterBuilderDiscoverExt}, runtime::{PrefetchMode, RouterBuilderRuntimeExt}};
use tower_http::compression::{predicate::SizeAbove, CompressionLayer};
use tower_http::services::ServeDir;
use tower::ServiceBuilder;

use crate::backend::fahrt::{self, FahrtStatus};
use crate::backend::fahrtenbuch::FAHRTENBUCH;
use crate::backend::gps_interface::GpsPosition;
use crate::backend::plugins::PLUGINS;
use crate::{backend::{camera_interface::CAMERA_INTERFACE}}; 

#[derive(Copy, Clone)]
struct BatteryState {
    pub battery_percentage: u8,
}

struct GpsSatelites {
    pub count: u8,
}

struct GpsState {
    pub speed: f32,
    pub latitude: f64,
    pub longitude: f64,
}

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
            let (battery_sender, battery_receiver) = watch::channel(BatteryState { battery_percentage: 0 });
            let (gps_satelits_sender, gps_satelits_receiver) = watch::channel(GpsSatelites { count: 0 });
            let (gps_state_sender, gps_state_receiver) = watch::channel(GpsState { speed: 0.0, latitude: 0.0, longitude: 0.0 });
            let (fahrt_sender, fahrt_receiver) = watch::channel(fahrt::FahrtStatus::Inactive);

            // Sent initial Camera configuration to the camera
            tokio::task::spawn_local(CAMERA_INTERFACE.send_to_camera());
            
            // Spawn backend Tasks
            task::spawn_local(update_battery_state(battery_sender));
            task::spawn_local(update_gps_state(gps_state_sender.clone(), gps_satelits_sender));
            task::spawn_local(update_fahrt_state(fahrt_sender.clone(), gps_state_receiver.clone(), gps_satelits_receiver.clone()));

            // Start the web server
            topcoat::start(
                Router::builder()
                    .app_context::<Receiver<BatteryState>>(battery_receiver)
                    .app_context::<Receiver<GpsSatelites>>(gps_satelits_receiver)
                    .app_context::<Receiver<GpsState>>(gps_state_receiver)
                    .app_context::<Receiver<fahrt::FahrtStatus>>(fahrt_receiver)
                    .app_context::<Sender<fahrt::FahrtStatus>>(fahrt_sender)
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

// Read battery module
async fn update_battery_state(battery_sender: Sender<BatteryState>) {
    loop {
        battery_sender.send_modify(|s| s.battery_percentage = (s.battery_percentage + 1) % 101);

        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    }
}

// Read GPS module
async fn update_gps_state(
    gps_state_sender: Sender<GpsState>,
    gps_satelites_sender: Sender<GpsSatelites>
) {
    loop {
        gps_state_sender.send_modify(|s| {
            s.speed = (s.speed + 5.0) % 120.0;
        });
        gps_satelites_sender.send_modify(|s| {
            s.count = (s.count + 1) % 32;
        });

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}

// Update fahrt state from gps and accelerometer data from the receivers
async fn update_fahrt_state(
    fahrt_sender: Sender<fahrt::FahrtStatus>,
    gps_state_receiver: Receiver<GpsState>,
    gps_satelites_receiver: Receiver<GpsSatelites>,
) {
    let mut timer = tokio::time::interval(std::time::Duration::from_secs(1));
    timer.set_missed_tick_behavior(MissedTickBehavior::Delay);

    loop {
        timer.tick().await;

        let GpsSatelites { count: _satelites_count,.. } = *gps_satelites_receiver.borrow();
        
        fahrt_sender.send_if_modified(|s| {
            match s {
                FahrtStatus::Active(a) => {
                    let GpsState { speed,.. } = *gps_state_receiver.borrow();
                    a.strecke_km += speed as f64 / 3600.0; 

                    true
                }
                FahrtStatus::Inactive => false
            }
        });       
    }
}