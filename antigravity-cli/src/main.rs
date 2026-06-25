use clap::{Parser, Subcommand};
use std::path::Path;

mod compressor;
mod typesense_client;
mod runner;
mod mem0_client;
mod browser_client;
mod project;
mod audit;
mod verify;
mod icons;

use crate::icons::*;

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
        file: Option<String>,
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
    /// Chạy toàn bộ suite kiểm thử và xác thực tích hợp
    Verify {
        #[arg(long, default_value = ".", help = "Đường dẫn thư mục workspace")]
        workspace: String,
        #[arg(long, help = "URL để chạy E2E và Lighthouse tests")]
        url: Option<String>,
        #[arg(long, help = "Bỏ qua các kịch bản kiểm thử E2E")]
        no_e2e: bool,
        #[arg(long, help = "Dừng suite xác thực ngay khi gặp lỗi nghiêm trọng")]
        stop_on_fail: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Init { project_path, stack } => {
            println!("{} Đang khởi tạo dự án tại: {}", ICON_PROGRESS, project_path);
            match project::initialize_project(project_path, stack) {
                Ok(_) => println!("{} Khởi tạo dự án thành công!", ICON_SUCCESS),
                Err(e) => {
                    eprintln!("{} Lỗi khởi tạo dự án: {}", ICON_ERROR, e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Add { project_path, tech } => {
            println!("{} Đang thêm kỹ năng {} vào dự án...", ICON_PROGRESS, tech);
            match project::add_skills(project_path, tech) {
                Ok(_) => println!("{} Đã nạp kỹ năng thành công!", ICON_SUCCESS),
                Err(e) => {
                    eprintln!("{} Lỗi nạp kỹ năng: {}", ICON_ERROR, e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Index { workspace } => {
            println!("{} Đang lập chỉ mục tài liệu trong {} lên Typesense...", ICON_PROGRESS, workspace);
            if typesense_client::run_indexer(workspace) {
                println!("{} Đã hoàn tất lập chỉ mục tài liệu!", ICON_SUCCESS);
            } else {
                eprintln!("{} Lỗi khi lập chỉ mục tài liệu.", ICON_ERROR);
                std::process::exit(1);
            }
        }
        Commands::Search { query, workspace } => {
            mem0_client::ensure_running();
            browser_client::ensure_running();
            typesense_client::search(query, workspace);
        }
        Commands::Audit { all, file, git_diff, workspace } => {
            let mode = if *all {
                "all"
            } else if file.is_some() {
                "file"
            } else if *git_diff {
                "git-diff"
            } else {
                "git-diff"
            };

            let passed = audit::run_audit(mode, file.as_deref(), None, workspace);
            if !passed {
                std::process::exit(1);
            }
        }
        Commands::Run { cmd, token_limit } => {
            mem0_client::ensure_running();
            browser_client::ensure_running();
            runner::run_command(cmd, *token_limit);
        }
        Commands::Upgrade { target_path } => {
            let current_exe = std::env::current_exe().unwrap_or_else(|_| Path::new("ag.exe").to_path_buf());
            match project::upgrade_project(target_path, current_exe) {
                Ok(_) => println!("{} Dự án đã được nâng cấp lên Pure Rust v5.1!", ICON_SUCCESS),
                Err(e) => {
                    eprintln!("{} Lỗi khi nâng cấp dự án: {}", ICON_ERROR, e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Verify { workspace, url, no_e2e, stop_on_fail } => {
            let passed = verify::run_verification(workspace, url.as_deref(), *no_e2e, *stop_on_fail);
            if !passed {
                std::process::exit(1);
            }
        }
    }
}

