use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArtifactKind {
    NeuralShard,
    NeuroRightsPolicy,
    EvolveStream,
    DonutLedger,
    BChainProof,
    Model,
    SovereignConfig,
    GenericData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeurorightsProfile {
    pub mental_privacy: bool,
    pub dreamstate_sensitive: bool,
    pub soul_non_tradeable: bool,
    pub forbid_decision_use: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SovereignArtifact {
    pub path: String,
    pub subject_id: String, // Bostrom / OrganicCPU subject
    pub kind: ArtifactKind,
    pub routes: Vec<String>, // e.g., ["CHAT", "BCI", "OTA"]
    pub roh_before: f32,
    pub roh_after: f32,
    pub neurorights: NeurorightsProfile,
    pub lifeforce_cost: f32,
    pub governance_tags: Vec<String>, // EVOLVE, SMART, etc.
}
