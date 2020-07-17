pub mod properties;
pub mod models;
pub mod bitbar;

use structopt::StructOpt;
use aws_connections_lib::datapipelines::get_datapipeline_client;
use aws_services_lib::datapipelines::datapipelines::status;
use aws_services_lib::datapipelines::models::pipeline::Pipeline;
use models::pipeline_health_visibility::PipelineHealthVisibility;
use properties::get_props;
use models::pipelines_report::PipelinesReport;
use bitbar as bb;


#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Opt {
    #[structopt(short = "f", long = "format", default_value = "json", possible_values = &vec![ "json", "bitbar" ])]
    format: String,
    #[structopt(short = "p", long = "properties", default_value = "")]
    properties: String,
    #[structopt(short = "a", long = "aws-profile", required = true)]
    aws_profile: String,
    #[structopt(short = "s", long = "show", default_value = "all", possible_values = &PipelineHealthVisibility::variants())]
    show: String,
    #[structopt(short = "n", long = "name", default_value = "")]
    name: String,
}

fn filter_pipelines_by_health(pipelines: Vec<Pipeline>, show: &PipelineHealthVisibility) -> Vec<Pipeline> {
    pipelines.iter()
        .filter(|p| {
            match show {
                PipelineHealthVisibility::ShowHealthy => p.is_healthy(),
                PipelineHealthVisibility::ShowError => !p.is_healthy(),
                _ => true,
            }
        })
        .map(|m| m.clone())
        .collect()
}


fn main() {
    let opt = Opt::from_args();
    let props = get_props(&opt.properties);
    let title = if opt.name.is_empty() { opt.aws_profile.clone() } else { opt.name };
    let show = PipelineHealthVisibility::from_string(&opt.show);
    match get_datapipeline_client(&opt.aws_profile, "us-east-1") {
        Some(client) => {
            let pipeline_name_filters = &props.filter_names;
            let pipelines = filter_pipelines_by_health(status(&client, pipeline_name_filters, &props.filter_operation), &show);
            let pipelines_report = PipelinesReport::compile(
                &title, 
                &pipelines
            );
            let output = match opt.format.as_ref() {
                "bitbar" => bb::format_display(&pipelines_report),
                _ => serde_json::to_string(&pipelines_report).unwrap(),
            };
            println!("{}", output)
        },
        None => println!("unable to establish credentials"),
    };
}
