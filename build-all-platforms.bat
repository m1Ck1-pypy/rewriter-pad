@echo off
setlocal enabledelayedexpansion

echo Building rewriter-pad for all platforms...

REM Create output directory
if not exist "build-output" mkdir build-output

REM Build for current platform (Windows)
echo Building for Windows (x86_64-pc-windows-msvc)...
cargo build --release --target=x86_64-pc-windows-msvc --bin RewriterPad
if !errorlevel! equ 0 (
    copy /Y .\target\x86_64-pc-windows-msvc\release\RewriterPad.exe .\build-output\RewriterPad.exe
    echo.✓ Windows build completed successfully
) else (
    echo.✗ Windows build failed
)

echo.
echo Note: Cross-compilation for Linux and macOS from Windows requires additional setup.
echo For Linux builds, you can use WSL (Windows Subsystem for Linux):
echo   1. Install WSL with Ubuntu or another Linux distribution
echo   2. Install Rust in WSL: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs ^| sh
echo   3. Install musl-tools: sudo apt install musl-tools
echo   4. Run: rustup target add x86_64-unknown-linux-musl
echo   5. Run: cargo build --release --target=x86_64-unknown-linux-musl
echo.
echo For macOS builds, you need to build on a macOS system:
echo   1. Install Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs ^| sh
echo   2. Install Xcode command line tools: xcode-select --install
echo   3. Run: rustup target add x86_64-apple-darwin
echo   4. Run: cargo build --release --target=x86_64-apple-darwin
echo.
echo Build process completed. Check build-output\ directory for binaries.

pause