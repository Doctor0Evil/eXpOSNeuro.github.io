use crate::artifact::SovereignArtifact;

#[derive(Debug, Clone)]
pub struct AuraBoundaryGuard;

#[derive(Debug, Clone)]
pub struct SovereignKernelLock;

impl AuraBoundaryGuard {
    /// Enforce cross-subject + neurorights boundaries (no foreign NEUROSTREAM, no dream export).
    pub fn check_read(&self, caller_subject: &str, art: &SovereignArtifact) -> Result<(), String> {
        if caller_subject != art.subject_id {
            return Err("AuraBoundaryGuard: cross-subject access denied".into());
        }
        if art.neurorights.mental_privacy && matches!(art.kind, crate::artifact::ArtifactKind::NeuralShard) {
            return Err("AuraBoundaryGuard: mental privacy forbids raw neural shard export".into());
        }
        if art.neurorights.dreamstate_sensitive && art.neurorights.forbid_decision_use {
            // Allow only local introspection routes, not generic AI agents.
            if !art.routes.contains(&"INTROSPECT".to_string()) {
                return Err("AuraBoundaryGuard: dreamstate-sensitive shard not exposed on this route".into());
            }
        }
        Ok(())
    }

    pub fn check_write(&self, caller_subject: &str, art: &SovereignArtifact) -> Result<(), String> {
        if caller_subject != art.subject_id {
            return Err("AuraBoundaryGuard: cross-subject write denied".into());
        }
        if art.neurorights.soul_non_tradeable && art.governance_tags.contains(&"EXPORT".to_string()) {
            return Err("AuraBoundaryGuard: soul-non-tradeable artifact cannot be exported or tokenized".into());
        }
        Ok(())
    }
}

impl SovereignKernelLock {
    /// Only EVOLVE path may change sovereign-kernel artifacts.
    pub fn check_mutation(&self, art: &SovereignArtifact, via_evolve_token: bool) -> Result<(), String> {
        use crate::artifact::ArtifactKind::*;
        let is_kernel = matches!(art.kind, SovereignConfig | EvolveStream | DonutLedger | BChainProof);
        if is_kernel && !via_evolve_token {
            return Err("SovereignKernelLock: mutation requires EVOLVE token path".into());
        }
        Ok(())
    }
}
