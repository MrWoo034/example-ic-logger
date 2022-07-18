// build.rs
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;

const PROD: &str = "Prod";
const STAGING: &str = "Staging";
const LOCAL: &str = "Local";

#[derive(Serialize, Deserialize, Debug)]
pub enum Profile {
    Prod(Config),
    Staging(Config),
    Local(Config),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    service: String,
    logger: String,
}

fn main() {
    // Read deployment / build stage from .env
    dotenv::dotenv().ok();
    let env = std::env::var("ENV").unwrap_or("Failed to find ENV var".to_string());

    // Setup the build path for generated rust file
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("env.rs");

    // read the config.json and generate the profiles.
    let config_json = fs::read_to_string(Path::new("./config.json")).unwrap();
    let profiles: Vec<Profile> = serde_json::from_str(&config_json).unwrap();

    let mut config: Config = Config {
        service: "".to_string(),
        logger: "".to_string(),
    };

    for profile in profiles {
        match profile {
            Profile::Prod(_config) => {
                if env.as_str() == PROD {
                    config = _config;
                    break;
                }
            }
            Profile::Staging(_config) => {
                if env.as_str() == STAGING {
                    config = _config;
                    break;
                }
            }
            Profile::Local(_config) => {
                if env.as_str() == LOCAL {
                    config = _config;
                    break;
                }
            }
        }
    }

    let mut file_str: String = String::new();
    file_str.push_str(PRELUDE);
    file_str.push_str(&format!("const CONFIG: Config = {:?};", config));
    std::println!("file_str: {:?}", file_str);
    fs::write(&dest_path, &file_str).unwrap();

    // The rerun-if-changed instruction tells Cargo that the build script only needs to re-run if the build script itself changes.
    // Without this line, Cargo will automatically run the build script if any file in the package changes.
    // If your code generation uses some input files, this is where you would print a list of each of those files.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=config.json");
    println!("cargo:rerun-if-changed=.env");
}

const PRELUDE: &str = r#"
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config<'a> {
    service: &'a str,
    logger: &'a str,
}
"#;
