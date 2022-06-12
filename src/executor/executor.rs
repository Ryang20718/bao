use crate::yaml_parser::yaml_parser::sanitize_yaml;
use rayon::prelude::*;

// execute the commands/alerts based on yaml
// non deterministic order to leverage multithreading
pub fn execute() {
    let yaml_list = sanitize_yaml();
    yaml_list.par_iter()
         .for_each(|yaml_item| {
            let name = yaml_item["name"].as_str().unwrap();
            let interval = yaml_item["interval"].as_i64().unwrap();
            let log_dir = yaml_item["log_dir"].as_vec().unwrap();
            let alert_threshold = yaml_item["alert_threshold"].as_i64().unwrap();
            let help = yaml_item["help"].as_vec().unwrap();
            println!("{:?}", yaml_item)
         });

    // for yaml_item in &yaml_list {
    //     let name = yaml_item["name"].as_str().unwrap();
    //     let interval = yaml_item["interval"].as_i64().unwrap();
    //     let log_dir = yaml_item["log_dir"].as_vec().unwrap();
    //     let alert_threshold = yaml_item["alert_threshold"].as_i64().unwrap();
    //     let help = yaml_item["help"].as_vec().unwrap();
        
    //     // Required Fields for yaml
    //     assert_eq!(name.is_empty(), false);
    //     assert_eq!(interval > 0, true);
    //     assert_eq!(log_dir.len() > 0, true);
    //     assert_eq!(alert_threshold > 0, true);
    //     assert_eq!(help.len() > 0, true);
    // }
}