use clap::Parser;
use std::collections::HashSet;

#[derive(Parser)]
#[command(name = "sing_ribet", version = "1.0", author = "Your Name", about = "Checks if the input matches predefined config types", aliases = &["srb"])]
pub struct Cli {
    /// Comma-separated list of config types
    #[arg(short, long)]
    pub config_types: String,
}

pub fn validate_config_types(cli: &Cli) -> Result<(), String> {
    let input_set: HashSet<_> = cli.config_types.split(',').collect();
    let valid_types: HashSet<&str> = ["sfa", "bfm", "husi", "nekobox"].iter().copied().collect();

    if input_set.is_subset(&valid_types) {
        Ok(())
    } else {
        for &item in &input_set {
            if !valid_types.contains(item) {
                return Err(format!(
                    "Unknown config type: {}\nThe supported config types are sfa, bfm, husi, and nekobox",
                    item
                ));
            }
        }
        Ok(())
    }
}
