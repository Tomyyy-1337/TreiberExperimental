use serde::Serialize;

#[derive(Clone, Copy, Serialize, Debug, PartialEq)]
pub struct GpsPosition {
    pub latitude: f64,
    pub longitude: f64,
}