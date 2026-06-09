$targetDir = "$env:USERPROFILE\.vscode\extensions\slate-vscode"
if (Test-Path $targetDir) {
    Remove-Item -Recurse -Force $targetDir
}
New-Item -ItemType Directory -Path $targetDir -Force
Copy-Item -Path ".\slate-vscode\*" -Destination $targetDir -Recurse -Force
Write-Host "Slate VS Code extension successfully installed to $targetDir!" -ForegroundColor Green
Write-Host "Please reload your VS Code window to apply the changes." -ForegroundColor Yellow
