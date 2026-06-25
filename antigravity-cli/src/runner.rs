use std::process::Command;
use std::io::{self, Write};
use crate::compressor;

fn is_destructive_command(cmd_str: &str) -> bool {
    let cmd_lower = cmd_str.to_lowercase();
    
    // Danh sách các từ khóa phá hủy hệ thống hoặc dữ liệu
    let keywords = vec![
        "rm -rf", "rm -r", "rmdir /s", "del /s", "del /f",
        "drop database", "drop table", "truncate table",
        "git reset --hard", "git push --force", "git push -f",
        "kubectl delete", "docker rm -f", "docker rmi", "docker system prune"
    ];
    
    for kw in keywords {
        if cmd_lower.contains(kw) {
            // Cho phép các trường hợp ngoại lệ (dọn dẹp thư mục build/tạm)
            let allowed_exceptions = vec![
                "node_modules", "target", "dist", ".next", "build", 
                "scratch", "reports", "tmp", "temp", "cache", ".agents/state"
            ];
            
            let mut is_allowed = false;
            for ext in allowed_exceptions {
                if cmd_lower.contains(ext) {
                    is_allowed = true;
                    break;
                }
            }
            
            if !is_allowed {
                return true;
            }
        }
    }
    
    false
}

pub fn run_command(cmd_str: &str, token_limit: usize) {
    // 🛡️ [AG SAFEGUARD] Lưới bảo vệ careful chặn lệnh shell nguy hiểm
    if is_destructive_command(cmd_str) {
        println!("\x1b[31m⚠️  [AG SAFEGUARD - CAREFUL] CẢNH BÁO LỆNH NGUY HIỂM PHÁ HỦY DỮ LIỆU!\x1b[0m");
        println!("\x1b[33mLệnh: {}\x1b[0m", cmd_str);
        print!("Bạn có chắc chắn muốn tiếp tục thực thi? (gõ 'yes' để xác nhận): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let trimmed = input.trim().to_lowercase();
                if trimmed != "yes" && trimmed != "y" {
                    println!("\x1b[31m❌ Lệnh bị hủy bỏ để đảm bảo an toàn hệ thống.\x1b[0m");
                    std::process::exit(1);
                }
                println!("\x1b[32m✔ Đã xác nhận. Tiếp tục thực thi lệnh...\x1b[0m");
            }
            Err(_) => {
                println!("\x1b[31m❌ Không đọc được xác nhận. Hủy lệnh.\x1b[0m");
                std::process::exit(1);
            }
        }
    }

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

            // Estimate line limit from token limit
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
