use std::io::Result;
use std::io::ErrorKind::NotFound;
use std::io::Write;

use toml::Table;
use toml::Value;

static CONFIG_FILE: &str = "config_env";

// ArceOS legal features
static FEATURES: [&str;7] = [
    "multitask", "irq", "fs", "net", "use_ramfs", "sched_rr", "sched_cfs",
];

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
    let args: Vec<_> = std::env::args().collect();
    let content = std::fs::read_to_string(&args[1])?;
    let table = content.parse::<Table>().expect("bad app toml");
    let features = get_features(&table).ok_or(NotFound)?;

    let mut config = std::fs::File::create(CONFIG_FILE)?;

    // Default features
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

    // Extra features
    if args.len() > 2 {
        for item in args[2].split_whitespace() {
            match item {
                "use_ramfs" => {
                    writeln!(config, "FS_TYPE := ramfs")?;
                },
                "sched_rr" => {
                    writeln!(config, "SCHED_POLICY := sched_rr")?;
                },
                "sched_cfs" => {
                    writeln!(config, "SCHED_POLICY := sched_cfs")?;
                },
                _ => {
                },
            }
        }
    }

    Ok(())
}

fn get_features(table: &Table) -> Option<&toml::Value> {
    table.get("package")?.get("metadata")?.get("arceos")?.get("features")
}
