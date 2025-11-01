<!-- file: TODO.md -->
<!-- version: 0.4.0 -->
<!-- guid: 12345678-90ab-cdef-1234-567890abcdef -->

# TODO

## Completed

- [x] Basic transcode command with metadata preservation
- [x] Git LFS setup for test media
- [x] Batch processing for recursive directory conversion
- [x] Dry-run mode for batch operations
- [x] GitHub Actions CI for lint, build, and basic smoke test

## In Progress

- [x] Add presets (tv-h265-fast, movie-quality)
- [ ] Add preset (web-optimized)
- [ ] Progress reporting with ETA during batch conversion
- [ ] Resume capability for interrupted batches (skip already converted files)

## Planned

- [ ] Embed static ffmpeg build from <https://github.com/jdfalk/FFmpeg-Builds>
- [ ] Implement unit tests for CLI argument parsing
- [ ] Add integration tests with sample media files (skippable in CI)
- [ ] Extend metadata preservation options (cover art, chapters)
- [ ] Hardware acceleration support (VAAPI, NVENC, VideoToolbox)
- [ ] Parallel processing for batch operations
- [ ] Quality comparison reports (original vs. transcoded file sizes)
