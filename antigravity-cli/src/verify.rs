use std::path::Path;
use std::process::Command as StdCommand;
use std::time::Instant;
use crate::icons::*;

struct Check {
    name: &'static str,
    script_path: &'static str,
    required: bool,
}

struct Category {
    name: &'static str,
    checks: Vec<Check>,
    requires_url: bool,
}

pub fn run_verification(workspace_dir: &str, url: Option<&str>, no_e2e: bool, stop_on_fail: bool) -> bool {
    println!("\n{} ANTIGRAVITY KIT - FULL VERIFICATION SUITE (RUST BUILTIN)", ICON_PROGRESS);
    println!("Workspace: {}", workspace_dir);
    if let Some(u) = url {
        println!("URL: {}", u);
    }
    println!("----------------------------------------------------------------------\n");

    let suites = vec![
        Category {
            name: "Security",
            requires_url: false,
            checks: vec![
                Check {
                    name: "Security Scan (Builtin)",
                    script_path: "", // Run builtin audit instead of python script
                    required: true,
                },
                Check {
                    name: "Dependency Analysis",
                    script_path: ".agent/skills/vulnerability-scanner/scripts/dependency_analyzer.py",
                    required: false,
                },
            ],
        },
        Category {
            name: "Code Quality",
            requires_url: false,
            checks: vec![
                Check {
                    name: "Lint Check",
                    script_path: ".agent/skills/lint-and-validate/scripts/lint_runner.py",
                    required: true,
                },
                Check {
                    name: "Type Coverage",
                    script_path: ".agent/skills/lint-and-validate/scripts/type_coverage.py",
                    required: false,
                },
            ],
        },
        Category {
            name: "Data Layer",
            requires_url: false,
            checks: vec![
                Check {
                    name: "Schema Validation",
                    script_path: ".agent/skills/database-design/scripts/schema_validator.py",
                    required: false,
                },
            ],
        },
        Category {
            name: "Testing",
            requires_url: false,
            checks: vec![
                Check {
                    name: "Test Suite",
                    script_path: ".agent/skills/testing-patterns/scripts/test_runner.py",
                    required: false,
                },
            ],
        },
        Category {
            name: "UX & Accessibility",
            requires_url: false,
            checks: vec![
                Check {
                    name: "UX Audit",
                    script_path: ".agent/skills/frontend-design/scripts/ux_audit.py",
                    required: false,
                },
                Check {
                    name: "Accessibility Check",
                    script_path: ".agent/skills/frontend-design/scripts/accessibility_checker.py",
                    required: false,
                },
            ],
        },
        Category {
            name: "SEO & Content",
            requires_url: false,
            checks: vec![
                Check {
                    name: "SEO Check",
                    script_path: ".agent/skills/seo-fundamentals/scripts/seo_checker.py",
                    required: false,
                },
                Check {
                    name: "GEO Check",
                    script_path: ".agent/skills/geo-fundamentals/scripts/geo_checker.py",
                    required: false,
                },
            ],
        },
        Category {
            name: "Performance",
            requires_url: true,
            checks: vec![
                Check {
                    name: "Lighthouse Audit",
                    script_path: ".agent/skills/performance-profiling/scripts/lighthouse_audit.py",
                    required: true,
                },
                Check {
                    name: "Bundle Analysis",
                    script_path: ".agent/skills/performance-profiling/scripts/bundle_analyzer.py",
                    required: false,
                },
            ],
        },
        Category {
            name: "E2E Testing",
            requires_url: true,
            checks: vec![
                Check {
                    name: "Playwright E2E",
                    script_path: "",
                    required: false,
                },
            ],
        },
        Category {
            name: "Mobile",
            requires_url: false,
            checks: vec![
                Check {
                    name: "Mobile Audit",
                    script_path: ".agent/skills/mobile-design/scripts/mobile_audit.py",
                    required: false,
                },
            ],
        },
        Category {
            name: "Internationalization",
            requires_url: false,
            checks: vec![
                Check {
                    name: "i18n Check",
                    script_path: ".agent/skills/i18n-localization/scripts/i18n_checker.py",
                    required: false,
                },
            ],
        },
    ];

    let start_time = Instant::now();
    let mut total_checks = 0;
    let mut passed_checks = 0;
    let mut failed_checks = 0;
    let mut skipped_checks = 0;
    let mut results = Vec::new();

    for cat in suites {
        if cat.requires_url && url.is_none() {
            continue;
        }
        if no_e2e && cat.name == "E2E Testing" {
            continue;
        }

        println!("{} CATEGORY: {}", ICON_REPORT, cat.name.to_uppercase());
        println!("{}", "-".repeat(40));

        for check in cat.checks {
            total_checks += 1;

            if check.name == "Security Scan (Builtin)" {
                println!("{} Running: {}", ICON_PROGRESS, check.name);
                let check_start = Instant::now();
                let passed = crate::audit::run_audit("all", None, None, workspace_dir);
                let duration = check_start.elapsed().as_secs_f32();

                if passed {
                    println!("{} {}: PASSED ({:.1}s)", ICON_SUCCESS, check.name, duration);
                    passed_checks += 1;
                    results.push((check.name, true, false, duration));
                } else {
                    println!("{} {}: FAILED ({:.1}s)", ICON_ERROR, check.name, duration);
                    failed_checks += 1;
                    results.push((check.name, false, false, duration));

                    if stop_on_fail && check.required {
                        println!("\n{} CRITICAL: Lỗi nghiêm trọng tại {}. Dừng suite xác thực.", ICON_ERROR, check.name);
                        return false;
                    }
                }
                continue;
            }

            if check.name == "Playwright E2E" {
                println!("{} Running: {}", ICON_PROGRESS, check.name);
                let check_start = Instant::now();
                
                let mut cmd = if cfg!(target_os = "windows") {
                    let mut c = StdCommand::new("bin\\ag.exe");
                    if let Some(u) = url {
                        c.args(["qa", u]);
                    }
                    c
                } else {
                    let mut c = StdCommand::new("bin/ag");
                    if let Some(u) = url {
                        c.args(["qa", u]);
                    }
                    c
                };

                let status = cmd.status();
                let duration = check_start.elapsed().as_secs_f32();

                match status {
                    Ok(s) if s.success() => {
                        println!("{} {}: PASSED ({:.1}s)", ICON_SUCCESS, check.name, duration);
                        passed_checks += 1;
                        results.push((check.name, true, false, duration));
                    }
                    _ => {
                        println!("{} {}: FAILED ({:.1}s)", ICON_ERROR, check.name, duration);
                        failed_checks += 1;
                        results.push((check.name, false, false, duration));

                        if stop_on_fail && check.required {
                            println!("\n{} CRITICAL: Lỗi nghiêm trọng tại {}. Dừng suite xác thực.", ICON_ERROR, check.name);
                            return false;
                        }
                    }
                }
                continue;
            }

            let script = Path::new(workspace_dir).join(check.script_path);
            if !script.exists() {
                println!("{} {}: Không tìm thấy script, bỏ qua", ICON_SKIP, check.name);
                skipped_checks += 1;
                results.push((check.name, true, true, 0.0));
                continue;
            }

            println!("{} Running: {}", ICON_PROGRESS, check.name);
            let check_start = Instant::now();

            let mut cmd = StdCommand::new("python");
            cmd.arg(&script).arg(workspace_dir);
            if let Some(u) = url {
                let name_lower = check.name.to_lowercase();
                if name_lower.contains("lighthouse") || name_lower.contains("playwright") {
                    cmd.arg(u);
                }
            }

            let status = cmd.status();
            let duration = check_start.elapsed().as_secs_f32();

            match status {
                Ok(s) if s.success() => {
                    println!("{} {}: PASSED ({:.1}s)", ICON_SUCCESS, check.name, duration);
                    passed_checks += 1;
                    results.push((check.name, true, false, duration));
                }
                _ => {
                    println!("{} {}: FAILED ({:.1}s)", ICON_ERROR, check.name, duration);
                    failed_checks += 1;
                    results.push((check.name, false, false, duration));

                    if stop_on_fail && check.required {
                        println!("\n{} CRITICAL: Lỗi nghiêm trọng tại {}. Dừng suite xác thực.", ICON_ERROR, check.name);
                        return false;
                    }
                }
            }
        }
        println!();
    }

    let total_duration = start_time.elapsed().as_secs_f32();
    println!("{}", "=".repeat(70));
    println!("{} BÁO CÁO XÁC THỰC TOÀN DIỆN", ICON_REPORT);
    println!("{}", "=".repeat(70));
    println!("Thời gian thực thi: {:.1}s", total_duration);
    println!("Tổng số bài kiểm tra: {}", total_checks);
    println!("{} Đạt: {}", ICON_SUCCESS, passed_checks);
    println!("{} Lỗi: {}", ICON_ERROR, failed_checks);
    println!("{} Bỏ qua: {}", ICON_SKIP, skipped_checks);
    println!("{}", "=".repeat(70));

    for (name, passed, skipped, duration) in results {
        let status_str = if skipped { ICON_SKIP } else if passed { ICON_SUCCESS } else { ICON_ERROR };
        let dur_str = if skipped { "".to_string() } else { format!(" ({:.1}s)", duration) };
        println!("  [{}] {}{}", status_str, name, dur_str);
    }
    println!("{}", "=".repeat(70));

    failed_checks == 0
}
