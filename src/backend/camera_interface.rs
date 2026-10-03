use std::time::Duration;

use reqwest::Client;
use serde::Serialize;

use crate::global::Global;

pub static CAMERA_INTERFACE: Global<CameraInterface> = Global::new(CameraInterface {
    hdr_enabled: true,
    exposure_compenstion: 0.0,
    metering_mode: Metering::Average,
    focus_mode: FocusMode::Fixed,
    focal_length: 28.0,
    bitrate: 3_200_000,
});

#[derive(Serialize, Debug, Copy, Clone)]
pub enum Metering {
    Center,
    Average,
}

#[derive(Serialize, Debug, Copy, Clone)]
pub enum FocusMode {
    Auto,
    Fixed,
}

pub struct CameraInterface {
    pub hdr_enabled: bool,
    pub exposure_compenstion: f32,
    pub metering_mode: Metering,
    pub focus_mode: FocusMode,
    pub focal_length: f32,
    pub bitrate: u32,
}

impl CameraInterface {
    pub fn send_to_camera(&self) {
        let client = Client::builder().build().unwrap();

        let config = CameraConfig::from(self);

        tokio::spawn(async move {
            let _e = internal_update_camera_config(&client, &config).await;
            // println!("Result of updating camera config: {:?}", e);
        });
    }

    fn get_roi(&self) -> String {
        const BASE_FOCAL_LENGTH: f32 = 28.0;
        
        let crop = BASE_FOCAL_LENGTH / self.focal_length;
        let offset = (1.0 - crop) / 2.0;

        format!("{offset:.6},{offset:.6},{crop:.6},{crop:.6}")
    }
}
    

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
struct CameraConfig {
    pub rpiCameraHDR: bool,
    pub rpiCameraEV: f32,
    pub rpiCameraMetering: &'static str,
    pub rpiCameraAfMode: &'static str,
    pub rpiCameraROI: String,
    pub rpiCameraBitrate: u32,
}

impl From<&CameraInterface> for CameraConfig {
    fn from(camera: &CameraInterface) -> Self {
        CameraConfig {
            rpiCameraHDR: camera.hdr_enabled,
            rpiCameraEV: camera.exposure_compenstion,
            rpiCameraMetering: camera.metering_mode.name(),
            rpiCameraAfMode: camera.focus_mode.name(),
            rpiCameraROI: camera.get_roi(),
            rpiCameraBitrate: camera.bitrate,
        }
    }
}

impl Metering {
    fn name(&self) -> &'static str {
        match self {
            Metering::Center => "centre",
            Metering::Average => "matrix",
        }
    }
}

impl FocusMode {
    fn name(&self) -> &'static str {
        match self {
            FocusMode::Auto => "continuous",
            FocusMode::Fixed => "manual",
        }
    }
}

#[allow(dead_code)]
async fn internal_update_camera_config(
    client: &Client,
    config: &CameraConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = client
        .patch("http://127.0.0.1:9997/v3/config/paths/patch/stream")
        .json(config)
        .timeout(Duration::from_secs(5))
        .send()
        .await?;

    let status = response.status();
    if status.is_success() {
        Ok(())
    } else {
        let body = response.text().await.unwrap_or_default();
        Err(format!("API error: {status}: {body}").into())
    }
}