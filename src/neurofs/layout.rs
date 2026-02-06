#[derive(Debug, Clone)]
pub enum NodeKind {
    Directory,
    File,
}

#[derive(Debug, Clone)]
pub struct FsNode {
    pub path: String,
    pub kind: NodeKind,
    pub description: Option<String>,
}

pub fn bootstrap_nexsm_layout() -> Vec<FsNode> {
    vec![
        FsNode {
            path: ".github/workflows/bootstrap-nexsm.yml".into(),
            kind: NodeKind::File,
            description: Some("CI pipeline for Bootstrap v5 NEXSM dashboard".into()),
        },
        FsNode {
            path: "frontend/".into(),
            kind: NodeKind::Directory,
            description: Some("UI source using Bootstrap 5".into()),
        },
        FsNode {
            path: "frontend/package.json".into(),
            kind: NodeKind::File,
            description: Some("Frontend dependencies and scripts".into()),
        },
        FsNode {
            path: "frontend/vite.config.js".into(),
            kind: NodeKind::File,
            description: Some("Build tool configuration".into()),
        },
        FsNode {
            path: "public/".into(),
            kind: NodeKind::Directory,
            description: Some("Built UI/asset output".into()),
        },
        FsNode {
            path: "server/".into(),
            kind: NodeKind::Directory,
            description: Some("Simulation backend (optional)".into()),
        },
    ]
}
