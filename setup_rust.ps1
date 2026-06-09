$ErrorActionPreference = "Stop"

# Download path
$url = "https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe"
$output = Join-Path $PSScriptRoot "rustup-init.exe"

if (Test-Path $output) {
    Remove-Item $output
}

Write-Host "Downloading Rustup from $url..."
Invoke-WebRequest -Uri $url -OutFile $output

Write-Host "Installing Rust (stable-x86_64-pc-windows-msvc) silently..."
# Run the installer with silent flag (-y)
& $output -y --default-toolchain stable

Write-Host "Rust installation complete. Cleaning up..."
if (Test-Path $output) {
    Remove-Item $output
}

# Verify cargo path
$cargoBin = Join-Path $env:USERPROFILE ".cargo\bin"
if (Test-Path $cargoBin) {
    Write-Host "Adding Cargo to the environment path for this session..."
    $env:PATH += ";$cargoBin"
    Write-Host "Success! Cargo is located at $cargoBin"
} else {
    Write-Warning "Could not find Cargo bin directory at $cargoBin"
}
