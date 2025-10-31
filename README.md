<!-- file: README.md -->
<!-- version: 0.2.0 -->
<!-- guid: 0a1b2c3d-4e5f-6789-abcd-ef0123456789 -->

# transcoderr

A small Rust CLI that wraps ffmpeg/ffprobe to transcode media while preserving metadata.

## Features

- `info`: show media info via ffprobe (optionally JSON)
- `transcode`: transcode while attempting to preserve metadata (map_metadata, movflags)
- Sensible defaults with override flags for codecs and extra args

## Requirements

- Rust (1.76+ recommended)
- ffmpeg and ffprobe available on PATH

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

# Transcode (change codecs, preserve metadata, keep subtitles)
cargo run -- transcode input.mp4 output.mp4 --vcodec libx265 --acodec libopus --extra -crf 28 -preset medium
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


Note: `testdata/` is gitignored.

## Roadmap

- Presets (h264/h265/aac/opus)
- Dry-run mode to print ffmpeg command
- Extended metadata (cover art, chapters)
- Integration tests (optional in CI)
