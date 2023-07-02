use std::io::Result;
use std::io::ErrorKind::{InvalidInput, NotFound};
use std::io::Write;

use toml::Table;
use toml::Value;

static CONFIG_FILE: &str = "config_env";

// ArceOS legal features
static FEATURES: [&str;2] = ["fs", "net"];

fn main() {
    if let Err(e) = parse() {
        panic!("Parse features for stdapp failed. Err: {:?}", e);
    }
    print!("{}", CONFIG_FILE);
}

fn has_feature(f: &str) -> bool {
    FEATURES.iter().find(|&x| x == &f).is_some()
}

fn parse() -> Result<()> {
    let mut args = std::env::args();
    let toml_file = args.nth(1).ok_or(InvalidInput)?;
    let content = std::fs::read_to_string(toml_file)?;
    let table = content.parse::<Table>().expect("bad app toml");
    let features = get_features(&table).ok_or(NotFound)?;

    let mut config = std::fs::File::create(CONFIG_FILE)?;

    // Default feature
    if let Some(Value::Array(default)) = features.get("default") {
        for f in default {
            if let Value::String(v) = f {
                if !has_feature(v) {
                    continue;
                }
                writeln!(config, "{} := y", v.to_uppercase())?;
            }
        }
    }

    Ok(())
}

fn get_features(table: &Table) -> Option<&toml::Value> {
    table.get("package")?.get("metadata")?.get("arceos")?.get("features")
}
