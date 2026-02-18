use crate::artifact::SovereignArtifact;
use crate::guards::{AuraBoundaryGuard, SovereignKernelLock};
use crate::error::FsError;
use std::io::{Read, Write};
use std::fs::{File, OpenOptions};

#[derive(Debug, Clone, Copy)]
pub enum FsMode {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

#[derive(Debug)]
pub struct FsHandle {
    artifact: SovereignArtifact,
    file: File,
    mode: FsMode,
    aura_guard: AuraBoundaryGuard,
    kernel_lock: SovereignKernelLock,
    caller_subject: String,
    via_evolve_token: bool,
}

impl FsHandle {
    pub fn open(
        artifact: SovereignArtifact,
        mode: FsMode,
        caller_subject: String,
        via_evolve_token: bool,
    ) -> Result<Self, FsError> {
        let aura_guard = AuraBoundaryGuard;
        let kernel_lock = SovereignKernelLock;

        match mode {
            FsMode::ReadOnly => aura_guard
                .check_read(&caller_subject, &artifact)
                .map_err(FsError::GuardError)?,
            FsMode::WriteOnly | FsMode::ReadWrite => {
                aura_guard
                    .check_write(&caller_subject, &artifact)
                    .map_err(FsError::GuardError)?;
                kernel_lock
                    .check_mutation(&artifact, via_evolve_token)
                    .map_err(FsError::GuardError)?;
            }
        }

        let mut opts = OpenOptions::new();
        match mode {
            FsMode::ReadOnly => {
                opts.read(true);
            }
            FsMode::WriteOnly => {
                opts.write(true).create(true);
            }
            FsMode::ReadWrite => {
                opts.read(true).write(true).create(true);
            }
        }

        let file = opts.open(&artifact.path).map_err(FsError::Io)?;
        Ok(Self {
            artifact,
            file,
            mode,
            aura_guard,
            kernel_lock,
            caller_subject,
            via_evolve_token,
        })
    }

    pub fn read_all(&mut self) -> Result<Vec<u8>, FsError> {
        if !matches!(self.mode, FsMode::ReadOnly | FsMode::ReadWrite) {
            return Err(FsError::ModeError("handle not opened for read".into()));
        }
        let mut buf = Vec::new();
        self.file.read_to_end(&mut buf).map_err(FsError::Io)?;
        Ok(buf)
    }

    pub fn write_all(&mut self, data: &[u8]) -> Result<(), FsError> {
        if !matches!(self.mode, FsMode::WriteOnly | FsMode::ReadWrite) {
            return Err(FsError::ModeError("handle not opened for write".into()));
        }
        self.file.write_all(data).map_err(FsError::Io)?;
        Ok(())
    }

    pub fn artifact(&self) -> &SovereignArtifact {
        &self.artifact
    }
}
