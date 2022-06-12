extern crate yaml_rust;
use std::fs::{read_dir, File};
use std::io::prelude::*;
use yaml_rust::{Yaml, YamlLoader};

pub fn load_file(file: &str) -> Vec<Yaml> {
    let mut file = File::open(file).expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let docs = YamlLoader::load_from_str(&contents).unwrap();

    return docs;
}

pub fn sanitize_yaml() -> Vec<Yaml> {
    let cwd = std::env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();

    let mut environment = "prod";
    if cfg!(debug_assertions) {
        environment = "test";
    }
    let yaml_dir = read_dir(format!("{}/{}/yaml_dirs", cwd, environment)).unwrap();

    let yaml_list: Vec<Yaml> = yaml_dir
        .map(|yaml_path| load_file(yaml_path.unwrap().path().to_str().unwrap()))
        .flatten()
        .collect();

    if cfg!(debug_assertions) {
        println!("Length of yaml {:#?}", yaml_list.len());
    }
    for yaml_item in &yaml_list {
        let name = yaml_item["name"].as_str().unwrap();
        let interval = yaml_item["interval"].as_i64().unwrap();
        let log_dir = yaml_item["log_dir"].as_vec().unwrap();
        let alert_action = yaml_item["alert_action"].as_vec().unwrap();
        let alert_threshold = yaml_item["alert_threshold"].as_i64().unwrap();
        let alert_regex = yaml_item["alert_regex"].as_vec().unwrap();
        let help = yaml_item["help"].as_vec().unwrap();

        // Required Fields for yaml
        assert_eq!(name.is_empty(), false);
        assert_eq!(interval > 0, true);
        assert_eq!(log_dir.len() > 0, true);
        assert_eq!(!alert_action.is_empty(), true);
        assert_eq!(!alert_regex.is_empty(), true);
        assert_eq!(alert_threshold > 0, true);
        assert_eq!(help.len() > 0, true);
    }
    return yaml_list;
}
