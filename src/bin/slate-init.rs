use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("\x1b[35m====================================================\x1b[0m");
    println!("\x1b[35m            SLATE - UNIVERSAL VISUAL LANGUAGE       \x1b[0m");
    println!("\x1b[35m                 Installer v0.1 Alpha               \x1b[0m");
    println!("\x1b[35m====================================================\x1b[0m");
    println!("Installing Slate to your system...\n");

    // Get user home directory
    let home_dir = match dirs_home_dir() {
        Some(path) => path,
        None => {
            eprintln!("\x1b[31mError: Could not resolve home directory.\x1b[0m");
            std::process::exit(1);
        }
    };

    let slate_dir = home_dir.join(".slate");
    let bin_dir = slate_dir.join("bin");

    // Create directories
    if let Err(e) = fs::create_dir_all(&bin_dir) {
        eprintln!("\x1b[31mError: Failed to create directories: {}\x1b[0m", e);
        std::process::exit(1);
    }

    println!("Creating installation files...");

    // Embed slate.exe (generated from compilation of the main package)
    let slate_exe_bytes = include_bytes!("../../target/release/slate.exe");
    let slate_exe_path = bin_dir.join("slate.exe");
    if let Err(e) = fs::write(&slate_exe_path, slate_exe_bytes) {
        eprintln!("\x1b[31mError: Failed to write slate.exe: {}\x1b[0m", e);
        std::process::exit(1);
    }
    println!("  -> Written compiler: {:?}", slate_exe_path);

    // Embed slate-preview.exe (generated from compiling slate-preview.cs)
    let preview_exe_bytes = include_bytes!("../../slate-preview.exe");
    let preview_exe_path = bin_dir.join("slate-preview.exe");
    if let Err(e) = fs::write(&preview_exe_path, preview_exe_bytes) {
        eprintln!("\x1b[31mError: Failed to write slate-preview.exe: {}\x1b[0m", e);
        std::process::exit(1);
    }
    println!("  -> Written preview application: {:?}", preview_exe_path);

    // Embed logo
    let logo_bytes = include_bytes!("../../SLATE-LANG-LOGO TT.png");
    let logo_path = slate_dir.join("logo.png");
    if let Err(e) = fs::write(&logo_path, logo_bytes) {
        eprintln!("\x1b[31mWarning: Failed to save logo.png: {}\x1b[0m", e);
    } else {
        println!("  -> Saved logo to {:?}", logo_path);
    }

    // Embed icon
    let ico_bytes = include_bytes!("../../logo.ico");
    let ico_path = slate_dir.join("logo.ico");
    if let Err(e) = fs::write(&ico_path, ico_bytes) {
        eprintln!("\x1b[31mWarning: Failed to save logo.ico: {}\x1b[0m", e);
    } else {
        println!("  -> Saved icon to {:?}", ico_path);
    }

    // Register file association using PowerShell
    println!("Registering .slt file association and creating Desktop shortcut...");
    let assoc_script = format!(
        "New-Item -Path 'HKCU:\\Software\\Classes\\.slt' -Force | Out-Null; \
         Set-ItemProperty -Path 'HKCU:\\Software\\Classes\\.slt' -Name '(Default)' -Value 'Slate.Document' -Force; \
         New-Item -Path 'HKCU:\\Software\\Classes\\Slate.Document\\DefaultIcon' -Force | Out-Null; \
         Set-ItemProperty -Path 'HKCU:\\Software\\Classes\\Slate.Document' -Name '(Default)' -Value 'Slate Visual File' -Force; \
         Set-ItemProperty -Path 'HKCU:\\Software\\Classes\\Slate.Document\\DefaultIcon' -Name '(Default)' -Value '{}' -Force; \
         New-Item -Path 'HKCU:\\Software\\Classes\\Slate.Document\\shell\\open\\command' -Force | Out-Null; \
         Set-ItemProperty -Path 'HKCU:\\Software\\Classes\\Slate.Document\\shell\\open\\command' -Name '(Default)' -Value '\"{}\" \"%1\"' -Force; \
         $WshShell = New-Object -ComObject WScript.Shell; \
         $Shortcut = $WshShell.CreateShortcut(\"$Home\\Desktop\\Slate Studio.lnk\"); \
         $Shortcut.TargetPath = '{}'; \
         $Shortcut.IconLocation = '{}'; \
         $Shortcut.WorkingDirectory = '{}'; \
         $Shortcut.Save(); \
         $sig = '[DllImport(\"shell32.dll\")] public static extern void SHChangeNotify(uint wEventId, uint uFlags, IntPtr dwItem1, IntPtr dwItem2);'; \
         $type = Add-Type -MemberDefinition $sig -Name 'Shell32' -Namespace 'Win32' -PassThru; \
         $type::SHChangeNotify(0x08000000, 0, [IntPtr]::Zero, [IntPtr]::Zero);",
        ico_path.to_string_lossy().replace("\\", "\\\\"),
        preview_exe_path.to_string_lossy().replace("\\", "\\\\"),
        preview_exe_path.to_string_lossy().replace("\\", "\\\\"),
        ico_path.to_string_lossy().replace("\\", "\\\\"),
        bin_dir.to_string_lossy().replace("\\", "\\\\")
    );

    let _ = Command::new("powershell")
        .args(&["-Command", &assoc_script])
        .output();

    // Configure PATH env variable
    println!("Configuring PATH environment variable...");
    let bin_path_str = bin_dir.to_string_lossy().to_string();
    
    // Check and add using PowerShell
    let ps_script = format!(
        "$p = [Environment]::GetEnvironmentVariable('PATH', 'User'); \
         if (-not $p.Contains('{}')) {{ \
             [Environment]::SetEnvironmentVariable('PATH', $p + ';{}', 'User'); \
             write-output 'PATH updated' \
         }} else {{ \
             write-output 'PATH already configured' \
         }}",
        bin_path_str.replace("\\", "\\\\"),
        bin_path_str.replace("\\", "\\\\")
    );

    let output = Command::new("powershell")
        .args(&["-Command", &ps_script])
        .output();

    match output {
        Ok(out) => {
            let out_str = String::from_utf8_lossy(&out.stdout);
            if out_str.contains("PATH updated") {
                println!("\x1b[32m  -> PATH successfully updated!\x1b[0m");
            } else {
                println!("  -> PATH was already configured.");
            }
        }
        Err(e) => {
            eprintln!("\x1b[31mWarning: Failed to update PATH automatically: {}\x1b[0m", e);
            println!("Please manually add this folder to your PATH: {:?}", bin_dir);
        }
    }

    println!("\x1b[32m\nSlate has been successfully installed!\x1b[0m");
    println!("To start using Slate, restart your terminal and type:");
    println!("  \x1b[36mslate help\x1b[0m\n");
    println!("Or watch a slate file in real-time:");
    println!("  \x1b[36mslate watch path/to/file.slt\x1b[0m\n");
    println!("\x1b[35m====================================================\x1b[0m");
}

fn dirs_home_dir() -> Option<PathBuf> {
    std::env::var_os("USERPROFILE")
        .or_else(|| std::env::var_os("HOME"))
        .map(PathBuf::from)
}
