use std::{collections::VecDeque, time::Duration};
use serde::Serialize;

use crate::global::Global;

pub static FAHRTENBUCH: Global<Fahrtenbuch> = Global::new(Fahrtenbuch::new());

const MAX_ENTRIES: usize = 20;

pub struct Fahrtenbuch {
    pub next_id: u32,
    pub entries: VecDeque<IndexedFahrt>,
}

impl Fahrtenbuch {
    pub const fn new() -> Self {
        Self {
            next_id: 0,
            entries: VecDeque::new(),
        }
    }

    pub fn get(&self, id: u32) -> Option<&Fahrt> {
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

    pub fn add_entry(&mut self, entry: Fahrt) {
        let entry = IndexedFahrt {
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

pub struct IndexedFahrt {
    pub id: u32,
    pub entry: Fahrt,
}

#[derive(Clone)]
pub struct Fahrt {
    pub start_time: String,
    pub dauer: Duration,
    pub strecke_km: f64,
    pub schläge: u32,
    pub position_history: Vec<GpsPosition>,
}

#[derive(Clone, Copy, Serialize)]
pub struct GpsPosition {
    pub latitude: f64,
    pub longitude: f64,
}