use dialoguer::Select;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;
use structopt::StructOpt;

#[derive(Serialize, Deserialize, Debug)]
struct SSHProfile {
    name: String,
    host: String,
    user: String,
}

#[derive(StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    command: Option<CommandType>,
}

#[derive(StructOpt)]
enum CommandType {
    Save {
        name: String,
        host: String,
        user: String,
    },
}

fn main() {
    let args = Cli::from_args();

    match args.command {
        Some(CommandType::Save { name, host, user }) => {
            let profile = SSHProfile { name, host, user };

            let mut profiles = if Path::new("profiles.json").exists() {
                let file = File::open("profiles.json").unwrap();
                let reader = std::io::BufReader::new(file);
                serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new())
            } else {
                Vec::new()
            };

            profiles.push(profile);

            let serialized = serde_json::to_string(&profiles).unwrap();
            let mut file = File::create("profiles.json").unwrap();
            file.write_all(serialized.as_bytes()).unwrap();
        }
        None => {
            let file = File::open("profiles.json").unwrap();
            let reader = std::io::BufReader::new(file);
            let profiles: Vec<SSHProfile> = serde_json::from_reader(reader).unwrap();

            let selection = Select::new()
                .with_prompt("Which profile do you want to use?")
                .items(
                    &profiles
                        .iter()
                        .map(|p| p.name.clone())
                        .collect::<Vec<String>>(),
                )
                .default(0)
                .interact()
                .unwrap();

            let profile = &profiles[selection];
            Command::new("ssh")
                .arg(format!("{}@{}", profile.user, profile.host))
                .spawn()
                .expect("SSH command failed to start");
        }
    }
}
