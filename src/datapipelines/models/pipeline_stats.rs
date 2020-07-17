use aws_services_lib::datapipelines::models::pipeline::Pipeline;
use serde_derive::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct PipelineStats {
    pub healthy: i64,
    pub healthy_building: i64,
    pub error: i64,
    pub error_building: i64,
}

impl PipelineStats {
    fn count(pipelines_status: &Vec<Pipeline>, condition: fn(&&Pipeline) -> bool) -> i64 {
        pipelines_status.iter().filter(condition).fold(0, |acc, _| acc + 1)
    }

    pub fn calc_from(pipelines_status: &Vec<Pipeline>) -> PipelineStats {
        PipelineStats {
            healthy: PipelineStats::count(&pipelines_status, |x| x.is_healthy()),
            error: PipelineStats::count(&pipelines_status, |x| !x.is_healthy()),
            error_building: PipelineStats::count(&pipelines_status, |x| !x.is_healthy() && x.is_building()),
            healthy_building: PipelineStats::count(&pipelines_status, |x| x.is_healthy() && x.is_building()),
        }
    }
}

#[test]
fn pipeline_stats_should_add_up_correctly() {
    let expected = PipelineStats { healthy: 3, healthy_building: 1, error: 4, error_building: 3 };
    let pipelines = vec![
        pipeline_stats_test_pipeline_builder("a", true, false),
        pipeline_stats_test_pipeline_builder("b", true, false),
        pipeline_stats_test_pipeline_builder("c", true, true),
        pipeline_stats_test_pipeline_builder("d", false, false),
        pipeline_stats_test_pipeline_builder("e", false, true),
        pipeline_stats_test_pipeline_builder("f", false, true),
        pipeline_stats_test_pipeline_builder("g", false, true),
    ];
    let actual = PipelineStats::calc_from(&pipelines);
    assert_eq!(actual, expected);
}
    
#[cfg(test)] use std::collections::hash_map::HashMap;
#[cfg(test)] use aws_services_lib::datapipelines::models::pipeline_tasks::PipelineTasks;
#[cfg(test)] use chrono::{Utc,TimeZone};

#[cfg(test)]
fn pipeline_stats_test_pipeline_builder(name: &str, is_healthy: bool, is_building: bool) -> Pipeline {
    let maybe_pipeline = Pipeline::create(
        pipeline_stats_test_pipeline_tasks_builder(name, is_building), 
        pipeline_stats_test_aws_fields_builder(name, is_healthy), 
        Utc.datetime_from_str("2020-01-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap());

    assert!(maybe_pipeline.is_some());
    maybe_pipeline.unwrap()
}

#[cfg(test)]
fn pipeline_stats_test_pipeline_tasks_builder(name: &str, is_building: bool) -> Vec<PipelineTasks> {
    vec![
        PipelineTasks { 
            pipeline_id: name.to_string(), 
            task_id: name.to_string(),
            task_name: name.to_string(),
            status: (if is_building { "RUNNING" } else { "FINISHED" } ).to_string(),
            attempt_status: "1".to_string() }
    ]
}

#[cfg(test)]
fn pipeline_stats_test_aws_fields_builder(name: &str, is_healthy: bool) -> HashMap<String, String> {
    rusty_toolbox::hashmap![
        "@id".to_string() => name.to_string(), 
        "name".to_string() => name.to_string(), 
        "@healthStatus".to_string() => (if is_healthy { "HEALTHY" } else { "ERROR" }).to_string()
    ]
}