# PyElevate Release Workflow Guide

## Overview

PyElevate includes a sophisticated GitHub Actions workflow for automated multi-platform builds and releases. The workflow is triggered manually via the GitHub UI and handles building, testing, and releasing binaries for Linux, macOS, and Windows.

## Features

### 🔄 Workflow Features

- **Multi-Platform Builds** - Compiles for:
  - Linux x86_64 (glibc)
  - Linux x86_64 (musl - Alpine compatible)
  - macOS x86_64
  - macOS aarch64 (Apple Silicon)
  - Windows x86_64

- **Smart Caching**
  - Rust toolchain caching via Swatinem/rust-cache
  - Cargo dependency caching
  - Fast incremental builds

- **Security**
  - SHA256 checksums for all binaries
  - Checksum verification manifest
  - Release verification documentation

- **Automation**
  - Automated changelog generation from commits
  - Release notes from commit history
  - Automatic asset uploads
  - UTC timestamp on all releases

- **Quality Gates**
  - CI testing on all push/PR
  - Clippy linting
  - Code formatting checks
  - Security audits (rustsec)

## Workflow Files

### `.github/workflows/release.yml`

Main release workflow triggered via `workflow_dispatch`:

```yaml
name: Release PyElevate
on:
  workflow_dispatch:
    inputs:
      release_type:
        description: 'Release type'
        options:
          - patch
          - minor
          - major
```

**Jobs:**
1. **build** - Compiles binaries for all platforms with matrix strategy
2. **generate-changelog** - Creates changelog from commits
3. **create-release** - Creates GitHub release with assets
4. **finalize** - Displays release summary

### `.github/workflows/ci.yml`

Continuous integration workflow:

```yaml
on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main, develop ]
  workflow_dispatch:
```

**Jobs:**
1. **test** - Runs tests on all platforms (stable + nightly Rust)
2. **clippy** - Linting checks
3. **fmt** - Code formatting verification
4. **security** - Vulnerability audits
5. **coverage** - Code coverage reports

## How to Use

### Triggering a Release

1. **Navigate** to GitHub Actions in your repository
2. **Select** "Release PyElevate" workflow
3. **Click** "Run workflow"
4. **Choose** release type:
   - **patch** - Bug fixes (0.1.0 → 0.1.1)
   - **minor** - New features (0.1.0 → 0.2.0)
   - **major** - Breaking changes (0.1.0 → 1.0.0)
5. **Click** "Run workflow"

### What Happens

1. **Build Phase** (5-10 minutes)
   - Installs Rust toolchain with all targets
   - Builds optimized release binaries for all platforms
   - Strips debug symbols from Unix binaries
   - Creates platform-specific archives

2. **Changelog Phase** (1 minute)
   - Analyzes git commit history
   - Categorizes commits (features, fixes, docs)
   - Generates changelog with contributor info

3. **Release Phase** (2 minutes)
   - Creates GitHub release
   - Uploads binaries and checksums
   - Generates installation instructions
   - Creates verification manifest

4. **Finalization** (< 1 minute)
   - Displays success summary

### Release Artifacts

Each release includes:

#### Binaries (Archived)
```
pyelevate-linux-x86_64.tar.gz        # Linux (glibc)
pyelevate-linux-musl-x86_64.tar.gz   # Linux (musl/Alpine)
pyelevate-macos-x86_64.tar.gz        # macOS Intel
pyelevate-macos-aarch64.tar.gz       # macOS ARM64
pyelevate-windows-x86_64.zip         # Windows
```

#### Checksums
```
CHECKSUMS.md    # SHA256 hashes for all binaries
```

#### Documentation
```
Release Notes   # Features, installation instructions
```

## Build Configuration

### Toolchain Setup

The workflow automatically:
1. Installs Rust via `dtolnay/rust-toolchain@stable`
2. Installs platform-specific targets
3. Installs additional tools (musl-tools for Alpine)
4. Caches everything for faster rebuilds

### Optimization

**Release Build Flags** (from Cargo.toml):
```toml
[profile.release]
opt-level = 3           # Maximum optimization
lto = true              # Link-time optimization
codegen-units = 1       # Better optimization
strip = true            # Strip debug symbols
```

### Caching Strategy

**Cache Configuration**:
- Cache scope: Workspace
- Cache all crates: `true`
- Failure mode: `cache-on-failure`
- Retention: 1 week (GitHub default)

**Caches:**
- Rust toolchain
- Cargo registry
- Compiled dependencies
- Incremental compilation artifacts

## Changelog Generation

### Automatic Detection

The workflow categorizes commits by conventional commit style:

```
feat:       🎉 Features
fix:        🐛 Bug Fixes
docs:       📖 Documentation
perf:       🚀 Performance
refactor:   🔄 Refactoring
test:       ✅ Tests
ci:         🔧 CI/CD
```

### Changelog Contents

Generated changelog includes:
- Release timestamp (UTC)
- Version number
- Feature highlights
- Bug fixes
- Documentation updates
- All commits (up to 50)
- Contributor list

### Example Changelog

```markdown
# Release Notes

**Release Time:** 2024-02-08 11:30:45 UTC

## Changes in this release

### Features
- feat: Add fuzzy search with / keybinding
- feat: Implement lock file generation

### Bug Fixes
- fix: Handle network timeouts gracefully
- fix: Preserve file formatting on upgrade

## All Commits
- abc1234: Add fuzzy search (John Doe, 2024-02-08)
- def5678: Fix timeout handling (Jane Smith, 2024-02-07)
...
```

## Verification

### Checksum Verification

**On Linux/macOS:**
```bash
sha256sum -c CHECKSUMS.md
```

**On Windows (PowerShell):**
```powershell
(Get-FileHash pyelevate-windows-x86_64.zip -Algorithm SHA256).Hash
```

### Compare with Release

Each binary's hash appears in:
1. `CHECKSUMS.md` file
2. Release notes
3. Release assets tab

## CI/CD Integration

### Automatic Testing

**On Every Push:**
- Runs tests on Linux, macOS, Windows
- Tests on Rust stable and nightly
- Runs clippy linting
- Checks code formatting
- Performs security audits
- Generates coverage reports

### Requirements for Release

All of these must pass:
- ✅ All tests pass
- ✅ No clippy warnings
- ✅ Code properly formatted
- ✅ No security vulnerabilities
- ✅ Successful build for all platforms

## Best Practices

### When Creating Releases

1. **Update Version** - Modify `Cargo.toml` before release
2. **Run Tests Locally** - Ensure `cargo test` passes
3. **Check Formatting** - Run `cargo fmt`
4. **Lint Code** - Run `cargo clippy`
5. **Commit Changes** - Make meaningful commit messages
6. **Trigger Release** - Use workflow dispatch

### Commit Message Conventions

For better changelog categorization:

```bash
# Features
git commit -m "feat: Add new feature description"

# Bug fixes
git commit -m "fix: Fix specific bug"

# Documentation
git commit -m "docs: Update README or other docs"

# Performance
git commit -m "perf: Improve performance metric"

# Breaking changes
git commit -m "feat!: Breaking change description"
```

### Release Naming

Versions follow semantic versioning:
- **MAJOR.MINOR.PATCH** (e.g., 0.1.0)
- Increment MAJOR for breaking changes
- Increment MINOR for new features
- Increment PATCH for bug fixes

## Troubleshooting

### Build Fails

**Issue:** Build fails on specific platform

**Solution:**
1. Check workflow logs in GitHub Actions
2. Review build output for error
3. Test locally on that platform: `cargo build --target <target>`
4. Fix issue and push new commit
5. Re-run workflow

### Assets Not Uploading

**Issue:** Release created but assets missing

**Solution:**
1. Check artifact upload step in workflow
2. Verify artifact paths
3. Check repository permissions (push rights required)
4. Manually upload via release edit interface

### Checksums Don't Match

**Issue:** Calculated checksum differs from release

**Solution:**
1. Re-download binary
2. Verify no corruption during download
3. Check network issues
4. Re-run release workflow if needed

### Slow Build Times

**Issue:** Workflow takes too long

**Solution:**
1. Cache is warming up on first run
2. Subsequent runs are significantly faster
3. Check for slow tests: `cargo test --release -- --nocapture`
4. Consider splitting workflow if >30 minutes

## Performance Metrics

### Typical Build Times

| Platform | First Run | Cached | Binary Size |
|----------|-----------|--------|-------------|
| Linux | 3-4 min | 1-2 min | 4.2 MB |
| macOS | 4-5 min | 1-2 min | 4.5 MB |
| Windows | 5-6 min | 2-3 min | 4.1 MB |

### Total Release Time

- **First release:** ~15-20 minutes
- **Subsequent:** ~10-15 minutes (with caching)
- **Changelog generation:** ~1 minute
- **Release creation:** ~2 minutes

## Security Considerations

### Token Usage

- **GITHUB_TOKEN** automatically provided by GitHub Actions
- Used only for creating releases and uploading assets
- No secrets required from users
- Scoped to repository only

### Binary Security

- **No external signing** - Uses GitHub's release infrastructure
- **SHA256 checksums** - Verify integrity on download
- **Source verification** - Tag points to specific commit
- **Build transparency** - Workflow logs are public

### Release Verification

Users can verify releases by:
1. Checking SHA256 against `CHECKSUMS.md`
2. Verifying GitHub release signature
3. Inspecting workflow run logs
4. Reviewing commit history before release

## Integration with CI

Workflow integrates with:
- **Status Checks** - Blocks merge if tests fail
- **Branch Protection** - Requires CI to pass
- **Auto-Deployment** - Can trigger other workflows
- **Notifications** - Sends release notifications

## Advanced Configuration

### Modifying Platforms

To add/remove platforms, edit matrix in `release.yml`:

```yaml
matrix:
  include:
    - os: ubuntu-latest
      target: x86_64-unknown-linux-gnu
      # ... configuration
```

### Adjusting Cache

Modify cache settings in workflows:

```yaml
- uses: Swatinem/rust-cache@v2
  with:
    cache-all-crates: true  # Cache all or selected only
    cache-on-failure: true  # Keep cache on failure
```

### Custom Release Notes

Edit the release body generation in `create-release` job:

```bash
# Modify release_body step to customize
```

## Maintenance

### Regular Tasks

- **Monthly:** Review and update workflow versions
- **Quarterly:** Audit dependencies
- **When Updated:** Test workflow changes on branch first
- **Before Release:** Verify all checks pass

### Cleanup

Old workflow runs can be manually deleted from GitHub Actions page. Artifacts are automatically cleaned after 1 day.

## Support & Resources

- **GitHub Actions Docs:** https://docs.github.com/actions
- **Rust Actions:** https://github.com/dtolnay/rust-toolchain
- **Swatinem Cache:** https://github.com/Swatinem/rust-cache
- **PyElevate Issues:** GitHub Issues tab

## Example Release

Complete example of a successful release:

**Release:** PyElevate v0.2.0
**Time:** 2024-02-08 12:00:00 UTC
**Duration:** 12 minutes 34 seconds

**Assets:**
- pyelevate-linux-x86_64.tar.gz (4.2 MB)
- pyelevate-linux-musl-x86_64.tar.gz (4.1 MB)
- pyelevate-macos-x86_64.tar.gz (4.5 MB)
- pyelevate-macos-aarch64.tar.gz (4.3 MB)
- pyelevate-windows-x86_64.zip (4.1 MB)
- CHECKSUMS.md

**Changes:**
- ✨ 3 new features
- 🐛 5 bug fixes
- 📖 2 documentation updates
- 👥 12 contributors

---

**Last Updated:** February 8, 2024
**Version:** 1.0
