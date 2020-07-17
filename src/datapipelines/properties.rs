use ::serde_derive::{Deserialize, Serialize};
use std::io::BufReader;
use std::fs::File;
use serde_json::Error;
use std::path::PathBuf;

#[derive(Debug,Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MonitoringProps {
    pub filter_operation: String,
    pub filter_names: Vec<String>,
}

impl MonitoringProps {
    fn default() -> MonitoringProps {
        MonitoringProps {
            filter_operation: "exclude".to_string(),
            filter_names: vec![],
        }
    }
}

pub fn get_props(properties_file_location: &str) -> MonitoringProps {
    if properties_file_location.is_empty() {
        MonitoringProps::default()
    } else {
        let maybe_file = File::open(PathBuf::from(properties_file_location));
        match maybe_file {
            Ok(file) => {
                let reader = BufReader::new(file);
                let maybe_props: Result<MonitoringProps, Error> = serde_json::from_reader(reader);
                match maybe_props {
                    Ok(props) => props,
                    Err(_e) => MonitoringProps::default(),
                }
            }
            Err(_e) => {
                println!("Bad file passed");
                MonitoringProps::default()
            },
        }
    }
}

#[test]
fn properties_should_be_read_from_file() {
    let props = get_props("tests/datapipelines/properties-test.json");
    
    assert_eq!("exclude", props.filter_operation);
    println!("filter names: {:?}", &props.filter_names);
    assert!(&props.filter_names.contains(&"An Unimportant Pipeline".to_string()));
    assert!(&props.filter_names.contains(&"Another Unimportant Pipeline".to_string()));
}

#[test]
fn properties_should_be_read_from_different_file() {
    let props = get_props("tests/datapipelines/properties-test2.json");
    
    assert_eq!("include", props.filter_operation);
    assert!(&props.filter_names.contains(&"Monitor This Pipeline".to_string()));
    assert!(&props.filter_names.contains(&"As Well As This Pipeline".to_string()));
}

#[test]
fn empty_properties_filename_uses_default_properties() {
    let props = get_props("");

    assert!(props.filter_names.is_empty());
    assert_eq!("exclude", props.filter_operation);
}

#[test]
fn invalid_properties_filename_uses_default_properties() {
    let props = get_props("abcdefg/no-file-here.json");

    assert!(props.filter_names.is_empty());
    assert_eq!("exclude", props.filter_operation);
}

#[test]
fn badly_formatted_json_in_properties_file_uses_default_properties() {
    let props = get_props("tests/datapipelines/properties-test-not-real.json");
    
    assert!(props.filter_names.is_empty());
    assert_eq!("exclude", props.filter_operation);
}