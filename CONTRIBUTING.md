# Contributing to PyElevate

Thank you for your interest in contributing to PyElevate! This document provides guidelines and instructions for contributing.

## Code of Conduct

Be respectful, inclusive, and professional in all interactions.

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Git
- A Unix-like environment (Linux, macOS, WSL2 on Windows)

### Setting Up Development Environment

```bash
# Clone the repository
git clone https://github.com/pro-grammer-SD/pyelevatee
cd pyelevate

# Build the project
cargo build

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run
```

## Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-number
```

Use descriptive names:
- `feature/fuzzy-search-improvement`
- `fix/pypi-timeout-handling`
- `docs/update-readme`

### 2. Make Changes

- Keep changes focused and logical
- Follow Rust conventions (use `cargo fmt`)
- Add tests for new functionality
- Update documentation as needed

### 3. Code Style

```bash
# Format code
cargo fmt

# Check for issues
cargo clippy -- -D warnings

# Run tests
cargo test
```

### 4. Commit Messages

Use clear, descriptive commit messages:

```
Add fuzzy search for package filtering

- Implement fuzzy matcher integration
- Add /: search hotkey binding
- Update help text with search instructions
- Add tests for search filtering
```

### 5. Push and Create PR

```bash
git push origin feature/your-feature-name
```

Then create a Pull Request on GitHub with:
- Clear title describing the change
- Description of what changed and why
- Reference to related issues (e.g., `Fixes #123`)
- Screenshots for UI changes

## Project Structure

```
src/
├── main.rs      # CLI and event loop
├── lib.rs       # Module definitions
├── app.rs       # State management
├── models.rs    # Data structures
├── parser.rs    # requirements.txt parsing
├── pypi.rs      # PyPI API client
├── ui.rs        # Ratatui rendering
├── upgrade.rs   # Upgrade logic
└── styles.rs    # Theming
```

### Adding a New Feature

1. **Plan the architecture**
   - Update relevant modules
   - Design data flow
   - Consider edge cases

2. **Implement**
   - Add code following style guidelines
   - Write tests
   - Document public APIs

3. **Test**
   - Unit tests: `cargo test`
   - Integration tests
   - Manual testing with example files

4. **Document**
   - Update README.md if user-facing
   - Add doc comments to public items
   - Update CHANGELOG if applicable

## Testing Guidelines

### Unit Tests

Place tests in the same file as the code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_behavior() {
        // Arrange
        let input = "test";
        
        // Act
        let result = some_function(input);
        
        // Assert
        assert_eq!(result, "expected");
    }
}
```

### Async Tests

For async code, use `#[tokio::test]`:

```rust
#[tokio::test]
async fn test_async_operation() {
    let result = async_function().await;
    assert!(result.is_ok());
}
```

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_feature_behavior

# With output
cargo test -- --nocapture

# Specific module
cargo test --lib app::tests
```

## Documentation

### Code Comments

```rust
/// Fetches the latest version of a package from PyPI.
///
/// # Arguments
/// * `package_name` - The name of the package
///
/// # Returns
/// * `Result<String>` - Latest version or error
///
/// # Example
/// ```
/// let version = client.fetch_latest_version("requests").await?;
/// ```
pub async fn fetch_latest_version(&self, package_name: &str) -> Result<String> {
    // Implementation
}
```

### Documentation Tests

```bash
cargo test --doc
```

## Performance Considerations

- Use async/await for I/O operations
- Cache PyPI responses
- Minimize allocations in hot paths
- Profile with: `cargo flamegraph`

## Debugging

```bash
# Debug build with logging
RUST_LOG=debug cargo run

# More verbose logging
RUST_LOG=trace cargo run

# Attach debugger (lldb on macOS)
rust-lldb ./target/debug/pyelevate
```

## Submitting Changes

### Before PR

- [ ] Code compiles: `cargo build --release`
- [ ] Tests pass: `cargo test`
- [ ] Formatted: `cargo fmt`
- [ ] No warnings: `cargo clippy`
- [ ] Documentation updated
- [ ] Commits are clean and logical

### PR Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Related Issues
Fixes #issue_number

## Testing
Describe how to test these changes

## Checklist
- [ ] Code compiles
- [ ] All tests pass
- [ ] New tests added
- [ ] Documentation updated
```

## Review Process

1. Automated checks must pass (CI/CD)
2. Code review by maintainers
3. Approval and merge

## Reporting Issues

### Bug Reports

Include:
- PyElevate version
- OS and Rust version
- Steps to reproduce
- Expected vs actual behavior
- Relevant logs (`--verbose` output)

### Feature Requests

Include:
- Use case/motivation
- Proposed behavior
- Alternatives considered
- Acceptance criteria

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Ratatui Docs](https://docs.rs/ratatui/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Semantic Versioning](https://semver.org/)

## Questions?

- Check existing issues and discussions
- Open a new discussion
- Contact maintainers

## Recognition

Contributors will be recognized in:
- CONTRIBUTORS.md file
- Release notes
- Project documentation

Thank you for contributing! 🚀
