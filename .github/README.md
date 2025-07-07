# GitHub Actions CI/CD Configuration

This directory contains GitHub Actions workflow files for continuous integration and deployment.

## Workflows

### 1. CI (`ci.yml`)
Runs on every push and pull request to `main` and `develop` branches.

**Features:**
- **Multi-platform testing**: Tests on stable, beta, and nightly Rust versions
- **Code quality**: Runs `rustfmt` and `clippy` checks
- **Cross-platform builds**: Builds for Linux, Windows, macOS (x86_64 and ARM64)
- **Security audit**: Uses `cargo-audit` to check for vulnerabilities
- **Code coverage**: Generates coverage reports and uploads to Codecov

### 2. Release (`release.yml`)
Triggers on version tags (e.g., `v1.0.0`).

**Features:**
- **Automated releases**: Creates GitHub releases with binaries
- **Cross-platform binaries**: Builds for multiple platforms
- **Crates.io publishing**: Automatically publishes to crates.io
- **Archive creation**: Creates compressed archives for each platform

### 3. Dependencies (`dependencies.yml`)
Runs weekly to keep dependencies up-to-date.

**Features:**
- **Scheduled updates**: Runs every Monday at 9:00 AM UTC
- **Automatic PRs**: Creates pull requests with dependency updates
- **Test validation**: Ensures tests pass before creating PR

### 4. Documentation (`docs.yml`)
Builds and deploys documentation.

**Features:**
- **Auto-generated docs**: Builds Rust documentation with `cargo doc`
- **GitHub Pages**: Deploys documentation to GitHub Pages
- **Updated on main**: Automatically updates docs when main branch changes

## Setup Requirements

### Secrets
Configure these secrets in your GitHub repository settings:

1. `CARGO_REGISTRY_TOKEN`: Token for publishing to crates.io
   - Get from [crates.io/me](https://crates.io/me)
   - Add to repository secrets

### Branch Protection
Recommended branch protection rules for `main`:

- Require status checks to pass before merging
- Require branches to be up to date before merging
- Required status checks:
  - `Test Suite (stable)`
  - `Build Release`
  - `Security Audit`

### Code Coverage
To use Codecov:

1. Sign up at [codecov.io](https://codecov.io)
2. Connect your GitHub repository
3. No additional secrets needed (uses GitHub token)

## Usage

### Triggering Workflows

- **CI**: Automatically runs on push/PR
- **Release**: Push a version tag: `git tag v1.0.0 && git push origin v1.0.0`
- **Dependencies**: Runs automatically weekly, or trigger manually
- **Documentation**: Automatically runs on main branch changes

### Manual Triggering

Some workflows support manual triggering via GitHub's "Actions" tab:
- Dependencies update
- Documentation build

### Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` (if you have one)
3. Commit changes: `git commit -m "chore: release v1.0.0"`
4. Create and push tag: `git tag v1.0.0 && git push origin v1.0.0`
5. GitHub Actions will automatically:
   - Create a GitHub release
   - Build and attach binaries
   - Publish to crates.io

## Customization

### Adding New Platforms
To add support for additional platforms, update the `matrix` in `ci.yml` and `release.yml`:

```yaml
- os: ubuntu-latest
  target: aarch64-unknown-linux-gnu
  suffix: ""
```

### Changing Test Requirements
Modify the `test` job in `ci.yml` to add or remove test requirements:

```yaml
- name: Run integration tests
  run: cargo test --test integration_tests
```

### Custom Release Notes
Edit the release body in `release.yml` or use a changelog generator tool.
