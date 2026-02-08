# PyElevate

Modern Python `requirements.txt` dependency upgrader with an interactive TUI. Quickly identify, select, and upgrade Python package versions with a beautiful, responsive terminal interface.

## Features

- 🎨 **Interactive TUI** - Built with Ratatui for a smooth, responsive experience
- 🚀 **Async PyPI Fetching** - Concurrent version checks using Tokio + Reqwest
- 📊 **Smart Version Analysis** - Categorizes updates by severity (patch/minor/major)
- 🔍 **Search & Filter** - Fuzzy search to quickly find packages
- ♿ **Keyboard Navigation** - Full keyboard support with intuitive shortcuts
- 🛡️ **Safe Upgrades** - Automatic backups before modifications
- 🔒 **Lock Files** - Optional generation of locked dependency versions
- 🧪 **Dry-run Mode** - Preview changes without modifying files
- 📦 **Extras Support** - Handles package extras syntax (e.g., `requests[security]`)
- 🎯 **Smart Selection** - Bulk select all/patch/minor/major updates

## Installation

### Prerequisites

- Rust 1.70+ ([Install Rust](https://rustup.rs/))

### Build from Source

```bash
git clone https://github.com/pro-grammer-SD/pyelevatee
cd pyelevate
cargo build --release
./target/release/pyelevate --help
```

## Usage

### Interactive Mode (Default)

```bash
pyelevate
```

Opens the interactive TUI where you can:
- Navigate packages with arrow keys
- Select/deselect packages with Space
- Choose multiple packages for upgrade
- Preview changes before applying

### Check Command

View available updates without modification:

```bash
pyelevate check
```

### Upgrade Command

Perform upgrades from the command line:

```bash
pyelevate upgrade --dry-run          # Preview changes
pyelevate upgrade                    # Apply changes
pyelevate upgrade --lock             # Generate lock file
```

### Options

- `--requirements <path>` - Specify requirements file (default: `requirements.txt`)
- `--dry-run` - Preview changes without modifications
- `--lock` - Generate `requirements.lock` with pinned versions
- `--verbose` - Enable debug logging

## Keyboard Shortcuts

### Display Mode

| Key | Action |
|-----|--------|
| `↑` / `↓` | Navigate up/down |
| `PgUp` / `PgDn` | Page up/down |
| `Home` / `End` | Jump to first/last |
| `Space` | Toggle selection |
| `A` | Select all |
| `D` | Deselect all |
| `M` | Select all major updates |
| `i` | Select all minor updates |
| `P` | Select all patch updates |
| `/` | Search mode |
| `S` | Cycle sort (Name/Status/Current/Latest) |
| `Enter` | Confirm selected upgrades |
| `Esc` / `Ctrl+C` | Quit |

### Search Mode

| Key | Action |
|-----|--------|
| Type | Add to search query |
| `Backspace` | Remove from query |
| `↑` / `↓` | Navigate filtered results |
| `Space` | Toggle selection in filtered view |
| `Esc` | Return to display mode |

### Confirm Mode

| Key | Action |
|-----|--------|
| `Enter` | Confirm upgrade |
| `Esc` | Cancel and return |

## Version Status Indicators

- **✓ (Green)** - Up to date
- **◆ (Yellow)** - Patch update available
- **◈ (Cyan)** - Minor update available
- **✕ (Red)** - Major update available
- **⬆ (Magenta)** - Prerelease version
- **? (Gray)** - Version unknown
- **⚠ (Red)** - Error fetching version

## Project Structure

```
pyelevate/
├── src/
│   ├── main.rs          # CLI entrypoint and event handling
│   ├── lib.rs           # Module definitions
│   ├── app.rs           # Application state management
│   ├── models.rs        # Data structures (Package, VersionStatus, etc.)
│   ├── parser.rs        # requirements.txt parsing
│   ├── pypi.rs          # PyPI API client with caching
│   ├── ui.rs            # Ratatui TUI rendering
│   ├── upgrade.rs       # Upgrade logic and file writing
│   └── styles.rs        # Theming and styling
├── Cargo.toml           # Rust project manifest
├── README.md            # This file
└── LICENSE
```

## Technology Stack

- **Ratatui** - Terminal UI framework
- **Tokio** - Async runtime
- **Reqwest** - HTTP client
- **Semver** - Version comparison
- **Serde/Serde JSON** - JSON parsing
- **Crossterm** - Terminal I/O
- **Clap** - CLI argument parsing
- **Fuzzy-matcher** - Fuzzy search

## Examples

### Upgrade specific packages interactively

```bash
pyelevate
# Navigate to django, flask, requests
# Press Space on each to select
# Press Enter to upgrade
```

### Update all patch versions

```bash
pyelevate
# Press 'P' to select all patch updates
# Press Enter to upgrade
```

### Create a lock file

```bash
pyelevate upgrade --lock
# Creates requirements.lock with exact versions
```

### Dry-run before committing changes

```bash
pyelevate upgrade --dry-run
# See what would be upgraded without modifying files
```

## Version Parsing

PyElevate supports various version specification formats:

- **Pinned**: `package==1.2.3`
- **Greater or equal**: `package>=1.2.3`
- **Compatible**: `package~=1.2.3`
- **Extras**: `package[extra1,extra2]==1.2.3`
- **Comments**: `package==1.2.3  # Web framework`

## Backup & Safety

- All upgrades create automatic backups: `requirements.txt.backup.YYYYMMDD_HHMMSS`
- Use `--dry-run` to preview changes safely
- Original file structure and comments are preserved where possible

## Error Handling

PyElevate gracefully handles:
- Network timeouts (10-second timeout per package)
- Missing packages on PyPI
- Invalid version strings
- Malformed requirements files
- Yanked package versions

Failed packages are marked with `⚠` status in the TUI.

## Performance

- **Async batch fetching**: All packages checked concurrently
- **Result caching**: PyPI responses cached during session
- **Efficient search**: Fuzzy matching with minimal overhead
- **Memory efficient**: Streaming file processing

Typical performance for 100 packages: <5 seconds for full check.

## Development

### Building

```bash
cargo build --release
```

### Testing

```bash
cargo test
```

### Code Quality

```bash
cargo clippy
cargo fmt
```

### Debug Build

```bash
RUST_LOG=debug cargo run
```

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with [Ratatui](https://github.com/ratatui-org/ratatui) for the TUI
- PyPI API documentation and infrastructure
- The Rust async ecosystem (Tokio, Reqwest)

## Roadmap

- [ ] GitHub Actions CI/CD integration
- [ ] Integration with poetry and pipenv
- [ ] Custom update rules and filters
- [ ] Configuration file support
- [ ] Pre/post-upgrade hooks
- [ ] Changelog generation
- [ ] Integration with pip audit for security

## Troubleshooting

### PyPI connection issues

```bash
RUST_LOG=debug pyelevate
# Check network logs
```

### Terminal display issues

- Ensure your terminal supports 256 colors
- Try `export TERM=xterm-256color`

### Permission denied when writing files

```bash
chmod +w requirements.txt
pyelevate
```

## Support

For issues, questions, or suggestions:
- Open an issue on GitHub
- Check existing issues for solutions
- Include relevant logs with `--verbose` flag

## Citation

If you use PyElevate in your project, consider giving it a star ⭐

---

**Made with ❤️ for Python developers**
