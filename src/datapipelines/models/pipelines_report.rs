use serde_derive::Serialize;
use crate::models::pipeline_stats::PipelineStats;
use aws_services_lib::datapipelines::models::pipeline::Pipeline;

#[derive(Serialize, Debug, Clone)]
pub struct PipelinesReport {
    pub title: String,
    pub counts: PipelineStats,
    pub statuses: Vec<Pipeline>,
}

impl PipelinesReport {
    pub fn compile(title: &str, pipelines_status: &Vec<Pipeline>) -> PipelinesReport {
        PipelinesReport {
            title: title.to_string(),
            counts: PipelineStats::calc_from(pipelines_status),
            statuses: pipelines_status.iter().cloned().collect(),
        }
    }
}
