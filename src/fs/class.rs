use crate::fs::types::FileType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileClass {
    Root,
    SovereignConfig,
    Ledger,
    NeuralModel,
    StreamShard,
    Biospec,
    GenericData,
}

pub fn classify(name: &str, ty: FileType) -> FileClass {
    if ty == FileType::Root { return FileClass::Root; }

    if name.ends_with(".neurorights.json")
        || name.ends_with(".stake.aln")
        || name.ends_with("neuro-workspace.manifest.aln")
    {
        FileClass::SovereignConfig
    } else if name.ends_with(".donutloop.aln")
        || name.ends_with(".evolve.jsonl")
        || name.ends_with(".answer.ndjson")
        || name.ends_with(".nnet-loop.aln")
    {
        FileClass::Ledger
    } else if name.ends_with(".nnetx")
        || name.ends_with(".nnetw")
        || name.ends_with(".nnetq")
    {
        FileClass::NeuralModel
    } else if name.ends_with(".nstream.neuroaln")
        || name.ends_with(".neuroaln")
        || name.ends_with(".lifaln")
    {
        FileClass::StreamShard
    } else if name.ends_with(".biospec.aln")
        || name.ends_with(".ocpuenv")
        || name.ends_with(".ocpulog")
    {
        FileClass::Biospec
    } else {
        FileClass::GenericData
    }
}
