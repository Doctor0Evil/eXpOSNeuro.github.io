use crate::fs::types::{FileAttr};
use crate::fs::root::RootTable;
use crate::fs::class::{FileClass, classify};

pub struct FsHandle {
    pub root: RootTable,
    // backing store (disk image, device, or Couchbase / IPFS mapping)
}

impl FsHandle {
    pub fn create(&mut self, attr: FileAttr) -> Result<(), String> {
        let class = classify(&attr.name, attr.file_type);

        // enforce neurorights and sovereignty invariants
        if let Some(neuro) = &attr.neurorights {
            if neuro.soulnontradeable && class == FileClass::NeuralModel {
                return Err("Cannot store soulnontradeable data in a generic neural model file".into());
            }
        }

        // allocate blocks, update root, data structures
        // ...
        Ok(())
    }

    pub fn read(&self, name: &str, offset_words: u32, len_words: u32) -> Result<Vec<u32>, String> {
        // translate to block offsets like eXpFS, but with additional RoH checks before returning
        // ...
        Ok(Vec::new())
    }

    pub fn write(&mut self, name: &str, offset_words: u32, data: &[u32]) -> Result<(), String> {
        // check RoH + Tsafe; log into .ocpulog & .donutloop.aln
        // ...
        Ok(())
    }

    pub fn delete(&mut self, name: &str) -> Result<(), String> {
        // update free list, donutloop, ocpu logs
        // ...
        Ok(())
    }
}
