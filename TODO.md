<!-- file: TODO.md -->
<!-- version: 0.5.0 -->
<!-- guid: 12345678-90ab-cdef-1234-567890abcdef -->

# TODO

## Completed

- [x] Basic transcode command with metadata preservation
- [x] Git LFS setup for test media
- [x] Batch processing for recursive directory conversion
- [x] Dry-run mode for batch operations
- [x] GitHub Actions CI for lint, build, and basic smoke test
- [x] Add presets (original-h265, tv-h265-fast, movie-quality)
- [x] Comprehensive integration test suite with test media
- [x] Benchmark suite using Criterion
- [x] Test utilities and helpers (common module)
- [x] Testing documentation (TESTING.md)

## In Progress

- [ ] Test CLI functionality (run with actual test files)
- [ ] Progress reporting with ETA during batch conversion
- [ ] Resume capability for interrupted batches (skip already converted files)

## Planned

### High Priority

- [ ] Embed static ffmpeg build from <https://github.com/jdfalk/FFmpeg-Builds>
- [ ] Add preset (web-optimized)
- [ ] Verify metadata preservation in tests (compare input vs output metadata)
- [ ] Add quality validation tests (ensure transcoded files play correctly)

### Medium Priority

- [ ] Extend metadata preservation options (cover art, chapters)
- [ ] Hardware acceleration support (VAAPI, NVENC, VideoToolbox)
- [ ] Parallel processing for batch operations
- [ ] Quality comparison reports (original vs. transcoded file sizes)
- [ ] Add code coverage reporting (tarpaulin)

### Low Priority

- [ ] Property-based testing with proptest
- [ ] Mutation testing with cargo-mutants
- [ ] Fuzz testing for CLI parsing
- [ ] Performance regression detection in CI
