use std::collections::VecDeque;

use crate::global::Global;

pub static FAHRTENBUCH_STATE: Global<Fahrtenbuch> = Global::new(Fahrtenbuch::new());

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

pub struct Fahrt {
    pub gesamtstrecke: f64,
}