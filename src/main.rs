// file: src/main.rs
// version: 0.2.0
// guid: 0f9e8d7c-6b5a-4c3d-2e1f-0a9b8c7d6e5f

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};
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
    /// Batch transcode a directory recursively (default: h265+aac)
    Batch {
        /// Input directory to scan recursively
        input_dir: String,
        /// Output directory (mirrors input structure)
        output_dir: String,
        /// Video codec (e.g., libx265)
        #[arg(long, default_value = "libx265")]
        vcodec: String,
        /// Audio codec (e.g., aac, libopus)
        #[arg(long, default_value = "aac")]
        acodec: String,
        /// Output file extension (e.g., mkv, mp4)
        #[arg(long, default_value = "mkv")]
        ext: String,
        /// File extensions to process (comma-separated)
        #[arg(long, default_value = "mp4,mkv,avi,mov,m4v,ts")]
        input_exts: String,
        /// Extra ffmpeg args (passed as-is after standard args)
        #[arg(long, num_args = 0.., value_delimiter = ' ')]
        extra: Vec<String>,
        /// Dry run: print commands without executing
        #[arg(long)]
        dry_run: bool,
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
        Commands::Batch {
            input_dir,
            output_dir,
            vcodec,
            acodec,
            ext,
            input_exts,
            extra,
            dry_run,
        } => batch_transcode(
            &input_dir,
            &output_dir,
            &vcodec,
            &acodec,
            &ext,
            &input_exts,
            &extra,
            dry_run,
        ),
    }
}

fn info(input: &str, json: bool) -> Result<()> {
    let mut cmd = Command::new("ffprobe");
    if json {
        cmd.args([
            "-v",
            "error",
            "-print_format",
            "json",
            "-show_format",
            "-show_streams",
            input,
        ]);
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

fn transcode(
    input: &str,
    output: &str,
    vcodec: &str,
    acodec: &str,
    extra: &[String],
) -> Result<()> {
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

fn batch_transcode(
    input_dir: &str,
    output_dir: &str,
    vcodec: &str,
    acodec: &str,
    ext: &str,
    input_exts: &str,
    extra: &[String],
    dry_run: bool,
) -> Result<()> {
    let input_path = Path::new(input_dir);
    let output_path = Path::new(output_dir);

    if !input_path.exists() {
        bail!("Input directory does not exist: {}", input_dir);
    }

    // Parse comma-separated extensions
    let exts: Vec<&str> = input_exts.split(',').map(|s| s.trim()).collect();

    // Collect all media files recursively
    let files = collect_media_files(input_path, &exts)?;

    if files.is_empty() {
        println!("No media files found matching extensions: {}", input_exts);
        return Ok(());
    }

    println!(
        "Found {} files to transcode (vcodec={}, acodec={}, ext={})",
        files.len(),
        vcodec,
        acodec,
        ext
    );

    for (idx, input_file) in files.iter().enumerate() {
        // Calculate relative path and mirror structure
        let rel_path = input_file
            .strip_prefix(input_path)
            .context("failed to strip prefix")?;

        let mut output_file = output_path.join(rel_path);
        output_file.set_extension(ext);

        // Ensure output directory exists
        if let Some(parent) = output_file.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create output dir: {:?}", parent))?;
        }

        println!(
            "\n[{}/{}] {} -> {}",
            idx + 1,
            files.len(),
            input_file.display(),
            output_file.display()
        );

        if dry_run {
            println!(
                "  [DRY RUN] Would transcode with vcodec={} acodec={}",
                vcodec, acodec
            );
            continue;
        }

        // Perform the transcode
        if let Err(e) = transcode(
            &input_file.to_string_lossy(),
            &output_file.to_string_lossy(),
            vcodec,
            acodec,
            extra,
        ) {
            eprintln!("  ERROR: {}", e);
            eprintln!("  Skipping and continuing with next file...");
        }
    }

    println!("\nBatch transcode completed!");
    Ok(())
}

fn collect_media_files(dir: &Path, extensions: &[&str]) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Recurse into subdirectories
            files.extend(collect_media_files(&path, extensions)?);
        } else if path.is_file() {
            if let Some(file_ext) = path.extension() {
                let file_ext_str = file_ext.to_string_lossy().to_lowercase();
                if extensions.iter().any(|e| e.to_lowercase() == file_ext_str) {
                    files.push(path);
                }
            }
        }
    }

    Ok(files)
}
