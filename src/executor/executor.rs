use crate::yaml_parser::yaml_parser::sanitize_yaml;
use rayon::prelude::*;
use std::process::Command; // to execute shell commands
use std::{thread, time};
use rand::Rng;

// execute the commands/alerts based on yaml
// non deterministic order to leverage multithreading
pub fn execute() {
    let yaml_list = sanitize_yaml();

    let hash:i16 = rand::thread_rng().gen();
    let marker:String = hash.to_string();

    yaml_list.par_iter().for_each(|yaml_item| {

        let mut alerts_hit:i64 = 0;
        

        let name = yaml_item["name"].as_str().unwrap();
        let interval = yaml_item["interval"].as_i64().unwrap();
        let log_dirs = yaml_item["log_dir"].as_vec().unwrap();
        let alert_threshold = yaml_item["alert_threshold"].as_i64().unwrap();
        let alert_action = yaml_item["alert_action"].as_vec().unwrap();
        let alert_regex = yaml_item["alert_regex"].as_vec().unwrap();
        let help = yaml_item["help"].as_vec().unwrap();

        let mut log_dir_str: Vec<&str> = vec![];
        let mut alert_regex_str: Vec<&str> = vec![];

        for log_dir in log_dirs {
            log_dir_str.push(log_dir.as_str().unwrap());
        }
        for ind_alerg_regex in alert_regex {
            alert_regex_str.push(ind_alerg_regex.as_str().unwrap());
        }
        loop {
            if find_from_logs(log_dir_str.clone(), marker.clone(), alert_regex_str.clone()) {
                alerts_hit += 1;
            }

            if alerts_hit > alert_threshold {
                for action in alert_action {
                    let action_str = action.as_str().unwrap();
                    execute_alert(action_str);
                    if cfg!(debug_assertions) {
                        println!("name: {:#?}, interval: {:#?}, log_dirs: {:#?}, alert_threshold {:#?}, help {:#?}"
                        , name, interval, log_dirs, alert_threshold, help);
                    }
                }
                break; // exit out of loop since we're done
            }
            thread::sleep(time::Duration::from_millis(1000 * interval as u64 )); // seconds to milliseconds
        }

    });
}

// execute action upon hitting alert threshold
pub fn execute_alert(shell_cmd: &str) {
    let output = Command::new("sh")
        .arg("-c")
        .arg(shell_cmd)
        .output()
        .expect("failed to execute process");

    if cfg!(debug_assertions) {


        println!("STDOUT = {:?}. STERR = {:?}.", 
            String::from_utf8(output.stdout).expect("invalid utf8"),
            String::from_utf8(output.stderr).expect("invalid utf8"));
    }

}

pub fn find_from_logs(log_dir: Vec<&str>, marker: String, alert_regex:Vec<&str>) -> bool {

    let mut log_dir_str:String = "".to_string();
    let mut alert_regex_str:String = "".to_string();
    for log in log_dir {
        let mut environment:String = "".to_string();
        if cfg!(debug_assertions) {
            let cwd = std::env::current_dir()
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap();
            environment = format!("{}/test",cwd);
        }

        log_dir_str += format!("{}{}/*",environment,log).as_str();
        log_dir_str += &" ".to_string();
    }
    for alert_reg in alert_regex {
        alert_regex_str += &" -e ".to_string();
        alert_regex_str += alert_reg;
    }

    // alert_regex_str += &marker;
    alert_regex_str += &" ".to_string();
    

    let zgrep_cmd = format!("zgrep {} {}", alert_regex_str,log_dir_str);
    println!("{:?}", &zgrep_cmd);
    let output = Command::new("sh")
        .arg("-c")
        .arg(zgrep_cmd)
        .output()
        .expect("failed to execute process");

    let output_empty:bool = !output.stdout.is_empty();
    if cfg!(debug_assertions) {
        println!("STDOUT = {:?}. STERR = {:?}. MARKER={:?}", 
            String::from_utf8(output.stdout).expect("invalid utf8"),
            String::from_utf8(output.stderr).expect("invalid utf8"),
            marker);
    }

    output_empty
}
