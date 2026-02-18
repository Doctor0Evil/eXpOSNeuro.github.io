use serde::{Deserialize, Serialize};
use crate::artifact::{SovereignArtifact, ArtifactKind};
use crate::fs_handle::{FsHandle, FsMode};
use crate::error::FsError;

/// What external agents may ask NeuroXFS to do.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentOperationKind {
    ReadSummary,
    ReadMetadata,
    AppendNote,
}

/// Agent-visible request (no raw path).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentFsRequest {
    pub subject_id: String,
    pub artifact_id: String, // logical ID, resolved by your manifest, not a path
    pub op: AgentOperationKind,
    pub via_evolve_token: bool,
}

/// Agent-visible response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentFsResponse {
    pub ok: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

/// Trait that Cyb.ai / Agent-Verse adapter implementations use.
pub trait AgentAdapter {
    fn handle_request(&self, req: AgentFsRequest) -> Result<AgentFsResponse, FsError>;
}

/// Simple in-memory resolver from artifact_id -> SovereignArtifact.
pub trait ArtifactResolver {
    fn resolve(&self, subject_id: &str, artifact_id: &str) -> Result<SovereignArtifact, FsError>;
}

pub struct NeuroxfsAgentAdapter<R> {
    resolver: R,
}

impl<R: ArtifactResolver> NeuroxfsAgentAdapter<R> {
    pub fn new(resolver: R) -> Self {
        Self { resolver }
    }

    fn summarize_bytes(bytes: &[u8]) -> String {
        // Very minimal; real implementation would neurorights-filtered summarization.
        let s = String::from_utf8_lossy(bytes);
        let snippet: String = s.chars().take(256).collect();
        format!("summary(snippet): {}", snippet)
    }
}

impl<R: ArtifactResolver> AgentAdapter for NeuroxfsAgentAdapter<R> {
    fn handle_request(&self, req: AgentFsRequest) -> Result<AgentFsResponse, FsError> {
        // Resolve artifact by logical ID.
        let art = self
            .resolver
            .resolve(&req.subject_id, &req.artifact_id)?;

        match req.op {
            AgentOperationKind::ReadSummary => {
                // Only allow summary-style reads on non-kernel artifacts.
                if matches!(art.kind, ArtifactKind::SovereignConfig | ArtifactKind::BChainProof) {
                    return Err(FsError::PolicyError(
                        "Agent cannot read sovereign-config or proof artifacts".into(),
                    ));
                }
                let mut handle = FsHandle::open(
                    art,
                    FsMode::ReadOnly,
                    req.subject_id.clone(),
                    false,
                )?;
                let bytes = handle.read_all()?;
                let summary = Self::summarize_bytes(&bytes);
                Ok(AgentFsResponse {
                    ok: true,
                    message: "summary-ok".into(),
                    data: Some(serde_json::json!({ "summary": summary })),
                })
            }
            AgentOperationKind::ReadMetadata => {
                let art = self
                    .resolver
                    .resolve(&req.subject_id, &req.artifact_id)?;
                Ok(AgentFsResponse {
                    ok: true,
                    message: "metadata-ok".into(),
                    data: Some(serde_json::to_value(&art).map_err(|e| {
                        FsError::PolicyError(format!("metadata serialization error: {}", e))
                    })?),
                })
            }
            AgentOperationKind::AppendNote => {
                // Only allowed on non-kernel, non-neural artifacts.
                if matches!(art.kind, ArtifactKind::SovereignConfig | ArtifactKind::NeuralShard) {
                    return Err(FsError::PolicyError(
                        "Agent cannot append to sovereign-config or raw neural shards".into(),
                    ));
                }
                let mut handle = FsHandle::open(
                    art,
                    FsMode::ReadWrite,
                    req.subject_id.clone(),
                    req.via_evolve_token,
                )?;
                let note = format!("\n# agent-note: {}", req.artifact_id);
                handle.write_all(note.as_bytes())?;
                Ok(AgentFsResponse {
                    ok: true,
                    message: "append-ok".into(),
                    data: None,
                })
            }
        }
    }
}
