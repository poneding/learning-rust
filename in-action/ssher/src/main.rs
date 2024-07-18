//! # ssher
//!
//! `ssher` 一个简易的连接远程服务器命令行工具，使用 Rust 语言实现。
//!
//! ## 安装
//!
//! ```bash
//! cargo install ssher
//! ```
//!
//! ## 使用
//!
//! 首先，配置 `profiles.json` 文件，参考格式：
//!
//! ```json
//! [
//!   {
//!     "name": "cloud-server",
//!     "host": "10.1.1.10",
//!     "user": "ubuntu",
//!     "identity_file": "~/.ssh/id_rsa"
//!   }
//! ]
//! ```
//!
//! 使用 `ssher` 命令行工具选择服务器并连接：
//!
//! ```bash
//! ssher
//! ```

use dialoguer::Select;
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use structopt::StructOpt;

#[derive(Serialize, Deserialize, Debug)]
struct SSHProfile {
    name: String,
    host: String,
    user: String,
    identity_file: String,
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
        identity_file: String,
    },
}

fn main() {
    let args = Cli::from_args();

    match args.command {
        Some(CommandType::Save {
            name,
            host,
            user,
            identity_file,
        }) => {
            let profile = SSHProfile {
                name,
                host,
                user,
                identity_file,
            };

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
                .arg("-i")
                .arg(OsStr::new(&profile.identity_file))
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .expect("SSH command failed to start")
                .wait()
                .expect("Failed to wait on child process.");
        }
    }
}
