use std::process::{Command, Stdio};
use std::net::TcpStream;
use std::time::Duration;
use std::path::{Path, PathBuf};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

fn find_binary_path(bin_name: &str) -> PathBuf {
    // 1. Check relative to current exe
    if let Ok(mut exe_path) = std::env::current_exe() {
        exe_path.pop(); // directory of ag.exe
        let path = exe_path.join(bin_name);
        if path.exists() {
            return path;
        }
        let path = exe_path.join("bin").join(bin_name);
        if path.exists() {
            return path;
        }
    }
    // 2. Check relative to current working directory
    let path = Path::new("bin").join(bin_name);
    if path.exists() {
        return path;
    }
    let path = Path::new(bin_name);
    if path.exists() {
        return path.to_path_buf();
    }
    PathBuf::from(bin_name)
}

fn is_port_open(port: u16) -> bool {
    let addr = format!("127.0.0.1:{}", port);
    if let Ok(socket_addr) = addr.parse() {
        TcpStream::connect_timeout(&socket_addr, Duration::from_millis(200)).is_ok()
    } else {
        false
    }
}

pub fn ensure_running() {
    // Check Docker Port first
    if is_port_open(9090) {
        println!("[Antigravity CLI] Browser Bridge Docker service detected on port 9090.");
        return;
    }

    // Docker is offline, check Local Port
    if is_port_open(3001) {
        println!("[Antigravity CLI] Browser Bridge Local service detected on port 3001.");
        return;
    }

    // Local is offline, spawn it
    println!("[Antigravity CLI] Browser Bridge Docker (9090) and Local (3001) are offline. Spawning Browser Bridge Local...");
    let bin_name = if cfg!(target_os = "windows") {
        "browser-bridge-server.exe"
    } else {
        "browser-bridge-server"
    };

    let bin_path = find_binary_path(bin_name);
    let mut cmd = Command::new(&bin_path);
    cmd.stdout(Stdio::null());
    cmd.stderr(Stdio::null());

    #[cfg(target_os = "windows")]
    {
        // 0x08000000 = CREATE_NO_WINDOW
        cmd.creation_flags(0x08000000);
    }

    match cmd.spawn() {
        Ok(_) => {
            println!("[Antigravity CLI] Successfully spawned Browser Bridge Local Service.");
            // Wait 500ms for initialization
            std::thread::sleep(Duration::from_millis(500));
        }
        Err(e) => {
            eprintln!("[Antigravity CLI] Error spawning Browser Bridge Local Service from {}: {}", bin_path.display(), e);
        }
    }
}
