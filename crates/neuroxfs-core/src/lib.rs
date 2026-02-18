pub mod artifact;
pub mod guards;
pub mod fs_handle;
pub mod kernel_lock; // optional wrapper re-export if you want
pub mod aura_boundary; // can re-export from guards or split
pub mod policy; // future Tsafe / RoH integration
pub mod error;
pub mod agent_adapter;
