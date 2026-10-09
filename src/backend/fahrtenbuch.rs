use std::{collections::VecDeque, time::Duration};
use serde::Serialize;
use tokio::time::Instant;

use crate::global::Global;

pub static FAHRTENBUCH: Global<Fahrtenbuch> = Global::new(Fahrtenbuch::new());

pub static FAHRT_STATE: Global<FahrtStatus> = Global::new(FahrtStatus::Inactive);

const MAX_ENTRIES: usize = 20;

#[derive(Debug, PartialEq, Clone)]
pub enum FahrtStatus {
    Active(ActiveFahrt),
    Inactive,
}

impl FahrtStatus {
    pub fn new_active(current_time: String) -> Self {
        FahrtStatus::Active(ActiveFahrt {
            start_time: current_time,
            start_time_internal: Instant::now(),
            strecke_km: 0.0,
            schläge: 0,
            position_history: Vec::new(),
        })
    }

    pub fn is_active(&self) -> bool {
        match self {
            FahrtStatus::Active(_) => true,
            FahrtStatus::Inactive => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ActiveFahrt {
    start_time: String,
    start_time_internal: Instant,
    strecke_km: f64,
    schläge: u32,
    position_history: Vec<GpsPosition>,
}

impl ActiveFahrt {
    pub fn finish(self) -> FinishedFahrt {
        FinishedFahrt {
            dauer: self.start_time_internal.elapsed(),
            start_time: self.start_time,
            strecke_km: self.strecke_km,
            schläge: self.schläge,
            position_history: self.position_history,
        }
    }
}


pub struct Fahrtenbuch {
    pub next_id: u32,
    pub entries: VecDeque<Indexed<FinishedFahrt>>,
}

pub struct Indexed<T> {
    pub id: u32,
    pub entry: T,
}

#[derive(Clone)]
pub struct FinishedFahrt {
    pub start_time: String,
    pub dauer: Duration,
    pub strecke_km: f64,
    pub schläge: u32,
    pub position_history: Vec<GpsPosition>,
}

#[derive(Clone, Copy, Serialize, Debug, PartialEq)]
pub struct GpsPosition {
    pub latitude: f64,
    pub longitude: f64,
}

impl Fahrtenbuch {
    pub const fn new() -> Self {
        Self {
            next_id: 0,
            entries: VecDeque::new(),
        }
    }

    pub fn get(&self, id: u32) -> Option<&FinishedFahrt> {
        self.entries.iter().find(|entry| entry.id == id).map(|indexed| &indexed.entry)
    }

    pub fn get_id_of_next(&self, id: u32) -> Option<u32> {
        let current_index = self.entries.iter().position(|entry| entry.id == id)?;

        let next_index = current_index + 1;
        if next_index >= self.entries.len() {
            return None;
        }

        Some(self.entries[next_index].id)
    }

    pub fn get_id_of_previous(&self, id: u32) -> Option<u32> {
        let current_index = self.entries.iter().position(|entry| entry.id == id)?;

        if current_index == 0 {
            return None;
        }

        Some(self.entries[current_index - 1].id)
    }

    pub fn add_entry(&mut self, entry: FinishedFahrt) {
        let entry = Indexed::<FinishedFahrt> { 
            id: self.next_id,
            entry,
        };
        self.next_id += 1;
        if self.entries.len() >= MAX_ENTRIES {
            self.entries.pop_front();
        }
        self.entries.push_back(entry);
    }
}