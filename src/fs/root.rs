use std::collections::HashMap;
use crate::fs::types::{FileAttr};

#[derive(Debug)]
pub struct RootEntry {
    pub attr: FileAttr,
    pub start_block: u32,
    pub block_count: u32,
}

#[derive(Debug)]
pub struct RootTable {
    entries: HashMap<String, RootEntry>,
}

impl RootTable {
    pub fn new() -> Self {
        Self { entries: HashMap::new() }
    }

    pub fn create(&mut self, entry: RootEntry) -> Result<(), String> {
        if self.entries.contains_key(&entry.attr.name) {
            return Err("File already exists".into());
        }
        self.entries.insert(entry.attr.name.clone(), entry);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&RootEntry> {
        self.entries.get(name)
    }

    pub fn delete(&mut self, name: &str) -> Option<RootEntry> {
        self.entries.remove(name)
    }

    pub fn list(&self) -> impl Iterator<Item = &RootEntry> {
        self.entries.values()
    }
}
