mod init;

use clap::App;
use std::fs::File;
use std::io::prelude::*;
use reploy_client::GlobalConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read config file
    let home_dir = std::env::var("HOME").expect("You need to have a HOME directory!");
    let config_file = File::open(home_dir.clone() + "/.config/reploy/config.json");
    let mut contents = String::new();
    if let Err(_) = config_file {
        std::fs::create_dir_all(home_dir.clone() + "/.config/reploy/")?;
        let mut config_file = File::create(home_dir + "/.config/reploy/config.json")?;
        config_file.write_all(b"{\"servers\":{}}");
        contents = "{\"servers\":{}}".to_string();
    } else if let Ok(mut file) = config_file {
        file.read_to_string(&mut contents)?;
    }
    let global_config: GlobalConfig = serde_json::from_str(&contents)?;

    let matches = App::new("reploy-client")
        .version("0.1.0")
        .author("Maya Farber Brodsky <maya@farberbrodsky.com>, Misha Farber Brodsky <misha@farberbrodsky.com>")
        .about("Reploy CLI")
        .subcommand(App::new("init")
            .about("Initialize Reploy project")
            .arg("-f, --force 'Force init if config file already exists'"))
        .get_matches();

    match matches.subcommand() {
        ("init", Some(sub_m)) => {
            init::run(global_config, sub_m);
        }
        _ => {
            println!("Unknown command or no command used, use --help for more info on available commands");
        }
    }
    // let (rx, mut write) = message_protocol::connect_to_tcp("127.0.0.1:37255")?;
    // write.call(b"hi server").unwrap();
    // while let Ok(msg) = rx.recv() {
    //     match msg {
    //         message_protocol::Message::Open => println!("open"),
    //         message_protocol::Message::Bytes(b) => println!("bytes {:?}", b),
    //         message_protocol::Message::Close => println!("close"),
    //     };
    // }
    Ok(())
}
