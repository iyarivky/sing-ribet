use clap::Parser;
use std::fs;
use anyhow::Result;

/// Struktur untuk menangani argumen CLI
#[derive(Parser)]
#[command(author = "Iya Rivvikyn", version = "0.1.0", about = "sing-rivert \nv2ray to sing-box config converter", long_about = None)]
struct Cli {
    /// File input
    #[clap(short, long)]
    input: String,

    /// File output
    #[clap(short, long)]
    output: Option<String>,

    #[clap(short, long)]
    config_type: String
}

fn main() -> Result<()> {
    // Parsing argumen CLI
    let cli = Cli::parse();
    let input_file = cli.input;
    let output_file = match cli.output {
        Some(name) => name,
        None => format!("output_{}", input_file),
    };
    let content = fs::read_to_string(&input_file)?;

    let modified_content = content.lines()
        .map(|line| format!("{} ceklis", line))
        .collect::<Vec<String>>()
        .join("\n");

    fs::write(&output_file, modified_content)?;

    Ok(())
}