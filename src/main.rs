mod global;
mod frontend;
mod backend;

use tokio::{runtime::LocalOptions, task};
use topcoat::{asset::{AssetBundle, RouterBuilderAssetExt}, cookie::RouterBuilderCookieExt, router::{Compression, CompressionLevel, OriginPolicy, Router, RouterBuilderDiscoverExt}, runtime::{PrefetchMode, RouterBuilderRuntimeExt}};

use crate::{backend::camera_interface::CAMERA_INTERFACE, global::Global}; 

struct SharedState {
    counter: u64,
    battery_percentage: u8,
}

static SHARED_STATE: Global<SharedState> = Global::new(
    SharedState {
        counter: 0,
        battery_percentage: 0,
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
        SHARED_STATE.modify(|state| {
            state.counter += 1;
            state.battery_percentage = (state.battery_percentage + 2) % 101;
        });
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }
}