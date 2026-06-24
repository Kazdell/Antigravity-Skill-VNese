use std::process::Command;
use crate::compressor;

pub fn run_command(cmd_str: &str, token_limit: usize) {
    eprintln!("[Antigravity Token Saver Proxy] Running: {}", cmd_str);
    
    // Choose shell based on OS (cmd.exe on Windows, sh on Unix)
    let mut command = if cfg!(target_os = "windows") {
        let mut c = Command::new("cmd");
        c.args(["/C", cmd_str]);
        c
    } else {
        let mut c = Command::new("sh");
        c.args(["-c", cmd_str]);
        c
    };

    // Execute command and capture output
    match command.output() {
        Ok(output) => {
            // Combine stdout and stderr
            let stdout_str = String::from_utf8_lossy(&output.stdout);
            let stderr_str = String::from_utf8_lossy(&output.stderr);
            
            let mut combined_output = stdout_str.to_string();
            if !stderr_str.is_empty() {
                combined_output.push_str("\n--- STDERR ---\n");
                combined_output.push_str(&stderr_str);
            }

            // Estimate line limit from token limit (approx 20 chars per line, or simple line constraint)
            // If token limit is 3000, we map to ~150 lines max
            let line_limit = (token_limit / 20).max(50).min(500);

            // Compress logs
            let compressed = compressor::compress_logs(&combined_output, line_limit);
            
            // Print compressed output
            println!("{}", compressed);
            
            // Exit with original process status code
            let exit_code = output.status.code().unwrap_or(0);
            std::process::exit(exit_code);
        }
        Err(e) => {
            eprintln!("❌ Lỗi thực thi lệnh: {}", e);
            std::process::exit(1);
        }
    }
}
