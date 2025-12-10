# GitHub Deployment Guide for Rewriter Pad

This guide explains how to set up GitHub repository, push your code, and create releases with your built binaries.

## 1. Initial Setup: Create GitHub Repository

### Step 1: Create Repository on GitHub

1. Go to https://github.com and log into your account
2. Click the "+" icon in the top right corner and select "New repository"
3. Name your repository (e.g., `rewriter-pad`)
4. Make it public or private as per your preference
5. Do NOT initialize with README, .gitignore, or license (we already have these)
6. Click "Create repository"

### Step 2: Add GitHub Remote and Push Code

```bash
# In your project directory
git remote add origin https://github.com/m1Ck1-pypy/rewriter-pad.git
git branch -M main
git push -u origin main
```

## 2. Create GitHub Release

### Manual Release Process:

1. Go to your GitHub repository
2. Click on the "Releases" tab (or "Tags" then "Create a new release")
3. Click "Draft a new release"
4. Create a tag (e.g., `v0.1.0`) and set release title (e.g., "Version 0.1.0")
5. Write release notes describing changes
6. Attach the built binaries:
   - `RewriterPad.exe` (Windows build)
   - `RewriterPad-linux` (Linux build - when available)
   - `RewriterPad-macos` (macOS build - when available)
7. Click "Publish release"

## 3. Automated GitHub Actions for Building and Releasing

Create the following GitHub Actions workflow to automate builds and releases:

### Create `.github/workflows/release.yml`:

```yaml
name: Release

on:
  push:
    tags:
      - "v*" # Triggers when a tag starting with 'v' is pushed

env:
  CARGO_TERM_COLOR: always

jobs:
  create-release:
    runs-on: ubuntu-latest
    outputs:
      release_id: ${{ steps.create-release.outputs.id }}
      release_upload_url: ${{ steps.create-release.outputs.upload_url }}

    steps:
      - name: Create Release
        id: create-release
        uses: actions/create-release@v1
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          tag_name: ${{ github.ref }}
          release_name: Release ${{ github.ref }}
          draft: false
          prerelease: false

  build:
    needs: create-release
    runs-on: ${{ matrix.os }}

    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-musl
            artifact_name: RewriterPad
            asset_name: RewriterPad-linux
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            artifact_name: RewriterPad.exe
            asset_name: RewriterPad.exe
          - os: macos-latest
            target: x86_64-apple-darwin
            artifact_name: RewriterPad
            asset_name: RewriterPad-macos

    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}

      - name: Install musl-tools (Linux)
        if: matrix.os == 'ubuntu-latest'
        run: |
          sudo apt-get update
          sudo apt-get install -y musl-tools

      - name: Build
        run: |
          rustup target add ${{ matrix.target }}
          cargo build --release --target ${{ matrix.target }} --bin RewriterPad

      - name: Upload Release Asset
        uses: actions/upload-release-asset@v1
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          upload_url: ${{ needs.create-release.outputs.release_upload_url }}
          asset_path: ./target/${{ matrix.target }}/release/${{ matrix.artifact_name }}
          asset_name: ${{ matrix.asset_name }}
          asset_content_type: application/octet-stream
```

## 4. Using the GitHub CLI for Releases

If you prefer using GitHub CLI instead of the website:

### Install GitHub CLI:

- Windows: `winget install GitHub.cli` or download from https://cli.github.com/
- macOS: `brew install gh`
- Linux: `sudo apt install gh` or `brew install gh`

### Create Release with GitHub CLI:

```bash
# Authenticate
gh auth login

# Create release with assets
gh release create v0.1.0 \
  --title "v0.1.0" \
  --notes "Initial release" \
  ./target/x86_64-pc-windows-msvc/release/RewriterPad.exe#RewriterPad.exe
```

## 5. Preparing for Release

Before creating your first release, make sure to:

### Update `Cargo.toml` for release:

```toml
[package]
name = "rewriter-pad"
version = "0.1.0"  # Update this for each release
edition = "2024"
description = "A simple rewriter application"
license = "MIT"  # Or your chosen license
repository = "https://github.com/m1Ck1-pypy/rewriter-pad"
```

### Create or update these files in your repository:

#### `README.md`:

```markdown
# Rewriter Pad

A simple application for rewriting files.

## Installation

Download the appropriate binary for your platform from the [releases page](https://github.com/m1Ck1-pypy/rewriter-pad/releases).

## Usage

Run the executable and use the graphical interface to work with your files.
```

#### `LICENSE`:

Consider adding a license file (e.g., MIT, GPL, etc.) to your repository.

## 6. Tagging and Releasing Process

### For manual releases:

```bash
# Update version in Cargo.toml
# Build for your platform(s)
# Create a tag
git tag -a v0.1.0 -m "Version 0.1.0"
git push origin v0.1.0
```

### For automated releases with GitHub Actions:

1. Update version in `Cargo.toml`
2. Commit and push changes to main branch
3. Create and push a new tag (format: `v*.*.*`)

## 7. GitHub Secrets for Actions

If using the GitHub Actions workflow, note that it uses the built-in `GITHUB_TOKEN` secret, so no additional configuration is needed.

## 8. Verification Steps

After setting everything up:

1. Verify code is pushed to GitHub
2. Confirm GitHub Actions workflow is configured correctly
3. Test that the Windows executable works properly
4. Plan for Linux/macOS builds using the documentation in `CROSS-COMPILATION.md`

Your releases will then be available at: `https://github.com/m1Ck1-pypy/rewriter-pad/releases`
