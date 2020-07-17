pub enum PipelineHealthVisibility {
    ShowAll,
    ShowHealthy,
    ShowError
}

use crate::models::pipeline_health_visibility::PipelineHealthVisibility::{ShowHealthy, ShowError, ShowAll};

impl PipelineHealthVisibility {

    pub fn from_string(s: &str) -> PipelineHealthVisibility {
        match s {
            "healthy" => ShowHealthy,
            "error" => ShowError,
            _ => ShowAll,
        }
    }

    pub fn variants() -> Vec<&'static str> {
        vec![ "all", "error", "healthy" ]
    }
}