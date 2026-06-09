# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Option to remove the aforementioned loading bar.

### Removed

- Linear downloader, and by extension the option to choose a downloading method.

## [0.2.0] - 2026-05-31

### Added

- Option to write to stdout instead of a file.

### Changed

- Rename argument `--filename` to `--output`.
- Write to stdout by default.

### Removed

- Print message describing download time.

## [0.1.1] - 2026-05-30

### Changed

- Use consistent sentence casing in help message.

## [0.1.0] - 2026-05-29

### Added

- Basic functionality
- Two downloading modes: *linear* and *streamed*.
- Option to rename the downloaded file.

[unreleased]: https://github.com/mattaroni/rufetch/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/mattaroni/rufetch/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/mattaroni/rufetch/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/mattaroni/rufetch/releases/tag/v0.1.0
