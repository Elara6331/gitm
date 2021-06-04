/* Gitm. Automatic git mirroring script.
 *
 * Copyright (C) 2021 Arsen Musayelyan <arsen@arsenm.dev>
 *
 * This program is free software: you can redistribute it and/or modify it under
 * the terms of the GNU General Public License as published by the Free Software
 * Foundation, either version 3 of the License, or (at your option) any later
 * version.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with
 * this program. If not, see <https://www.gnu.org/licenses/>.
 */

use serde_derive::Deserialize;
use std::fs;
use std::{
    collections::HashMap,
    env,
    process::{exit, Child, Command},
};
use toml;

#[macro_use]
extern crate log;

// TOML config filename
const CFG_NAME: &str = ".gitm.toml";

// Config struct stores decoded TOML from config
#[derive(Deserialize, Default)]
struct Config {
    repos: HashMap<String, String>,
    options: Option<Options>,
}

// Options struct stores the options table in the TOML config
#[derive(Deserialize, Default)]
struct Options {
    branch: Option<String>,
}

fn main() {
    // Create new logger with level Info and no timestamp
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp(None)
        .init();

    // Get config contents
    let cfg_contents = fs::read_to_string(CFG_NAME).log_err("Error reading file");

    // Decode config contents
    let config: Config = toml::from_str(&cfg_contents).unwrap_or_default();

    // If no repos provided, error and exit
    if config.repos.len() < 1 {
        error!("Please add repos to the {} file", CFG_NAME);
        exit(1);
    }

    // If origin repo is not defined, error and exit
    if config.repos.get("origin").is_none() {
        error!("Origin repo required in {} file", CFG_NAME);
        exit(1);
    }

    // Collect arguments into vector
    let args: Vec<String> = env::args().collect();

    // If no arguments provided
    if args.len() < 2 {
        // Run git --help
        let mut proc = Command::new("git")
            .arg("--help")
            .spawn()
            .log_err("Error running git command");
        exit_if_code_nonzero(&mut proc);
    }

    // Ensure options exists in config, otherwise set to default
    let options = config.options.unwrap_or_default();
    // Ensure branch exists in options, otherwise set to "master"
    let branch = options.branch.unwrap_or("master".to_string());
    match args[1].as_str() {
        "push" => {
            info!("Intercepted push command");
            // For every repo in config
            for (name, _) in config.repos {
                // Run git push with applicable arguments
                let mut proc = Command::new("git")
                    .args(&["push", &name, &branch])
                    .args(&args[2..])
                    .spawn()
                    .log_err("Error running git command");
                exit_if_code_nonzero(&mut proc);
            }
        }
        "init" => {
            info!("Intercepted init command");
            // Run git init with any preceding arguments
            let mut proc = Command::new("git")
                .arg("init")
                .args(&args[2..])
                .spawn()
                .log_err("Error running git command");
            exit_if_code_nonzero(&mut proc);
            // For every repo in config
            for (name, repo) in config.repos {
                // Run git remote add with name and repository URL
                let mut proc = Command::new("git")
                    .args(&["remote", "add", &name, &repo])
                    .spawn()
                    .log_err("Error running git command");
                exit_if_code_nonzero(&mut proc);
            }
            // Run git fetch origin
            proc = Command::new("git")
                .args(&["fetch", "origin"])
                .spawn()
                .log_err("Error running git command");
            exit_if_code_nonzero(&mut proc);
            // Run git checkout master
            proc = Command::new("git")
                .args(&["checkout", "master"])
                .spawn()
                .log_err("Error running git command");
            exit_if_code_nonzero(&mut proc);
        }
        // Default
        _ => {
            // Run git, passing through all arguments provided
            let mut proc = Command::new("git")
                .args(&args[1..])
                .spawn()
                .log_err("Error running git command");
            exit_if_code_nonzero(&mut proc);
        }
    }
}

fn exit_if_code_nonzero(proc: &mut Child) {
    // Wait for process and get exit status
    let status = proc.wait().log_err("Command was not running");
    // Get exit code, default 0
    let exit_code = status.code().unwrap_or(0);
    // If nonzero exit code
    if exit_code != 0 {
        // Exit with the same exit code as process
        exit(exit_code)
    }
}

// Trait LogErr allows user-friendly error messages
trait LogErr<T> {
    fn log_err(self, msg: &str) -> T;
}

// Implement LogErr on Result of any type where E implements Debug
impl<T, E> LogErr<T> for Result<T, E>
where
    E: ::std::fmt::Debug,
{
    // log_err unwraps Result, logging error and exiting if needed
    fn log_err(self, msg: &str) -> T {
        match self {
            // If no error
            Ok(res) => {
                // Return value within Result
                return res;
            }
            // If error
            Err(err) => {
                // Log error in format "message: error" and exit
                error!("{}: {:?}", msg, err);
                exit(1)
            }
        }
    }
}
