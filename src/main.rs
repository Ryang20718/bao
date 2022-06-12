mod executor;
mod yaml_parser;

use crate::executor::executor::execute;

fn main() {
    execute();
    // assert_eq!(doc["health"]["interval"][0].as_str().unwrap(), "30");
}
