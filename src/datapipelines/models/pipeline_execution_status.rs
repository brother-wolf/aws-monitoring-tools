use crate::models::pipelines_report::PipelinesReport;

pub enum PipelineExecutionStatus {
    OKAY,
    BUILDING,
    ERROR,
    WARNING
}

impl PipelineExecutionStatus {
    pub fn from(pipeline_report: &PipelinesReport) -> PipelineExecutionStatus {
        if pipeline_report.statuses.is_empty() {
            PipelineExecutionStatus::WARNING
        } else if pipeline_report.counts.error == 0 {
            PipelineExecutionStatus::OKAY
        } else if pipeline_report.counts.error == pipeline_report.counts.error_building {
            PipelineExecutionStatus::BUILDING
        } else {
            PipelineExecutionStatus::ERROR
        }
    }
}