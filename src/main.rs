// file: src/main.rs
// version: 0.1.0
// guid: 0f9e8d7c-6b5a-4c3d-2e1f-0a9b8c7d6e5f

use std::process::{Command, Stdio};

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about = "Transcode media while preserving metadata (ffmpeg wrapper)", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Show media info via ffprobe (optionally as JSON)
    Info {
        /// Input media file
        input: String,
        /// Output as JSON (requires --features json)
        #[arg(long)]
        json: bool,
    },
    /// Transcode a file while preserving metadata
    Transcode {
        /// Input media file
        input: String,
        /// Output media file
        output: String,
        /// Video codec (e.g., libx264, libx265, copy)
        #[arg(long, default_value = "libx264")]
        vcodec: String,
        /// Audio codec (e.g., aac, libopus, copy)
        #[arg(long, default_value = "aac")]
        acodec: String,
        /// Extra ffmpeg args (passed as-is after standard args)
        #[arg(long, num_args = 0.., value_delimiter = ' ')]
        extra: Vec<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Info { input, json } => info(&input, json),
        Commands::Transcode {
            input,
            output,
            vcodec,
            acodec,
            extra,
        } => transcode(&input, &output, &vcodec, &acodec, &extra),
    }
}

fn info(input: &str, json: bool) -> Result<()> {
    let mut cmd = Command::new("ffprobe");
    if json {
        cmd.args(["-v", "error", "-print_format", "json", "-show_format", "-show_streams", input]);
    } else {
        cmd.args(["-hide_banner", "-i", input]);
    }

    let status = cmd
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .with_context(|| "failed to spawn ffprobe")?;

    if !status.success() {
        bail!("ffprobe exited with status: {:?}", status.code());
    }
    Ok(())
}

fn transcode(input: &str, output: &str, vcodec: &str, acodec: &str, extra: &[String]) -> Result<()> {
    // Build a conservative default arg list that tries to preserve metadata
    // -map_metadata 0 copies global metadata
    // -movflags use_metadata_tags preserves tags in MP4 containers
    // -c:s copy keeps subtitle streams
    let mut args = vec![
        "-hide_banner".to_string(),
        "-y".to_string(), // overwrite
        "-i".to_string(),
        input.to_string(),
        "-map_metadata".to_string(),
        "0".to_string(),
        "-movflags".to_string(),
        "use_metadata_tags".to_string(),
        "-c:v".to_string(),
        vcodec.to_string(),
        "-c:a".to_string(),
        acodec.to_string(),
        "-c:s".to_string(),
        "copy".to_string(),
    ];

    // Append any extra args the user provided
    args.extend(extra.iter().cloned());

    // Output path last
    args.push(output.to_string());

    let status = Command::new("ffmpeg")
        .args(&args)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .with_context(|| format!("failed to spawn ffmpeg; args: {:?}", &args))?;

    if !status.success() {
        bail!("ffmpeg exited with status: {:?}", status.code());
    }
    Ok(())
}
