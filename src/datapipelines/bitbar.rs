use crate::models::pipelines_report::PipelinesReport;
use aws_services_lib::datapipelines::models::pipeline::Pipeline;
use crate::models::pipeline_execution_status::PipelineExecutionStatus;

pub fn display_colour(pipelines_status: &PipelineExecutionStatus) -> &str {
    match pipelines_status {
        PipelineExecutionStatus::OKAY => "green",
        PipelineExecutionStatus::BUILDING => "orange",
        PipelineExecutionStatus::ERROR => "red",
        PipelineExecutionStatus::WARNING => "red",
    }
}

pub fn display_icon(pipelines_status: &PipelineExecutionStatus) -> &str {
    match pipelines_status {
        PipelineExecutionStatus::OKAY => "large_blue_circle",
        PipelineExecutionStatus::BUILDING => "large_orange_diamond",
        PipelineExecutionStatus::ERROR => "red_circle",
        PipelineExecutionStatus::WARNING => "large_orange_diamond",
    }
}

pub fn display_building(pipeline_report: &PipelinesReport) -> &str {
    if pipeline_report.counts.error_building > 0 { "!" }
    else if pipeline_report.counts.healthy_building > 0 { "." }
    else { "" }
}

pub fn format_header(pipeline_report: &PipelinesReport) -> Vec<String> {
    let pipelines_status = PipelineExecutionStatus::from(pipeline_report);
    vec![
        format!(":{}:{}{} | color={}",
                display_icon(&pipelines_status),
                pipeline_report.counts.error,
                display_building(&pipeline_report),
                display_colour(&pipelines_status)),
        "---".to_string(),
        format!("[{}]", pipeline_report.title)
    ]
}

pub fn sort_pipelines(pipelines: Vec<Pipeline>) -> Vec<Pipeline> {
    let mut sorted_pipelines = pipelines.clone();
    sorted_pipelines.sort_by(|a, b| a.health_status.cmp(&b.health_status));
    sorted_pipelines.sort_by(|a, b| b.is_building().cmp(&a.is_building()));
    sorted_pipelines
}

pub fn format_body(pipeline_report: &PipelinesReport) -> Vec<String> {
    let local_copy = pipeline_report.statuses.iter().map(|p| p.clone()).collect();
    let sorted_pipelines = sort_pipelines(local_copy);
    sorted_pipelines.iter().flat_map(|ps| {
        let colour = {
            let blue_component = if ps.is_building() { "66" } else { "00" };
            let health_base_colour = if ps.is_healthy() { "#00FF" } else { "#FF00" };
            format!("{}{}", health_base_colour, blue_component)
        };
        let main_status = format!("{} {} | color={}",
                ps.name,
                if ps.is_building() { "**building**" } else { "" },
                colour
        );
        let mut body: Vec<String> = ps.tasks.iter().map(|t|{
            format!("-- {} {} | color={}", t.status, t.task_name, colour)
        }).collect();
        body.insert(0, main_status);
        body
    }).collect()
}

pub fn format_display(pipeline_report: &PipelinesReport) -> String {
    let mut display = format_header(pipeline_report);
    let mut body = format_body(pipeline_report);
    display.append(&mut body);
    display.iter().fold("".to_string(), |acc, x| format!("{}\n{}", acc, x))
}

