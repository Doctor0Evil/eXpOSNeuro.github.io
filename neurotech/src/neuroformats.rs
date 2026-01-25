use neuroformats::{read_surf, read_curv, read_mgh};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NeuroDataError {
    #[error("Failed to read surface file: {0}")]
    SurfaceReadError(String),
}

pub struct BrainMesh {
    pub vertices: Vec<[f32; 3]>,
    pub faces: Vec<[i32; 3]>,
    pub thickness: Option<Vec<f32>>, // Cortical thickness per vertex
}

pub fn load_hemisphere(surf_path: &str, thickness_path: Option<&str>) -> Result<BrainMesh, NeuroDataError> {
    let surface = read_surf(surf_path).map_err(|e| NeuroDataError::SurfaceReadError(e.to_string()))?;
    let thickness = if let Some(path) = thickness_path {
        Some(read_curv(path).unwrap().data)
    } else {
        None
    };
    Ok(BrainMesh {
        vertices: surface.mesh.vertices().to_vec(),
        faces: surface.mesh.faces().to_vec(),
        thickness,
    })
}
// You can extend this to load MGH volumes for voxel-based analysis[citation:1]
