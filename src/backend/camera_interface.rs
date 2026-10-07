use std::{str::FromStr, time::Duration};

use reqwest::Client;
use serde::Serialize;

use crate::global::Global;

pub static CAMERA_INTERFACE: Global<CameraInterface> = Global::new(CameraInterface {
    hdr_enabled: true,
    exposure_compenstion: 0.0,
    metering_mode: Metering::Average,
    focus_mode: FocusMode::Fixed,
    focal_length: 28,
    bitrate: 3_200_000,
});

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum Metering {
    Center,
    Average,
}

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum FocusMode {
    Auto,
    Fixed,
}

pub struct CameraInterface {
    pub hdr_enabled: bool,
    pub exposure_compenstion: f32,
    pub metering_mode: Metering,
    pub focus_mode: FocusMode,
    pub focal_length: u8,
    pub bitrate: u32,
}

impl Global<CameraInterface> {
    pub async fn modify_and_send<F>(&self, f: F)
    where
        F: FnOnce(&mut CameraInterface),
    {
        self.modify(|s| f(s));
        
        self.send_to_camera();
    }
}

impl CameraInterface {
    pub fn send_to_camera(&self) {
        let config = CameraConfig::from(self);
        
        tokio::task::spawn_local(async move { 
            let client = Client::builder().build().unwrap();
            
            let err = internal_update_camera_config(&client, &config).await;
            if let Err(_e) = err {
                println!("Camera can not be updated");
            }
        });
    }

    fn get_roi(&self) -> String {
        const BASE_FOCAL_LENGTH: f32 = 28.0;
        
        let crop = BASE_FOCAL_LENGTH / self.focal_length as f32;
        let offset = (1.0 - crop) / 2.0;

        format!("{offset:.6},{offset:.6},{crop:.6},{crop:.6}")
    }
}
    

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
struct CameraConfig {
    pub rpiCameraHDR: bool,
    pub rpiCameraEV: f32,
    pub rpiCameraMetering: String,
    pub rpiCameraAfMode: String,
    pub rpiCameraROI: String,
    pub rpiCameraBitrate: u32,
}

impl From<&CameraInterface> for CameraConfig {
    fn from(camera: &CameraInterface) -> Self {
        CameraConfig {
            rpiCameraHDR: camera.hdr_enabled,
            rpiCameraEV: camera.exposure_compenstion,
            rpiCameraMetering: camera.metering_mode.to_string(),
            rpiCameraAfMode: camera.focus_mode.to_string(),
            rpiCameraROI: camera.get_roi(),
            rpiCameraBitrate: camera.bitrate,
        }
    }
}

impl ToString for Metering {
    fn to_string(&self) -> String {
        match self {
            Metering::Center => "centre".to_owned(),
            Metering::Average => "matrix".to_owned(),
        }
    }
}

impl FromStr for Metering {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "centre" => Ok(Metering::Center),
            "matrix" => Ok(Metering::Average),
            _ => Err(()),
        }
    }
}

impl ToString for FocusMode {
    fn to_string(&self) -> String {
        match self {
            FocusMode::Auto => "continuous".to_owned(),
            FocusMode::Fixed => "manual".to_owned(),
        }
    }
}

impl FromStr for FocusMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "continuous" => Ok(FocusMode::Auto),
            "manual" => Ok(FocusMode::Fixed),
            _ => Err(()),
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