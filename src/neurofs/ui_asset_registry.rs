use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct UiRelease {
    pub id: u64,
    pub cid_or_hash: String,
    pub timestamp: u64,
}

#[derive(Debug, Default)]
pub struct UiAssetRegistry {
    releases: Vec<UiRelease>,
}

impl UiAssetRegistry {
    pub fn new() -> Self {
        Self { releases: Vec::new() }
    }

    pub fn publish_release(&mut self, cid_or_hash: impl Into<String>) -> &UiRelease {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let id = (self.releases.len() as u64) + 1;

        self.releases.push(UiRelease {
            id,
            cid_or_hash: cid_or_hash.into(),
            timestamp: ts,
        });

        self.releases.last().unwrap()
    }

    pub fn latest_release(&self) -> Option<&UiRelease> {
        self.releases.last()
    }

    pub fn all_releases(&self) -> impl Iterator<Item = &UiRelease> {
        self.releases.iter()
    }
}
