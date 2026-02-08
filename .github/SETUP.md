# GitHub Actions Setup Guide

## Prerequisites

1. **GitHub Repository** - Your code must be in a GitHub repository
2. **Main Branch** - Your default branch should be `main` or `master`
3. **Cargo.toml** - Must have version specified: `version = "0.x.y"`
4. **Git History** - At least some commits for changelog generation

## Initial Setup

### Step 1: Copy Workflow Files

The workflow files are already in `.github/workflows/`:

```
.github/workflows/
├── release.yml      # Manual release workflow (workflow_dispatch)
└── ci.yml          # Automated CI on push/PR
```

### Step 2: Enable Actions

1. Go to GitHub repository
2. Click **Actions** tab
3. Enable GitHub Actions if needed
4. Workflows should appear automatically

### Step 3: Configure Branch Protection (Optional)

For production repositories:

1. Go to **Settings** → **Branches**
2. Click **Add rule** for `main` branch
3. Enable:
   - ✅ Require status checks to pass
   - ✅ Include administrators
4. Select CI jobs as required checks

### Step 4: Set Secrets (Optional)

If using custom deployments:

1. Go to **Settings** → **Secrets and variables** → **Actions**
2. Add repository secrets if needed
3. Reference in workflows: `${{ secrets.SECRET_NAME }}`

## First Release

### Step 1: Prepare Code

Ensure your code is ready:

```bash
# Update version in Cargo.toml
vim Cargo.toml
# Change: version = "0.1.0"

# Test locally
cargo test
cargo build --release

# Commit changes
git add .
git commit -m "feat: Prepare v0.2.0 release"
git push origin main
```

### Step 2: Trigger Release

1. Go to **Actions** tab
2. Select **Release PyElevate**
3. Click **Run workflow**
4. Select release type:
   - **patch** - Bug fixes
   - **minor** - New features
   - **major** - Breaking changes
5. Click **Run workflow**

### Step 3: Monitor Build

Watch the workflow progress:

1. Click on the running workflow
2. View individual job logs
3. See real-time build output
4. Check artifact generation

### Step 4: Verify Release

After workflow completes:

1. Go to **Releases** page
2. Verify release notes are correct
3. Check all assets are present
4. Download and test binary
5. Verify checksums:
   ```bash
   sha256sum -c CHECKSUMS.md
   ```

## Customization

### Changing Release Type Behavior

Edit `.github/workflows/release.yml`:

```yaml
on:
  workflow_dispatch:
    inputs:
      release_type:
        description: 'Release type'
        options:
          - patch      # Increment PATCH version
          - minor      # Increment MINOR version
          - major      # Increment MAJOR version
```

Add version bumping logic if desired (currently manual).

### Adding More Platforms

Edit the matrix in `release.yml`:

```yaml
strategy:
  matrix:
    include:
      - os: ubuntu-latest
        target: x86_64-unknown-linux-gnu
        artifact_name: pyelevate
        asset_name: pyelevate-linux-x86_64
```

Add new entries for additional targets.

### Customizing Changelog

Edit `.github/workflows/release.yml` in the `generate-changelog` job:

```bash
### Features
git log --oneline --grep='feat' HEAD...HEAD~20
```

Modify the grep patterns and formatting.

## Troubleshooting

### Workflow Not Appearing

**Problem:** Workflow doesn't show in Actions tab

**Solution:**
1. Push `.github/workflows/` files to main branch
2. Wait 30 seconds
3. Refresh GitHub Actions page
4. Check file format is valid YAML

### Build Fails on All Platforms

**Problem:** All build jobs fail

**Solution:**
1. Check code compiles locally: `cargo build --release`
2. Verify `Cargo.toml` is valid
3. Review workflow logs for exact error
4. Fix issue and re-run

### Missing Dependencies

**Problem:** Build fails with dependency errors

**Solution:**
1. Ensure `Cargo.toml` is in repository root
2. Check `Cargo.lock` is committed (optional but recommended)
3. Run `cargo update` locally
4. Commit `Cargo.lock` if using it

### Release Created But No Assets

**Problem:** GitHub release exists but binaries missing

**Solution:**
1. Check artifacts were uploaded correctly
2. Review "create-release" job in logs
3. Check file permissions
4. Manually upload assets via release edit

### Slow Builds

**Problem:** Each build takes 10+ minutes

**Solution:**
1. First build caches; subsequent builds are faster
2. Check for expensive tests: `cargo test --release`
3. Review build logs for slow operations
4. Consider splitting into separate workflow jobs

## Monitoring Releases

### GitHub Release Page

All releases appear on:
- **Releases** page (in repository sidebar)
- **Tags** page (linked from releases)
- **API** (for programmatic access)

### Release Assets

Each release includes:
- **Binaries** - Platform-specific executables
- **Checksums** - SHA256 verification
- **Release Notes** - Changelog and details

### Notifications

Configure notifications:
1. Go to **Settings** → **Notifications**
2. Enable release notifications
3. Watch repository for releases

## Advanced Features

### Pre-release Builds

To mark as pre-release:

1. Manually create release
2. Check **This is a pre-release**
3. Or edit workflow to set `prerelease: true`

### Draft Releases

Workflow creates published releases. For drafts:

Edit workflow:
```yaml
draft: true  # Creates draft release
```

### Release Tags

Workflow tags releases as: `v{version}`

Examples:
- `v0.1.0`
- `v1.0.0`
- `v2.1.3`

### GitHub Pages Integration

To publish release notes to GitHub Pages:

1. Enable GitHub Pages
2. Set source to `docs/` or branch
3. Manually create docs with release notes

## Security Best Practices

### Token Management

- ✅ `GITHUB_TOKEN` auto-provided (no setup needed)
- ✅ Scoped to current repository only
- ✅ Expires after job completes
- ✅ No secrets needed for basic releases

### Verification

Users can verify releases:
1. Check commit signature in release tag
2. Compare SHA256 checksums
3. Review source code (public on GitHub)
4. Inspect workflow logs (public)

### Access Control

Control who can trigger releases:

1. Go to **Settings** → **Actions** → **General**
2. Set **Workflow permissions**
3. Choose **Read and write** (recommended)
4. Require PR reviews before merge

## Scheduled Releases

To create automatic (scheduled) releases:

Add to workflow:
```yaml
on:
  schedule:
    - cron: '0 0 1 * *'  # Monthly on 1st
```

Then trigger workflow_dispatch manually or via schedule.

## Integration Examples

### Publish to Crates.io

Add job to release workflow:
```yaml
publish-crate:
  needs: [build]
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@stable
    - run: cargo publish --token ${{ secrets.CARGO_TOKEN }}
```

### Deploy Binary

Add job to release workflow:
```yaml
deploy:
  needs: [create-release]
  runs-on: ubuntu-latest
  steps:
    - name: Download artifacts
      uses: actions/download-artifact@v4
    - name: Deploy to server
      run: |
        # Your deployment script
```

### Notify on Slack

Add job to release workflow:
```yaml
notify-slack:
  needs: [create-release]
  runs-on: ubuntu-latest
  steps:
    - uses: 8398a7/action-slack@v3
      with:
        status: ${{ job.status }}
        text: 'Release created successfully!'
        webhook_url: ${{ secrets.SLACK_WEBHOOK }}
```

## Maintenance

### Update Workflow Versions

Regularly update action versions:

```yaml
# Check for updates
uses: actions/checkout@v4  # Latest is v4

# Update when new major versions released
uses: actions/upload-artifact@v4
uses: Swatinem/rust-cache@v2
```

### Review Logs

Periodically review workflow logs:

1. Go to **Actions** tab
2. Click on workflow runs
3. Check for warnings or errors
4. Note performance improvements

## Documentation

For detailed information:

- **Release Workflow:** See `.github/RELEASE_WORKFLOW.md`
- **GitHub Actions:** https://docs.github.com/actions
- **Workflow Syntax:** https://docs.github.com/actions/using-workflows/workflow-syntax-for-github-actions
- **Runner Environment:** https://docs.github.com/actions/using-github-hosted-runners/about-github-hosted-runners

## Support

### Getting Help

1. Check **Actions** tab for workflow logs
2. Review **RELEASE_WORKFLOW.md** documentation
3. Check GitHub Actions documentation
4. Search repository issues
5. Create new issue with workflow logs

### Reporting Problems

When reporting issues:
1. Include workflow run URL
2. Paste error message from logs
3. Describe what triggered the workflow
4. Include OS/environment info

## Quick Reference

### Essential Commands

```bash
# Test workflow locally (with act)
act workflow_dispatch

# View workflow status
gh run list --workflow release.yml

# View specific run
gh run view <run-id>

# Cancel run
gh run cancel <run-id>

# Download artifacts
gh run download <run-id>
```

### Common Files

```
.github/workflows/release.yml   # Main release workflow
.github/workflows/ci.yml        # CI/test workflow
.github/RELEASE_WORKFLOW.md     # This documentation
Cargo.toml                      # Version source
CHANGELOG.md                    # Manual changelog
```

---

**Last Updated:** February 8, 2024
**For:** PyElevate v0.1.0+
