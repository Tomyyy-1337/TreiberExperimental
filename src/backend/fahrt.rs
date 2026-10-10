use tokio::time::{Duration, Instant};

use crate::{backend::gps_interface::GpsPosition, global::Global};


pub static FAHRT_STATE: Global<FahrtStatus> = Global::new(FahrtStatus::Inactive);

#[derive(Debug, PartialEq, Clone)]
pub enum FahrtStatus {
    Active(Fahrt<Active>),
    Inactive,
}

impl FahrtStatus {
    pub fn new_active(current_time: String) -> Self {
        FahrtStatus::Active(Fahrt::new(current_time))
    }

    pub fn is_active(&self) -> bool {
        match self {
            FahrtStatus::Active(_) => true,
            FahrtStatus::Inactive => false,
        }
    }
}

pub type Active = Instant;
pub type Finished = Duration;

#[derive(Debug, Clone, PartialEq)]
pub struct Fahrt<D: FahrtDurationTrait> {
    timestamp: D,
    pub start_time: String,
    pub strecke_km: f64,
    pub schläge: u32,
    pub position_history: Vec<GpsPosition>,
}

impl Fahrt<Active> {
    pub fn finish(self) -> Fahrt<Finished> {
        Fahrt {
            timestamp: self.timestamp.duration(),
            start_time: self.start_time,
            strecke_km: self.strecke_km,
            schläge: self.schläge,
            position_history: self.position_history,
        }
    }

    pub fn new(start_time: String) -> Self {
        Self {
            timestamp: Instant::now(),
            start_time,
            strecke_km: 0.0,
            schläge: 0,
            position_history: Vec::new(),
        }
    }
}

impl<D: FahrtDurationTrait> Fahrt<D> {
    pub fn formated_duration_mm_ss(&self) -> String {
        let duration = self.timestamp.duration();
        let secs = duration.as_secs();
        let minutes = secs / 60;
        let seconds = secs % 60;
        format!("{:02}:{:02}", minutes, seconds)
    }

    pub fn formated_distance_km(&self, number_of_digits: usize) -> String {
        format!("{:.1$}", self.strecke_km, number_of_digits)
    }

    pub fn average_split_time_s(&self) -> Option<f64> {
        if self.strecke_km > 0.0 {
            Some(self.timestamp.duration().as_secs_f64() / (self.strecke_km * 1000.0 / 500.0))
        } else {
            None
        }
    }

    pub fn average_split_time_mm_ss<S: ToString>(&self, invalid_placeholder: S) -> String {
        match self.average_split_time_s() {
            Some(split_time_s) => Self::mm_ss_format(split_time_s),
            None => invalid_placeholder.to_string(),
        }
    }

    fn mm_ss_format(seconds: f64) -> String {
        let minutes = (seconds / 60.0).floor() as u64;
        let seconds = (seconds % 60.0).round() as u64;
        format!("{:02}:{:02}", minutes, seconds)
    }

    pub fn average_speed_kmh(&self) -> Option<f64> {
        if self.timestamp.duration().as_secs_f64() > 0.0 {
            Some(self.strecke_km / (self.timestamp.duration().as_secs_f64() / 3600.0))
        } else {
            None
        }
    }

    pub fn average_speed_kmh_formated(&self, number_of_digits: usize, invalid_placeholder: &str) -> String {
        match self.average_speed_kmh() {
            Some(speed) => format!("{:.1$}", speed, number_of_digits),
            None => invalid_placeholder.to_string(),
        }
    }

    pub fn average_stroke_rate(&self) -> Option<f64> {
        if self.timestamp.duration().as_secs_f64() > 0.0 {
            Some(self.schläge as f64 / (self.timestamp.duration().as_secs_f64() / 60.0))
        } else {
            None
        }
    }

    pub fn average_stroke_rate_formated(&self, number_of_digits: usize, invalid_placeholder: &str) -> String {
        match self.average_stroke_rate() {
            Some(rate) => format!("{:.1$}", rate, number_of_digits),
            None => invalid_placeholder.to_string(),
        }
    }
}

pub trait FahrtDurationTrait {
    fn duration(&self) -> Duration;
}

impl FahrtDurationTrait for Duration {
    fn duration(&self) -> Duration {
        *self
    }
}

impl FahrtDurationTrait for Instant {
    fn duration(&self) -> Duration {
        self.elapsed()
    }
}