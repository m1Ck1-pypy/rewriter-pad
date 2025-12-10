# Cross-Compilation Guide for Rewriter Pad

This guide explains how to build the Rewriter Pad application for different platforms.

## Current Build Status

- ✅ **Windows (x86_64-pc-windows-msvc)**: Successfully built on Windows
- ❌ **Linux (x86_64-unknown-linux-gnu/musl)**: Requires additional setup
- ❌ **macOS (x86_64-apple-darwin)**: Requires additional setup

## Building for Different Platforms

### Building for Windows (from Windows)
```bash
cargo build --release --target=x86_64-pc-windows-msvc
```

### Building for Linux (from Windows using WSL)

1. Install Windows Subsystem for Linux (WSL)
2. Install Ubuntu or another Linux distribution from the Microsoft Store
3. Open your WSL terminal and navigate to your project directory (accessible via `/mnt/d/me/rust/rewriter-pad` if your project is on D:\ drive)
4. Install Rust in WSL:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```
5. Add the Linux target:
```bash
rustup target add x86_64-unknown-linux-musl
```
6. Install musl-tools:
```bash
sudo apt update
sudo apt install musl-tools
```
7. Build:
```bash
cargo build --release --target=x86_64-unknown-linux-musl
```

### Building for macOS

Cross-compilation from Windows to macOS is not supported. You need to build on a macOS system:

1. Install Xcode command line tools:
```bash
xcode-select --install
```
2. Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
3. Add the macOS target:
```bash
rustup target add x86_64-apple-darwin
```
4. Build:
```bash
cargo build --release --target=x86_64-apple-darwin
```

## Using Docker for Cross-Platform Builds

If you want to build for Linux from Windows without WSL, you can use Docker:

### Linux Build with Docker
```bash
docker run --rm -v "$(pwd)":/usr/src/myapp -w /usr/src/myapp rust:1.90.0 cargo build --release --target=x86_64-unknown-linux-musl
```

### Multi-platform Builds with GitHub Actions

For automated builds across all platforms, consider using GitHub Actions with the following workflow:

```yaml
name: Build and Release

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-musl
            artifact_name: RewriterPad-linux
            asset_name: RewriterPad-linux
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            artifact_name: RewriterPad.exe
            asset_name: RewriterPad.exe
          - os: macos-latest
            target: x86_64-apple-darwin
            artifact_name: RewriterPad-macos
            asset_name: RewriterPad-macos

    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}
      - name: Install musl-tools (Linux)
        if: matrix.os == 'ubuntu-latest'
        run: sudo apt-get update && sudo apt-get install -y musl-tools
      - name: Build
        run: |
          rustup target add ${{ matrix.target }}
          cargo build --release --target ${{ matrix.target }}
      - name: Upload binaries to release
        uses: svenstaro/upload-release-action@v2
        with:
          repo_token: ${{ secrets.GITHUB_TOKEN }}
          file: target/${{ matrix.target }}/release/${{ matrix.artifact_name }}
          asset_name: ${{ matrix.asset_name }}
          tag: ${{ github.ref }}
```

## Build Scripts

This project includes the following build scripts:

- `build-all-platforms.sh`: Bash script for building on Unix-like systems
- `build-all-platforms.bat`: Batch script for Windows systems

## Notes

1. The Windows build completed successfully when running the initial build.
2. Cross-compilation from Windows to Linux/macOS requires additional setup as detailed above.
3. The GUI dependencies (eframe, egui) should work correctly on all platforms with proper compilation.
4. For distribution, you may need to bundle additional resources or libraries depending on your target platform.