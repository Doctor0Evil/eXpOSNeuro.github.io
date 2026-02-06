#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Root,
    Data,
    Exec,
    NeuroStream,   // continuous neural/bioscale stream
    BioSnapshot,   // lifeforce / fatigue snapshots
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    Exclusive,
    Open,
    SharedRead,
    SharedWrite,
}

#[derive(Debug, Clone)]
pub struct NeuroRights {
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
pub struct FileAttr {
    pub name: String,
    pub owner: String,
    pub size_words: u32,
    pub file_type: FileType,
    pub perm: Permission,
    pub neurorights: Option<NeuroRights>,
}
