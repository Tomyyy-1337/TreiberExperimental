use std::{str::FromStr, time::Duration};

use reqwest::Client;
use serde::Serialize;

lazy_static::lazy_static! {
    static ref CLIENT: Client = Client::builder().build().unwrap();
}

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

#[derive(Debug, Clone)]
pub struct CameraInterface {
    pub hdr_enabled: bool,
    pub exposure_compenstion: f32,
    pub metering_mode: Metering,
    pub focus_mode: FocusMode,
    pub focal_length: u8,
    pub bitrate: u32,
}

impl CameraInterface {
    pub fn new() -> Self {
        CameraInterface {
            hdr_enabled: true,
            exposure_compenstion: 0.0,
            metering_mode: Metering::Average,
            focus_mode: FocusMode::Fixed,
            focal_length: 28,
            bitrate: 3_200_000,
        }
    }

    pub async fn send_to_camera(&self) -> Result<(), reqwest::Error> {
        let config = CameraConfig::from(self);

        let response = CLIENT
            .patch("http://127.0.0.1:9997/v3/config/paths/patch/stream")
            .json(&config)
            .timeout(Duration::from_secs(5))
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            println!("API error: {status}: {body}");
        }
        Ok(())
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