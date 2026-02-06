use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsBlockClass {
    Generic,
    NeuroStream,
    BioSpec,
    Ledger,
    Model,
    SovereignConfig,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsFileType {
    Root,
    Data,
    Exec,
    NeuroStream,
    BioSpec,
    Ledger,
    Model,
    SovereignConfig,
}

#[derive(Debug, Clone)]
pub struct NeurorightsFlags {
    pub mental_privacy: bool,
    pub mental_integrity: bool,
    pub cognitive_liberty: bool,
    pub noncommercial_neural_data: bool,
    pub soulnontradeable: bool,
    pub dreamstate_sensitive: bool,
    pub forbid_decision_use: bool,
    pub forget_sla_hours: u32,
}

#[derive(Debug, Clone)]
pub struct SmartScope {
    pub maxeffectsizel2: f32,
    pub domains: Vec<String>,
    pub expiry: Option<Duration>,
    pub physioguard_enabled: bool,
    pub revocable: bool,
}

#[derive(Debug, Clone)]
pub struct EvolveRequirement {
    pub required: bool,
    pub scope_paths: Vec<String>,
    pub roh_ceiling: f32,
}

#[derive(Debug, Clone)]
pub struct ShardGovernance {
    pub neurorights: NeurorightsFlags,
    pub smart_scope: Option<SmartScope>,
    pub evolve: EvolveRequirement,
}

#[derive(Debug, Clone)]
pub struct ShardClassSpec {
    pub file_type: FsFileType,
    pub block_class: FsBlockClass,
    pub extensions: Vec<String>,
    pub description: String,
    pub governance: ShardGovernance,
}

#[derive(Debug, Clone)]
pub struct OrganicCpuFsSpec {
    pub disk_block_words: u32,
    pub shard_classes: Vec<ShardClassSpec>,
}
