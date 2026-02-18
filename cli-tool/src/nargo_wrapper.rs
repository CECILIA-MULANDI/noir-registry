use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Handle commands that we delegate to our binaries
    if args.len() > 1 {
        let command = &args[1];
        let binary_name = match command.as_str() {
            "add" => "nargo-add",
            "remove" => "nargo-remove",
            "publish" => "nargo-publish",
            "login" => "nargo-login",
            _ => {
                // Not one of our commands, pass through to real nargo
                let real_nargo = find_real_nargo().unwrap_or_else(|| {
                    eprintln!("Error: Could not find nargo binary in PATH");
                    eprintln!("Please ensure nargo is installed and in your PATH");
                    std::process::exit(1);
                });

                let mut cmd = Command::new(real_nargo);
                cmd.args(&args[1..]);

                match cmd.status() {
                    Ok(status) => std::process::exit(status.code().unwrap_or(1)),
                    Err(e) => {
                        eprintln!("❌ Failed to execute nargo: {}", e);
                        std::process::exit(1);
                    }
                }
            }
        };

        let binary_path = find_binary(binary_name).unwrap_or_else(|| {
            eprintln!("❌ Error: Could not find {} binary", binary_name);
            eprintln!(
                "   Please ensure {} is installed and in your PATH",
                binary_name
            );
            eprintln!(
                "   Install with: cargo install --path cli-tool --bin {}",
                binary_name
            );
            std::process::exit(1);
        });

        let mut cmd = Command::new(&binary_path);
        if args.len() > 2 {
            cmd.args(&args[2..]);
        }

        match cmd.status() {
            Ok(status) => {
                std::process::exit(status.code().unwrap_or(1));
            }
            Err(e) => {
                eprintln!("❌ Failed to execute {}: {}", binary_name, e);
                eprintln!("   Path tried: {:?}", binary_path);
                std::process::exit(1);
            }
        }
    }

    // No arguments - pass through to real nargo
    let real_nargo = find_real_nargo().unwrap_or_else(|| {
        eprintln!("Error: Could not find nargo binary in PATH");
        eprintln!("Please ensure nargo is installed and in your PATH");
        std::process::exit(1);
    });

    let mut cmd = Command::new(real_nargo);
    match cmd.status() {
        Ok(status) => std::process::exit(status.code().unwrap_or(1)),
        Err(e) => {
            eprintln!("❌ Failed to execute nargo: {}", e);
            std::process::exit(1);
        }
    }
}

/// Find a binary (nargo-add, nargo-publish, etc.) in PATH or common locations
fn find_binary(binary_name: &str) -> Option<PathBuf> {
    // First, try to find in the same directory as this wrapper
    if let Ok(current_exe) = env::current_exe() {
        let same_dir = current_exe.with_file_name(binary_name);
        if same_dir.exists() {
            return Some(same_dir);
        }
    }

    // If not found, search in PATH
    if let Ok(path) = env::var("PATH") {
        for dir in path.split(':') {
            let candidate = std::path::Path::new(dir).join(binary_name);
            if candidate.exists() {
                return Some(candidate);
            }
        }
    }

    // Fallback: try common installation locations
    if let Ok(home) = env::var("HOME") {
        let common_paths = vec![
            format!("{}/.cargo/bin/{}", home, binary_name),
            format!("{}/.local/bin/{}", home, binary_name),
            format!("/usr/local/bin/{}", binary_name),
            format!("/usr/bin/{}", binary_name),
        ];

        for path_str in common_paths {
            let path = std::path::Path::new(&path_str);
            if path.exists() {
                return Some(path.to_path_buf());
            }
        }
    }

    None
}
fn find_real_nargo() -> Option<String> {
    // First, try to find nargo in PATH (but skip ourselves)
    if let Ok(path) = env::var("PATH") {
        for dir in path.split(':') {
            let nargo_path = std::path::Path::new(dir).join("nargo");
            if nargo_path.exists() {
                // Check if it's not us (compare canonical paths)
                let canon_nargo = std::fs::canonicalize(&nargo_path).ok();
                let canon_self = env::current_exe()
                    .ok()
                    .and_then(|p| std::fs::canonicalize(p).ok());

                if let (Some(canon_nargo), Some(canon_self)) = (canon_nargo, canon_self) {
                    if canon_nargo != canon_self {
                        return Some(nargo_path.to_string_lossy().to_string());
                    }
                } else {
                    // If we can't canonicalize, just use it (might be us, but worth trying)
                    return Some(nargo_path.to_string_lossy().to_string());
                }
            }
        }
    }

    // Fallback: try common installation locations
    let home = env::var("HOME").unwrap_or_default();
    let common_paths = vec![
        "/usr/local/bin/nargo".to_string(),
        "/usr/bin/nargo".to_string(),
        format!("{}/.cargo/bin/nargo", home),
    ];

    for path in common_paths {
        if std::path::Path::new(&path).exists() {
            return Some(path);
        }
    }

    None
}
