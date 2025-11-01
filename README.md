<!-- file: README.md -->
<!-- version: 0.3.0 -->
<!-- guid: 0a1b2c3d-4e5f-6789-abcd-ef0123456789 -->

# transcoderr

A Rust CLI that wraps ffmpeg/ffprobe to transcode media while preserving metadata. Designed for batch conversion of TV shows and movies to modern codecs (h265/opus/aac).

## Features

- `info`: show media info via ffprobe (optionally JSON)
- `transcode`: transcode while preserving metadata (map_metadata, movflags)
- `batch`: process entire directories recursively with h265 encoding
- Sensible defaults with override flags for codecs and extra args

## Requirements

- Rust (1.76+ recommended)
- ffmpeg and ffprobe available on PATH
- Git LFS (for cloning test media): `brew install git-lfs` or `apt install git-lfs`

## Install / Build

```bash
cargo build
```

## Usage

```bash
# Show help
cargo run -- --help

# Show media info (JSON)
cargo run -- info testdata/test_color_720p_h264_aac.mp4 --json

# Transcode single file (h265+opus, preserve metadata)
cargo run -- transcode input.mp4 output.mkv --vcodec libx265 --acodec libopus

# Batch convert TV show directory to h265+aac
cargo run -- batch /path/to/tv-shows /path/to/output --vcodec libx265 --acodec aac --ext mkv

# Advanced: custom CRF and preset
cargo run -- transcode input.mp4 output.mp4 --vcodec libx265 --acodec libopus --extra -crf 28 -preset medium
```

## Git LFS Setup

Test media files are tracked via Git LFS. After cloning:

```bash
git lfs install
git lfs pull
```

## Generate test media

Use the helper script (writes files to `testdata/`):

```bash
python3 scripts/generate_test_media.py
```

Generated examples:

- `test_color_720p_h264_aac.mp4` (3s, color pattern, metadata)
- `test_bars_480p_h265_opus.mkv` (3s, SMPTE bars)
- `test_audio_sine_opus.ogg` (3s, audio-only)
- `test_with_subs_h264_aac.mp4` (3s, embedded subtitles)


## Roadmap

- [x] Basic transcode with metadata preservation
- [x] Git LFS setup for test media
- [ ] Batch processing for directories
- [ ] Presets (tv-h265, movie-quality, fast-encode)
- [ ] Dry-run mode to preview ffmpeg commands
- [ ] Progress reporting and ETA
- [ ] Resume capability for interrupted batches
- [ ] Extended metadata (cover art, chapters)
- [ ] Integration tests (optional in CI)
