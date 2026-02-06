use crate::neurofs::ui_asset_registry::{UiAssetRegistry, UiRelease};

#[derive(Debug, Clone)]
pub struct UiAssetGovernance {
    pub requires_evolve_token: bool,
    pub roh_ceiling: f32,
    pub allowed_domains: Vec<String>, // e.g., ["dashboard", "metrics"]
}

#[derive(Debug)]
pub enum UiAssetError {
    MissingEvolveToken,
    RoHExceedsCeiling,
}

pub struct UiAssetRegistryGuard {
    registry: UiAssetRegistry,
    governance: UiAssetGovernance,
}

impl UiAssetRegistryGuard {
    pub fn new(registry: UiAssetRegistry, governance: UiAssetGovernance) -> Self {
        Self { registry, governance }
    }

    pub fn publish_release(
        &mut self,
        cid_or_hash: impl Into<String>,
        roh_estimate: f32,
        evolve_token_present: bool,
    ) -> Result<&UiRelease, UiAssetError> {
        if self.governance.requires_evolve_token && !evolve_token_present {
            return Err(UiAssetError::MissingEvolveToken);
        }
        if roh_estimate > self.governance.roh_ceiling {
            return Err(UiAssetError::RoHExceedsCeiling);
        }
        Ok(self.registry.publish_release(cid_or_hash))
    }

    pub fn latest_release(&self) -> Option<&UiRelease> {
        self.registry.latest_release()
    }
}
