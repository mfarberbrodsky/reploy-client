use reploy_client::{GlobalConfig, ProjectConfig};
use std::fs::File;
use clap::ArgMatches;
use std::io::prelude::*;

pub fn run(global_config: GlobalConfig, args: &ArgMatches) {
    let config_file = File::open(".reploy.json");

    if let Ok(_) = config_file {
        if !args.is_present("f") {
            eprintln!("Config file .reploy.json already exists, use --force to override it.");
            return;
        }
    }

    let mut config_file = File::create(".reploy.json").expect("Can't create .reploy.json file.");
    let project_config = ProjectConfig {
        uuid: uuid::Uuid::new_v4(),
        default_servers: vec![],
    };
    config_file.write_all(serde_json::to_string(&project_config).unwrap().as_bytes()).unwrap();

    let mut runfile = File::create("reploy.sh").expect("Can't create reploy.sh file.");
    runfile.write_all(b"# This file was automatically generated using Reploy.\n# It contains the code the server runs at each deployment.\necho \"Deployed! Change reploy.sh.\"").unwrap();
    println!("Success! Please edit reploy.sh, and then run reploy to deploy your project.");
}