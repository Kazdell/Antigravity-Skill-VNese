use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, Write};
use std::collections::HashSet;
use crate::icons::*;

const GLOBAL_REPO: &str = r"C:\Users\ACER\Downloads\Antigravity-Skill-VNese";

// Copy directory recursively
pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn get_tech_mapping(tech: &str) -> Option<&'static str> {
    match tech.trim().to_lowercase().as_str() {
        "react" | "nextjs" | "angular" | "ui" | "ux" | "tailwind" | "css" | "html" | "frontend" | "desktop" | "avalonia" | "wpf" | "winforms" | "javascript" | "typescript" => Some("mk01"),
        "nodejs" | "dotnet" | "c#" | "csharp" | "python" | "go" | "rust" | "api" | "database" | "sql" | "postgres" | "mysql" | "backend" | "mongodb" | "redis" => Some("mk02"),
        "docker" | "kubernetes" | "k8s" | "cicd" | "aws" | "azure" | "terraform" | "devops" | "infrastructure" | "gcp" => Some("mk03"),
        "security" | "pentest" | "owasp" | "vulnerability" | "compliance" => Some("mk04"),
        "testing" | "tdd" | "e2e" | "unit-test" | "quality" | "audit" => Some("mk05"),
        "ai" | "llm" | "rag" | "ml" | "data-science" => Some("mk06"),
        "system" | "shell" | "git" | "bash" | "powershell" => Some("mk08"),
        "mobile" | "react-native" | "flutter" | "ios" | "android" => Some("mk09"),
        "mcp" | "integration" | "browser-automation" => Some("mk11"),
        "architecture" | "design-patterns" | "planning" => Some("mk12"),
        "orchestration" | "workflow" | "multi-agent" => Some("mk13"),
        "debug" | "troubleshoot" | "migration" => Some("mk14"),
        "seo" | "marketing" | "growth" => Some("mk15"),
        "localization" | "i18n" => Some("mk16"),
        "game" | "godot" | "unity" | "unreal" => Some("mk17"),
        "saas" | "crm" | "hubspot" | "automation" => Some("mk19"),
        "wordpress" | "odoo" | "erp" | "cms" => Some("mk20"),
        _ => None,
    }
}

pub fn initialize_project(project_path: &str, stack: &str) -> io::Result<()> {
    let target_agents_dir = Path::new(project_path).join(".agents");
    println!("{} Đang khởi tạo cấu trúc agents tại: {}", ICON_PROGRESS, target_agents_dir.display());

    // 1. Copy base folders
    let base_folders = vec!["rules", "workflows", ".shared"];
    for folder in base_folders {
        let src = Path::new(GLOBAL_REPO).join(folder);
        let dst = target_agents_dir.join(folder);
        if src.exists() {
            println!("   -> Sao chép thư mục: {}", folder);
            if let Err(e) = copy_dir_all(&src, &dst) {
                eprintln!("{} Cảnh báo: Không thể sao chép thư mục {}: {}", ICON_WARNING, folder, e);
            }
        }
    }

    // 2. Setup skills based on stack
    let target_skills_dir = target_agents_dir.join("skills");
    fs::create_dir_all(&target_skills_dir)?;

    let mut selected_specialists = HashSet::new();
    // Default core specialists
    selected_specialists.insert("mk08".to_string());
    selected_specialists.insert("mk12".to_string());
    selected_specialists.insert("mk14".to_string());

    for tech in stack.split(',') {
        if let Some(spec) = get_tech_mapping(tech) {
            selected_specialists.insert(spec.to_string());
        }
    }

    for mk in &selected_specialists {
        let src = Path::new(GLOBAL_REPO).join("skills").join(mk);
        let dst = target_skills_dir.join(mk);
        if src.exists() {
            println!("   -> Kích hoạt kỹ năng specialist: {}", mk);
            if let Err(e) = copy_dir_all(&src, &dst) {
                eprintln!("{} Cảnh báo: Không thể sao chép kỹ năng {}: {}", ICON_WARNING, mk, e);
            }
        }
    }

    // 3. Install git hooks
    if let Err(e) = install_git_hooks(project_path) {
        eprintln!("{} Cảnh báo: Không thể cài đặt git hooks: {}", ICON_WARNING, e);
    }

    println!("\n{} Khởi tạo cấu trúc .agents thành công!", ICON_SUCCESS);
    println!("Các specialist đã kích hoạt: {:?}", selected_specialists);
    Ok(())
}

pub fn add_skills(project_path: &str, tech_list: &str) -> io::Result<()> {
    let target_skills_dir = Path::new(project_path).join(".agents").join("skills");
    if !target_skills_dir.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Dự án chưa được khởi tạo thư mục .agents. Vui lòng chạy lệnh 'ag init' trước.",
        ));
    }

    let mut added = Vec::new();
    for tech in tech_list.split(',') {
        let tech_clean = tech.trim();
        if let Some(mk) = get_tech_mapping(tech_clean) {
            let src = Path::new(GLOBAL_REPO).join("skills").join(mk);
            let dst = target_skills_dir.join(mk);
            if src.exists() {
                if !dst.exists() {
                    println!("   -> Thêm kỹ năng specialist cho '{}': {}", tech_clean, mk);
                    copy_dir_all(&src, &dst)?;
                    added.push(format!("{} ({})", tech_clean, mk));
                } else {
                    println!("   -> Kỹ năng specialist cho '{}' ({}) đã tồn tại.", tech_clean, mk);
                }
            } else {
                eprintln!("{} Cảnh báo: Thư mục kỹ năng {} không tồn tại trong repo gốc.", ICON_WARNING, mk);
            }
        } else {
            eprintln!("{} Cảnh báo: Công nghệ '{}' chưa được hỗ trợ ánh xạ.", ICON_WARNING, tech_clean);
        }
    }

    if !added.is_empty() {
        println!("\n{} Đã thêm thành công kỹ năng: {}", ICON_SUCCESS, added.join(", "));
    } else {
        println!("\n{} Không có kỹ năng mới nào được thêm vào.", ICON_INFO);
    }
    Ok(())
}

pub fn install_git_hooks(workspace_dir: &str) -> io::Result<()> {
    let git_dir = Path::new(workspace_dir).join(".git");
    if !git_dir.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Thư mục .git không tồn tại. Lệnh phải chạy từ thư mục gốc của Git repository.",
        ));
    }

    let hooks_dir = git_dir.join("hooks");
    fs::create_dir_all(&hooks_dir)?;

    let pre_commit_path = hooks_dir.join("pre-commit");

    let hook_content = r#"#!/bin/sh
echo "[Git Hook] Đang tự động quét bảo mật các kỹ năng AI Agent qua Rust CLI..."

# Choose ag binary path
if [ -f "./bin/ag.exe" ]; then
    ./bin/ag.exe audit --git-diff --workspace .
elif [ -f "./bin/ag" ]; then
    ./bin/ag audit --git-diff --workspace .
else
    echo "[Git Hook Warning] Không tìm thấy ag binary tại ./bin/ag. Bỏ qua quét bảo mật."
    exit 0
fi

if [ $? -ne 0 ]; then
    echo "[Git Hook] LỖI BẢO MẬT: Commit bị chặn do phát hiện rủi ro cao trong tệp kỹ năng AI!"
    exit 1
fi

echo "[Git Hook] Quét bảo mật hoàn tất. Tệp kỹ năng an toàn."
exit 0
"#;

    let mut file = fs::File::create(&pre_commit_path)?;
    file.write_all(hook_content.as_bytes())?;

    // On Unix, set permissions to make it executable (0o755)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&pre_commit_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&pre_commit_path, perms)?;
    }

    println!("{} Git pre-commit hook đã được cài đặt tại: {}", ICON_SUCCESS, pre_commit_path.display());
    Ok(())
}

pub fn upgrade_project(target_path: &str, current_exe: PathBuf) -> io::Result<()> {
    let target = Path::new(target_path).canonicalize().unwrap_or_else(|_| Path::new(target_path).to_path_buf());
    println!("{} Bắt đầu nâng cấp dự án tại: {}", ICON_PROGRESS, target.display());

    let target_agents_dir = target.join(".agents");
    let bin_dir = target.join("bin");

    fs::create_dir_all(&target_agents_dir)?;
    fs::create_dir_all(&bin_dir)?;

    // 1. Copy base folders
    let base_folders = vec!["rules", "workflows", ".shared"];
    for folder in base_folders {
        let src = Path::new(GLOBAL_REPO).join(folder);
        let dst = target_agents_dir.join(folder);
        if src.exists() {
            println!("   -> Cập nhật thư mục: {}", folder);
            copy_dir_all(&src, &dst)?;
        }
    }

    // 2. Copy docker-compose
    let docker_src = Path::new(GLOBAL_REPO).join("docker-compose.yml");
    let docker_dst = target.join("docker-compose.yml");
    if docker_src.exists() {
        println!("   -> Cập nhật docker-compose.yml");
        fs::copy(docker_src, docker_dst)?;
    }

    // 3. Copy ag binary
    let dest_exe = bin_dir.join("ag.exe");
    println!("   -> Sao chép binary ag.exe");
    fs::copy(&current_exe, &dest_exe)?;

    // 4. Install Git hook at target
    println!("{} Đang cài đặt Git hook tại dự án đích...", ICON_PROGRESS);
    if let Err(e) = install_git_hooks(&target.to_string_lossy()) {
        eprintln!("{} Cảnh báo: Không thể cài đặt Git hook tại dự án đích: {}", ICON_WARNING, e);
    }

    // 5. Start Docker Typesense at target
    println!("{} Đang khởi chạy Typesense Server tại dự án đích...", ICON_PROGRESS);
    let status = std::process::Command::new("docker-compose")
        .args(["up", "-d"])
        .current_dir(&target)
        .status();

    match status {
        Ok(s) if s.success() => println!("   -> Khởi chạy Typesense Server thành công."),
        _ => eprintln!("{} Cảnh báo: Lỗi khi khởi chạy Typesense Server (Đảm bảo Docker Desktop đang chạy).", ICON_WARNING),
    }

    println!("\n{} BÀN GIAO NÂNG CẤP DỰ ÁN THÀNH CÔNG! {}", ICON_SPARKLES, ICON_SPARKLES);
    Ok(())
}
