use std::collections::HashSet;

use crate::library::visit::{Visit, is_recordable};

pub const MAX_VISITS: usize = 5000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Recording {
    Added,
    Retitled,
    Ignored,
}

#[derive(Debug, Default)]
pub struct History {
    visits: Vec<Visit>,
}

impl History {
    pub fn parse(contents: &str) -> Self {
        let mut visits: Vec<Visit> = contents.lines().filter_map(Visit::decode).collect();
        let excess = visits.len().saturating_sub(MAX_VISITS);
        visits.drain(..excess);
        Self { visits }
    }

    pub fn serialize(&self) -> String {
        self.visits
            .iter()
            .map(|visit| visit.encode() + "\n")
            .collect()
    }

    pub fn len(&self) -> usize {
        self.visits.len()
    }

    pub fn last(&self) -> Option<&Visit> {
        self.visits.last()
    }

    pub fn record(&mut self, visit: Visit) -> Recording {
        if !is_recordable(&visit.uri) {
            return Recording::Ignored;
        }
        match self.visits.last_mut() {
            Some(last) if last.uri == visit.uri => retitle(last, visit.title),
            _ => {
                self.visits.push(visit);
                if self.visits.len() > MAX_VISITS {
                    self.visits.remove(0);
                }
                Recording::Added
            }
        }
    }

    pub fn search(&self, query: &str, limit: usize) -> Vec<Visit> {
        let query = query.trim().to_lowercase();
        let mut seen = HashSet::new();
        self.visits
            .iter()
            .rev()
            .filter(|visit| visit.matches(&query))
            .filter(|visit| seen.insert(visit.uri.as_str()))
            .take(limit)
            .cloned()
            .collect()
    }

    pub fn clear(&mut self) {
        self.visits.clear();
    }
}

fn retitle(visit: &mut Visit, title: String) -> Recording {
    if !visit.title.trim().is_empty() || title.trim().is_empty() {
        return Recording::Ignored;
    }
    visit.title = title;
    Recording::Retitled
}

#[cfg(test)]
mod tests;
