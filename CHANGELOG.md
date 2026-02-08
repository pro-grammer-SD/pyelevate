# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2024-01-XX

### Added

- Initial release of PyElevate
- Interactive TUI with Ratatui for dependency management
- Async PyPI version fetching with Tokio + Reqwest
- Smart version comparison and categorization (patch/minor/major)
- Fuzzy search and filtering capabilities
- Full keyboard navigation support
- Color-coded status indicators
- Automatic backup creation before upgrades
- Dry-run mode for safe previewing
- Lock file generation with `--lock` flag
- Support for package extras (e.g., `requests[security]`)
- Bulk selection controls (Select All/Major/Minor/Patch)
- Backup and safety features
- CLI interface with `check` and `upgrade` commands
- PyPI response caching for performance
- Version parsing for pinned, ranged, and compatible constraints
- Comprehensive error handling
- Unit tests for parsing and version comparison
- Full documentation and examples

### Features

#### Core Functionality
- Parse requirements.txt with support for various version specifications
- Fetch latest versions from PyPI asynchronously
- Compare versions using semantic versioning
- Display upgrade recommendations with severity levels

#### TUI Interface
- Modern interactive terminal interface with Ratatui
- Real-time package table with sortable columns
- Selection-based upgrade workflow
- Search and filter with fuzzy matching
- Color-coded status visualization
- Progress indicators and loading states

#### Safety & Reliability
- Automatic backup creation before modifications
- Dry-run mode for previewing changes
- Lock file generation for reproducible installs
- Error recovery and graceful degradation
- Network timeout handling (10-second default)

#### User Experience
- Intuitive keyboard shortcuts
- Multiple navigation modes
- Responsive TUI feedback
- Clear error messages
- Statistics dashboard
- Sort by name, status, current, or latest version

### Changed

- N/A (initial release)

### Deprecated

- N/A (initial release)

### Removed

- N/A (initial release)

### Fixed

- N/A (initial release)

### Security

- No security issues in initial release
- Validates package names before PyPI requests
- Safe file operations with backup protection

## Version History

### Future Roadmap

- [ ] GitHub Actions CI/CD pipeline
- [ ] Integration with Poetry and Pipenv
- [ ] Custom update rules and filtering policies
- [ ] Configuration file support (.pyelevate.toml)
- [ ] Pre/post-upgrade hooks
- [ ] Changelog generation from upgrade commits
- [ ] Integration with pip audit for security scanning
- [ ] Batch operations across multiple requirements files
- [ ] Export upgrade reports to JSON/HTML
- [ ] Plugin system for custom handlers

---

### 0.1.0 Initial Release

**Release Date:** 2024

**Features:**
- Complete interactive TUI implementation
- Full PyPI integration with async fetching
- All core functionality implemented
- Comprehensive testing suite
- Production-ready codebase
- Extensive documentation

**Known Limitations:**
- Single-file requirements processing
- No multi-workspace support
- Limited to PyPI packages
- Terminal must support 256+ colors

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to contribute to this project.

## Versioning

This project follows [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking changes (incompatible API changes)
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

---

Last updated: 2024
