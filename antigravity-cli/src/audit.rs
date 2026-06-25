use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command as StdCommand;
use regex::Regex;
use serde_json::Value;
use crate::icons::*;

// Vulnerability patterns for Static Analysis
struct Pattern {
    regex: Regex,
    desc: &'static str,
    finding_type: &'static str,
    deduction: i32,
}

fn get_patterns() -> Vec<Pattern> {
    vec![
        // Dangerous Shell
        Pattern {
            regex: Regex::new(r"\brm\s+-rf\b").unwrap(),
            desc: "Sử dụng lệnh xóa cưỡng ép rm -rf",
            finding_type: "DANGEROUS_SHELL",
            deduction: 20,
        },
        Pattern {
            regex: Regex::new(r"\bDROP\s+DATABASE\b").unwrap(),
            desc: "Lệnh xóa cơ sở dữ liệu DROP DATABASE",
            finding_type: "DANGEROUS_SHELL",
            deduction: 20,
        },
        Pattern {
            regex: Regex::new(r"\bDROP\s+TABLE\b").unwrap(),
            desc: "Lệnh xóa bảng DROP TABLE",
            finding_type: "DANGEROUS_SHELL",
            deduction: 20,
        },
        Pattern {
            regex: Regex::new(r"\bformat\s+[a-zA-Z]:").unwrap(),
            desc: "Lệnh format ổ đĩa Windows",
            finding_type: "DANGEROUS_SHELL",
            deduction: 20,
        },
        Pattern {
            regex: Regex::new(r"\bdel\s+/s\b").unwrap(),
            desc: "Lệnh xóa tệp cưỡng ép trên Windows del /s",
            finding_type: "DANGEROUS_SHELL",
            deduction: 20,
        },
        Pattern {
            regex: Regex::new(r"\bshutdown\s+/s\b").unwrap(),
            desc: "Lệnh tắt máy hệ thống Windows",
            finding_type: "DANGEROUS_SHELL",
            deduction: 20,
        },
        // Secret Leakage
        Pattern {
            regex: Regex::new(r#"(?i)\b(api_key|apikey|password|secret|passwd|token|private_key)\s*=\s*['"][a-zA-Z0-9_\-]{16,}['"]"#).unwrap(),
            desc: "Phát hiện lộ lọt Secrets/Hardcoded Keys",
            finding_type: "SECRET_LEAK",
            deduction: 15,
        },
        Pattern {
            regex: Regex::new(r#"(?i)\b(bearer\s+[a-zA-Z0-9_\-\.]{20,})['"]"#).unwrap(),
            desc: "Phát hiện lộ lọt Bearer Token",
            finding_type: "SECRET_LEAK",
            deduction: 15,
        },
        // Prompt Injection
        Pattern {
            regex: Regex::new(r"(?i)\b(ignore\s+all\s+rules|bỏ\s+qua\s+mọi\s+quy\s+tắc|override\s+gemini\.md|ghi\s+đè\s+gemini\.md|quên\s+hướng\s+dẫn\s+trước)\b").unwrap(),
            desc: "Phát hiện dấu hiệu Prompt Injection nhằm thay đổi hiến pháp/luật Agent",
            finding_type: "PROMPT_INJECTION",
            deduction: 15,
        },
        Pattern {
            regex: Regex::new(r"(?i)\b(you\s+are\s+no\s+longer\s+an\s+ai|bạn\s+không\s+còn\s+là\s+ai|từ\s+nay\s+hãy\s+đóng\s+vai)\b").unwrap(),
            desc: "Phát hiện dấu hiệu Jailbreak/Prompt Injection",
            finding_type: "PROMPT_INJECTION",
            deduction: 15,
        },
    ]
}

#[derive(Debug)]
pub struct Finding {
    pub filepath: String,
    pub line: usize,
    pub finding_type: String,
    pub detail: String,
    pub deduction: i32,
}

pub fn scan_text_file(filepath: &Path, workspace_dir: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    let text = match fs::read_to_string(filepath) {
        Ok(t) => t,
        Err(e) => {
            findings.push(Finding {
                filepath: filepath.to_string_lossy().to_string(),
                line: 0,
                finding_type: "FILE_ERROR".to_string(),
                detail: format!("Không thể đọc file: {}", e),
                deduction: 5,
            });
            return findings;
        }
    };

    let is_md = filepath.extension().map_or(false, |ext| ext == "md");
    let rel_path = filepath
        .strip_prefix(workspace_dir)
        .unwrap_or(filepath)
        .to_string_lossy()
        .replace('\\', "/");

    let patterns = get_patterns();

    for (lineno, line) in text.lines().enumerate() {
        let lineno = lineno + 1;
        if line.contains("nosec") || line.contains("no-audit") {
            continue;
        }

        for pattern in patterns.iter() {
            if pattern.regex.is_match(line) {
                let f_type = if is_md && pattern.finding_type == "DANGEROUS_SHELL" {
                    "DOCUMENT_SHELL_WARNING"
                } else {
                    pattern.finding_type
                };

                let deduction = if f_type == "DOCUMENT_SHELL_WARNING" { 1 } else { pattern.deduction };

                findings.push(Finding {
                    filepath: rel_path.clone(),
                    line: lineno,
                    finding_type: f_type.to_string(),
                    detail: format!("{}: '{}'", pattern.desc, line.trim()),
                    deduction,
                });
            }
        }
    }

    findings
}

pub fn scan_ui_file(filepath: &Path, workspace_dir: &str) -> Vec<Finding> {
    let mut findings = scan_text_file(filepath, workspace_dir);

    // Call Impeccable Node.js linter offline with --json
    let rel_path = filepath
        .strip_prefix(workspace_dir)
        .unwrap_or(filepath)
        .to_string_lossy()
        .replace('\\', "/");

    // Path to cli.js of Impeccable
    let linter_path = Path::new(workspace_dir).join("scratch").join("impeccable-src").join("cli").join("bin").join("cli.js");
    if !linter_path.exists() {
        // Fallback: search in active workspace root
        return findings;
    }

    let output = StdCommand::new("node")
        .arg(&linter_path)
        .arg("detect")
        .arg("--json")
        .arg(filepath)
        .output();

    if let Ok(out) = output {
        let stdout_str = String::from_utf8_lossy(&out.stdout);
        if let Ok(json_val) = serde_json::from_str::<Value>(&stdout_str) {
            if let Some(arr) = json_val.as_array() {
                for item in arr {
                    let antipattern = item.get("antipattern").and_then(|v| v.as_str()).unwrap_or("unknown-ui-slop");
                    let description = item.get("description").and_then(|v| v.as_str()).unwrap_or("Lỗi thiết kế UI/UX");
                    let line = item.get("line").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
                    let snippet = item.get("snippet").and_then(|v| v.as_str()).unwrap_or("");

                    findings.push(Finding {
                        filepath: rel_path.clone(),
                        line,
                        finding_type: "UI_DESIGN_SLOP".to_string(),
                        detail: format!("[{}] {} (Đoạn mã: '{}')", antipattern, description, snippet),
                        deduction: 5,
                    });
                }
            }
        }
    }

    findings
}

fn get_git_diff_files(workspace_dir: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();

    let run_git = |args: &[&str]| -> Option<String> {
        let out = StdCommand::new("git")
            .args(args)
            .current_dir(workspace_dir)
            .output()
            .ok()?;
        if out.status.success() {
            Some(String::from_utf8_lossy(&out.stdout).to_string())
        } else {
            None
        }
    };

    let mut candidate_files = std::collections::HashSet::new();

    if let Some(staged) = run_git(&["diff", "--name-only", "--cached"]) {
        for line in staged.lines() {
            if !line.trim().is_empty() {
                candidate_files.insert(line.trim().to_string());
            }
        }
    }

    if let Some(unstaged) = run_git(&["diff", "--name-only"]) {
        for line in unstaged.lines() {
            if !line.trim().is_empty() {
                candidate_files.insert(line.trim().to_string());
            }
        }
    }

    if let Some(untracked) = run_git(&["status", "--porcelain"]) {
        for line in untracked.lines() {
            if line.starts_with("?? ") {
                candidate_files.insert(line[3..].trim().to_string());
            }
        }
    }

    for f in candidate_files {
        let filepath = Path::new(workspace_dir).join(&f);
        let rel_normalized = f.replace('\\', "/");
        // Only scan skill files or markdown files in .agents/
        if (rel_normalized.contains("skills/") || rel_normalized.contains(".agents/")) && filepath.is_file() {
            files.push(filepath);
        }
    }

    files
}

pub fn run_audit(mode: &str, target_file: Option<&str>, target_dir: Option<&str>, workspace_dir: &str) -> bool {
    let mut files_to_scan = Vec::new();

    match mode {
        "file" => {
            if let Some(tf) = target_file {
                let p = Path::new(tf);
                if p.exists() {
                    files_to_scan.push(p.to_path_buf());
                } else {
                    eprintln!("{} Lỗi: Tệp tin không tồn tại: {}", ICON_ERROR, tf);
                    return false;
                }
            }
        }
        "dir" => {
            if let Some(td) = target_dir {
                let p = Path::new(td);
                if p.exists() {
                    let mut dirs = vec![p.to_path_buf()];
                    while let Some(dir) = dirs.pop() {
                        if let Ok(entries) = fs::read_dir(dir) {
                            for entry in entries.flatten() {
                                let path = entry.path();
                                if path.is_dir() {
                                    dirs.push(path);
                                } else if path.is_file() {
                                    let ext = path.extension().map(|e| e.to_string_lossy().to_string());
                                    if let Some(ref e) = ext {
                                        if vec!["md", "py", "sh", "ps1", "html", "css", "js", "jsx", "tsx"].contains(&e.as_str()) {
                                            files_to_scan.push(path);
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    eprintln!("{} Lỗi: Thư mục không tồn tại: {}", ICON_ERROR, td);
                    return false;
                }
            }
        }
        "git-diff" => {
            files_to_scan = get_git_diff_files(workspace_dir);
            println!("{} Tìm thấy {} tệp tin thay đổi/thêm mới trong Git.", ICON_SEARCH, files_to_scan.len());
        }
        _ => {
            // mode = "all"
            let skills_dir = Path::new(workspace_dir).join(".agents").join("skills");
            let skills_dir = if skills_dir.exists() {
                skills_dir
            } else {
                Path::new(workspace_dir).join("skills") // fallback
            };

            if skills_dir.exists() {
                let mut dirs = vec![skills_dir];
                while let Some(dir) = dirs.pop() {
                    if let Ok(entries) = fs::read_dir(dir) {
                        for entry in entries.flatten() {
                            let path = entry.path();
                            if path.is_dir() {
                                dirs.push(path);
                            } else if path.is_file() {
                                let ext = path.extension().map(|e| e.to_string_lossy().to_string());
                                if let Some(ref e) = ext {
                                    if vec!["md", "py", "sh", "ps1", "html", "css", "js", "jsx", "tsx"].contains(&e.as_str()) {
                                        files_to_scan.push(path);
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                eprintln!("{} Lỗi: Thư mục skills không tồn tại.", ICON_ERROR);
                return false;
            }
        }
    }

    let mut all_findings = Vec::new();
    let mut score_deduction = 0;

    for filepath in &files_to_scan {
        let ext = filepath.extension().map(|e| e.to_string_lossy().to_lowercase());
        let findings = match ext.as_deref() {
            Some("html") | Some("css") | Some("jsx") | Some("tsx") | Some("vue") | Some("astro") => {
                scan_ui_file(filepath, workspace_dir)
            }
            _ => scan_text_file(filepath, workspace_dir),
        };

        for f in findings {
            score_deduction += f.deduction;
            all_findings.push(f);
        }
    }

    println!("\n{}", "=".repeat(70));
    println!("{}  ANTIGRAVITY AGENT SKILLS SECURITY AUDIT REPORT", ICON_SHIELD);
    println!("{}", "=".repeat(70));
    println!("Chế độ quét: {}", mode.to_uppercase());
    println!("Tổng số file đã quét: {}", files_to_scan.len());
    println!("Tổng số lỗi phát hiện: {}", all_findings.len());
    println!("{}\n", "=".repeat(70));

    if !all_findings.is_empty() {
        // Group by filepath for printing
        let mut current_file = String::new();
        for f in &all_findings {
            if f.filepath != current_file {
                current_file = f.filepath.clone();
                println!("{} Tệp: {}", ICON_FILE, current_file);
            }
            println!("   [{}] Dòng {}: {}", f.finding_type, f.line, f.detail);
        }
        println!("{}", "-".repeat(70));
    }

    let risk_score = score_deduction.min(100);
    let security_score = 100 - risk_score;

    println!("{} ĐIỂM SỐ BẢO MẬT (SECURITY SCORE): {}/100", ICON_REPORT, security_score);

    if security_score < 70 {
        println!("\n{} KẾT LUẬN: KHÔNG AN TOÀN (FAILED) - Phát hiện lỗi nghiêm trọng!", ICON_ERROR);
        false
    } else {
        println!("\n{} KẾT LUẬN: AN TOÀN (PASSED)", ICON_SUCCESS);
        true
    }
}
