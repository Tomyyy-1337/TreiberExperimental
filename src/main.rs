mod global;
mod frontend;
mod backend;

use tokio::{runtime::LocalOptions, task};
use topcoat::{asset::{AssetBundle, RouterBuilderAssetExt}, cookie::RouterBuilderCookieExt, router::{Compression, CompressionLevel, Router, RouterBuilderDiscoverExt}, runtime::RouterBuilderRuntimeExt};

use crate::global::Global;

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
            task::spawn_local(increment_counter());

            topcoat::start(
                Router::builder()
                    .discover()
                    .cookies()
                    .assets(AssetBundle::load().unwrap())
                    .compression(Compression::new().brotli(false).level(CompressionLevel::Fastest))
                    .runtime()
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