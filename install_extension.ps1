# Install to standard VS Code extensions directory
$targetDir1 = "$env:USERPROFILE\.vscode\extensions\slate-vscode"
if (Test-Path $targetDir1) {
    Remove-Item -Recurse -Force $targetDir1
}
New-Item -ItemType Directory -Path $targetDir1 -Force
Copy-Item -Path ".\slate-vscode\*" -Destination $targetDir1 -Recurse -Force
Write-Host "Slate VS Code extension successfully installed to $targetDir1!" -ForegroundColor Green

# Install to Antigravity IDE extensions directory
$targetDir2 = "$env:USERPROFILE\.antigravity-ide\extensions\slate-vscode"
if (Test-Path $targetDir2) {
    Remove-Item -Recurse -Force $targetDir2
}
New-Item -ItemType Directory -Path $targetDir2 -Force
Copy-Item -Path ".\slate-vscode\*" -Destination $targetDir2 -Recurse -Force
Write-Host "Slate extension successfully installed to Antigravity IDE: $targetDir2!" -ForegroundColor Green

Write-Host "Please reload your IDE window to apply the changes." -ForegroundColor Yellow
