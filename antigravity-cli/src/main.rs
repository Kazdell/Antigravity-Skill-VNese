use clap::{Parser, Subcommand};
use std::process::Command;
use std::path::Path;
use std::fs;

mod compressor;
mod typesense_client;
mod runner;
mod mem0_client;
mod browser_client;

#[derive(Parser)]
#[command(name = "ag")]
#[command(about = "Antigravity CLI - Điều phối trung tâm AI Agent Framework", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Khởi tạo thư mục cấu hình .agents cho dự án mới
    Init {
        #[arg(help = "Đường dẫn thư mục dự án cần khởi tạo")]
        project_path: String,
        #[arg(long, help = "Danh sách công nghệ chính (ví dụ: react,nodejs)")]
        stack: String,
    },
    /// Tải và nạp kỹ năng công nghệ mới vào dự án
    Add {
        #[arg(help = "Đường dẫn thư mục dự án")]
        project_path: String,
        #[arg(long, help = "Tên kỹ năng công nghệ cần nạp")]
        tech: String,
    },
    /// Đánh chỉ mục tài liệu và kỹ năng vào Typesense Server
    Index {
        #[arg(long, default_value = ".", help = "Đường dẫn thư mục workspace")]
        workspace: String,
    },
    /// Tìm kiếm tài liệu, kỹ năng và rules (typo-tolerant)
    Search {
        #[arg(help = "Từ khóa tìm kiếm")]
        query: String,
        #[arg(long, default_value = ".", help = "Đường dẫn thư mục workspace")]
        workspace: String,
    },
    /// Quét an toàn và kiểm duyệt các file kỹ năng AI Agent
    Audit {
        #[arg(long, help = "Quét toàn bộ kho kỹ năng")]
        all: bool,
        #[arg(long, help = "Chỉ quét một file cụ thể")]
        file: OptionalString,
        #[arg(long, help = "Chỉ quét các kỹ năng thay đổi trong git (mặc định)")]
        git_diff: bool,
        #[arg(long, default_value = ".", help = "Đường dẫn thư mục workspace")]
        workspace: String,
    },
    /// Chạy lệnh terminal và nén logs tự động để tiết kiệm token
    Run {
        #[arg(help = "Lệnh terminal cần chạy")]
        cmd: String,
        #[arg(long, default_value = "3000", help = "Giới hạn độ dài logs trả về (token limit)")]
        token_limit: usize,
    },
    /// Nâng cấp dự án cũ lên phiên bản Antigravity mới nhất (v5.1)
    Upgrade {
        #[arg(default_value = ".", help = "Đường dẫn dự án cần nâng cấp")]
        target_path: String,
    },
}

type OptionalString = Option<String>;

fn main() {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Init { project_path, stack } => {
            println!("🔄 Đang khởi tạo dự án tại: {}", project_path);
            let status = Command::new("python")
                .arg("C:\\Users\\ACER\\.gemini\\antigravity\\scripts\\agent_manager.py")
                .arg("init")
                .arg(project_path)
                .arg("--stack")
                .arg(stack)
                .status();
                
            match status {
                Ok(s) if s.success() => println!("✅ Khởi tạo dự án thành công!"),
                _ => {
                    eprintln!("❌ Lỗi khi khởi tạo dự án.");
                    std::process::exit(1);
                }
            }
        }
        Commands::Add { project_path, tech } => {
            println!("🔄 Đang thêm kỹ năng {} vào dự án...", tech);
            let status = Command::new("python")
                .arg("C:\\Users\\ACER\\.gemini\\antigravity\\scripts\\agent_manager.py")
                .arg("add")
                .arg(project_path)
                .arg("--tech")
                .arg(tech)
                .status();
                
            match status {
                Ok(s) if s.success() => println!("✅ Đã nạp kỹ năng thành công!"),
                _ => {
                    eprintln!("❌ Lỗi khi nạp kỹ năng.");
                    std::process::exit(1);
                }
            }
        }
        Commands::Index { workspace } => {
            println!("🔄 Đang lập chỉ mục tài liệu trong {} lên Typesense...", workspace);
            let script_path = Path::new(workspace).join(".agents").join("scripts").join("typesense_indexer.py");
            
            let status = Command::new("python")
                .arg(script_path)
                .arg("--index")
                .arg("--workspace")
                .arg(workspace)
                .status();
                
            match status {
                Ok(s) if s.success() => println!("✅ Đã hoàn tất lập chỉ mục tài liệu!"),
                _ => {
                    eprintln!("❌ Lỗi khi lập chỉ mục tài liệu.");
                    std::process::exit(1);
                }
            }
        }
        Commands::Search { query, workspace } => {
            mem0_client::ensure_running();
            browser_client::ensure_running();
            typesense_client::search(query, workspace);
        }
        Commands::Audit { all, file, git_diff, workspace } => {
            let script_path = Path::new(workspace).join(".agents").join("scripts").join("audit_skills.py");
            
            let mut cmd = Command::new("python");
            cmd.arg(script_path).arg("--workspace").arg(workspace);
            
            if *all {
                cmd.arg("--all");
            } else if let Some(f) = file {
                cmd.arg("--file").arg(f);
            } else if *git_diff {
                cmd.arg("--git-diff");
            } else {
                cmd.arg("--git-diff");
            }
            
            let status = cmd.status();
            match status {
                Ok(s) => std::process::exit(s.code().unwrap_or(1)),
                _ => {
                    eprintln!("❌ Lỗi khi thực thi quét bảo mật.");
                    std::process::exit(1);
                }
            }
        }
        Commands::Run { cmd, token_limit } => {
            mem0_client::ensure_running();
            browser_client::ensure_running();
            runner::run_command(cmd, *token_limit);
        }
        Commands::Upgrade { target_path } => {
            println!("🔄 Bắt đầu nâng cấp dự án tại: {}", target_path);
            let target = Path::new(target_path).canonicalize().unwrap_or_else(|_| Path::new(target_path).to_path_buf());
            
            // 1. Create directories
            let scripts_dir = target.join(".agents").join("scripts");
            let bin_dir = target.join("bin");
            
            if let Err(e) = fs::create_dir_all(&scripts_dir) {
                eprintln!("❌ Lỗi tạo thư mục scripts: {}", e);
                std::process::exit(1);
            }
            if let Err(e) = fs::create_dir_all(&bin_dir) {
                eprintln!("❌ Lỗi tạo thư mục bin: {}", e);
                std::process::exit(1);
            }

            // 2. Read and copy templates from current workspace
            let source_root = Path::new(".");
            let files_to_copy = vec![
                (".agents/scripts/typesense_indexer.py", ".agents/scripts/typesense_indexer.py"),
                (".agents/scripts/audit_skills.py", ".agents/scripts/audit_skills.py"),
                (".agents/scripts/run.py", ".agents/scripts/run.py"),
                (".agents/scripts/install_git_hooks.py", ".agents/scripts/install_git_hooks.py"),
                ("docker-compose.yml", "docker-compose.yml"),
            ];

            for (src_rel, dest_rel) in files_to_copy {
                let src_path = source_root.join(src_rel);
                let dest_path = target.join(dest_rel);
                
                if src_path.exists() {
                    if let Err(e) = fs::copy(&src_path, &dest_path) {
                        eprintln!("❌ Lỗi copy tệp {}: {}", src_rel, e);
                    } else {
                        println!("   -> Copy thành công: {}", src_rel);
                    }
                } else {
                    eprintln!("⚠️ Không tìm thấy tệp mẫu: {}", src_path.display());
                }
            }

            // 3. Copy ag binary
            if let Ok(current_exe) = std::env::current_exe() {
                let dest_exe = bin_dir.join("ag.exe");
                if let Err(e) = fs::copy(&current_exe, &dest_exe) {
                    eprintln!("❌ Lỗi copy binary ag.exe: {}", e);
                } else {
                    println!("   -> Copy binary ag.exe thành công.");
                }
            }

            // 4. Run git hooks setup at target
            println!("🔄 Đang cài đặt Git hook tại dự án đích...");
            let status = Command::new("python")
                .arg(".agents/scripts/install_git_hooks.py")
                .current_dir(&target)
                .status();
            match status {
                Ok(s) if s.success() => println!("   -> Git Hook cài đặt thành công."),
                _ => eprintln!("⚠️ Lỗi khi cài đặt Git Hook tại dự án đích."),
            }

            // 5. Start Docker Typesense at target
            println!("🔄 Đang khởi chạy Typesense Server tại dự án đích...");
            let status = Command::new("docker-compose")
                .args(["up", "-d"])
                .current_dir(&target)
                .status();
            match status {
                Ok(s) if s.success() => println!("   -> Khởi chạy Typesense Server thành công."),
                _ => eprintln!("⚠️ Lỗi khi khởi chạy Typesense Server (Vui lòng đảm bảo Docker Desktop đang chạy)."),
            }

            // 6. Run indexer at target
            println!("🔄 Đang lập chỉ mục tài liệu tại dự án đích...");
            let status = Command::new("python")
                .arg(".agents/scripts/typesense_indexer.py")
                .arg("--index")
                .arg("--workspace")
                .arg(".")
                .current_dir(&target)
                .status();
            match status {
                Ok(s) if s.success() => println!("   -> Lập chỉ mục thành công."),
                _ => eprintln!("⚠️ Lỗi khi lập chỉ mục tài liệu tại dự án đích."),
            }

            println!("✨ BÀN GIAO NÂNG CẤP DỰ ÁN THÀNH CÔNG! ✨");
        }
    }
}
