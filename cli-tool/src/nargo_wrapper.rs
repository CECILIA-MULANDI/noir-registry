use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    // If first argument is "add", delegate to nargo-add
    if args.len() > 1 && args[1] == "add" {
        // Try to find nargo-add binary
        let nargo_add_path = find_nargo_add().unwrap_or_else(|| {
            eprintln!("❌ Error: Could not find nargo-add binary");
            eprintln!("   Please ensure nargo-add is installed and in your PATH");
            eprintln!("   Install with: cargo install --path cli-tool --bin nargo-add");
            std::process::exit(1);
        });

        // Build command with remaining arguments (skip "nargo" and "add")
        let mut cmd = Command::new(&nargo_add_path);
        if args.len() > 2 {
            // Pass all arguments after "add" to nargo-add
            cmd.args(&args[2..]);
        }

        // Execute nargo-add
        match cmd.status() {
            Ok(status) => {
                std::process::exit(status.code().unwrap_or(1));
            }
            Err(e) => {
                eprintln!("❌ Failed to execute nargo-add: {}", e);
                eprintln!("   Path tried: {:?}", nargo_add_path);
                std::process::exit(1);
            }
        }
    } else {
        // For all other commands, pass through to the real nargo
        let real_nargo = find_real_nargo().unwrap_or_else(|| {
            eprintln!("Error: Could not find nargo binary in PATH");
            eprintln!("Please ensure nargo is installed and in your PATH");
            std::process::exit(1);
        });

        let mut cmd = Command::new(real_nargo);
        if args.len() > 1 {
            cmd.args(&args[1..]);
        }

        match cmd.status() {
            Ok(status) => std::process::exit(status.code().unwrap_or(1)),
            Err(e) => {
                eprintln!("❌ Failed to execute nargo: {}", e);
                std::process::exit(1);
            }
        }
    }
}

fn find_nargo_add() -> Option<PathBuf> {
    // First, try to find nargo-add in the same directory as this wrapper
    if let Ok(current_exe) = env::current_exe() {
        let same_dir = current_exe.with_file_name("nargo-add");
        if same_dir.exists() {
            return Some(same_dir);
        }
    }

    // If not found, search in PATH
    if let Ok(path) = env::var("PATH") {
        for dir in path.split(':') {
            let candidate = std::path::Path::new(dir).join("nargo-add");
            if candidate.exists() {
                return Some(candidate);
            }
        }
    }

    // Fallback: try common installation locations
    if let Ok(home) = env::var("HOME") {
        let common_paths = vec![
            format!("{}/.cargo/bin/nargo-add", home),
            format!("{}/.local/bin/nargo-add", home),
            "/usr/local/bin/nargo-add".to_string(),
            "/usr/bin/nargo-add".to_string(),
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
