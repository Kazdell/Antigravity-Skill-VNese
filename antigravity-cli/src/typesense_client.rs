use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::time::Duration;
use crate::icons::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResult {
    pub name: String,
    pub description: String,
    pub filepath: String,
    pub r#type: String,
    pub score: i64,
}

fn parse_markdown_file(filepath: &Path) -> Option<(String, String, String, String)> {
    let text = match fs::read_to_string(filepath) {
        Ok(t) => t,
        Err(_) => return None,
    };

    let mut name = String::new();
    let mut description = String::new();
    let mut trigger = String::new();
    let mut content = text.clone();

    // Check frontmatter
    if text.starts_with("---\n") || text.starts_with("---\r\n") {
        let parts: Vec<&str> = text.splitn(3, "---").collect();
        if parts.len() >= 3 {
            let yaml_block = parts[1];
            content = parts[2].to_string();

            for line in yaml_block.lines() {
                if line.contains(':') {
                    let kv: Vec<&str> = line.splitn(2, ':').collect();
                    let key = kv[0].trim().to_lowercase();
                    let val = kv[1].trim().to_string();
                    match key.as_str() {
                        "name" => name = val,
                        "description" => description = val,
                        "trigger" => trigger = val,
                        _ => {}
                    }
                }
            }
        }
    }

    // Fallback name if empty (first H1)
    if name.is_empty() {
        for line in content.lines() {
            if line.starts_with("# ") {
                name = line[2..].trim().to_string();
                break;
            }
        }
        if name.is_empty() {
            name = filepath.file_name()?.to_string_lossy().to_string();
        }
    }

    // Fallback description if empty (first 200 chars)
    if description.is_empty() {
        let clean: String = content
            .chars()
            .filter(|c| !['#', '*', '`', '_', '-', '\n', '\r'].contains(c))
            .collect();
        let clean: String = clean.split_whitespace().collect::<Vec<&str>>().join(" ");
        description = if clean.chars().count() > 200 {
            let truncated: String = clean.chars().take(200).collect();
            format!("{}...", truncated)
        } else {
            clean
        };
    }

    Some((name, description, trigger, content))
}

fn local_fallback_search(query: &str, workspace_dir: &str) -> Vec<SearchResult> {
    println!("\n{} Typesense Server offline. Đang chuyển sang Local Regex/Grep Search...", ICON_WARNING);
    let agents_dir = Path::new(workspace_dir).join(".agents");
    if !agents_dir.exists() {
        eprintln!("Error: Không tìm thấy thư mục .agents tại {}", workspace_dir);
        return Vec::new();
    }

    let categories = vec!["rules", "skills", "workflows", ".shared"];
    let mut results = Vec::new();
    let q_lower = query.to_lowercase();

    for cat in categories {
        let cat_path = agents_dir.join(cat);
        if !cat_path.exists() {
            continue;
        }

        // Recursively find markdown files
        let mut paths_to_visit = vec![cat_path];
        while let Some(dir_path) = paths_to_visit.pop() {
            if let Ok(entries) = fs::read_dir(dir_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        paths_to_visit.push(path);
                    } else if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                        if let Some((name, desc, trig, content)) = parse_markdown_file(&path) {
                            let mut score = 0;
                            if name.to_lowercase().contains(&q_lower) {
                                score += 10;
                            }
                            if desc.to_lowercase().contains(&q_lower) {
                                score += 5;
                            }
                            if trig.to_lowercase().contains(&q_lower) {
                                score += 8;
                            }
                            if content.to_lowercase().contains(&q_lower) {
                                score += 1;
                            }

                            if score > 0 {
                                // Make relative path
                                let rel_path = path
                                    .strip_prefix(workspace_dir)
                                    .unwrap_or(&path)
                                    .to_string_lossy()
                                    .replace('\\', "/");

                                results.push(SearchResult {
                                    name,
                                    description: desc,
                                    filepath: rel_path,
                                    r#type: cat.to_string(),
                                    score,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    // Sort by score descending
    results.sort_by(|a, b| b.score.cmp(&a.score));
    results.into_iter().take(5).collect()
}

pub fn search(query: &str, workspace_dir: &str) {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(2))
        .build();

    let client = match client {
        Ok(c) => c,
        Err(_) => {
            let res = local_fallback_search(query, workspace_dir);
            print_results(query, res);
            return;
        }
    };

    let url = format!(
        "http://localhost:8108/collections/agents_knowledge/documents/search?q={}&query_by=name,description,trigger,content&per_page=5",
        query
    );

    let response = client
        .get(&url)
        .header("X-TYPESENSE-API-KEY", "antigravity_secret_key_123")
        .send();

    match response {
        Ok(res) if res.status().is_success() => {
            let body: Value = match res.json() {
                Ok(j) => j,
                Err(_) => {
                    let res = local_fallback_search(query, workspace_dir);
                    print_results(query, res);
                    return;
                }
            };

            let mut results = Vec::new();
            if let Some(hits) = body.get("hits").and_then(|h| h.as_array()) {
                for hit in hits {
                    if let (Some(doc), Some(match_score)) = (hit.get("document"), hit.get("text_match")) {
                        let name = doc.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
                        let desc = doc.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string();
                        let filepath = doc.get("filepath").and_then(|v| v.as_str()).unwrap_or("").to_string();
                        let r#type = doc.get("type").and_then(|v| v.as_str()).unwrap_or("").to_string();
                        let score = match_score.as_i64().unwrap_or(0);

                        results.push(SearchResult {
                            name,
                            description: desc,
                            filepath,
                            r#type,
                            score,
                        });
                    }
                }
            }
            print_results(query, results);
        }
        _ => {
            let res = local_fallback_search(query, workspace_dir);
            print_results(query, res);
        }
    }
}

fn print_results(query: &str, results: Vec<SearchResult>) {
    println!("\n{} Kết quả tìm kiếm Typesense cho: '{}'", ICON_SEARCH, query);
    println!("{}", "=".repeat(70));
    
    if results.is_empty() {
        println!("{} Không tìm thấy tài liệu phù hợp.", ICON_ERROR);
        println!("{}", "=".repeat(70));
        return;
    }

    for (idx, r) in results.iter().enumerate() {
        println!("{}. [{}] {} (Khớp: {})", idx + 1, r.r#type.to_uppercase(), r.name, r.score);
        println!("   Đường dẫn: {}", r.filepath);
        println!("   Mô tả: {}", r.description);
        println!("{}", "-".repeat(70));
    }
}

fn setup_schema(client: &reqwest::blocking::Client) -> Result<(), Box<dyn std::error::Error>> {
    // Delete existing collection if any
    let delete_url = "http://localhost:8108/collections/agents_knowledge";
    let _ = client.delete(delete_url)
        .header("X-TYPESENSE-API-KEY", "antigravity_secret_key_123")
        .send();

    // Create new collection schema
    let create_url = "http://localhost:8108/collections";
    let schema = serde_json::json!({
        "name": "agents_knowledge",
        "fields": [
            {"name": "name", "type": "string"},
            {"name": "description", "type": "string"},
            {"name": "trigger", "type": "string", "optional": true},
            {"name": "content", "type": "string"},
            {"name": "filepath", "type": "string"},
            {"name": "type", "type": "string", "facet": true}
        ]
    });

    let res = client.post(create_url)
        .header("X-TYPESENSE-API-KEY", "antigravity_secret_key_123")
        .json(&schema)
        .send()?;

    if res.status().is_success() {
        println!("Collection 'agents_knowledge' created successfully.");
        Ok(())
    } else {
        let err_text = res.text().unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!("Failed to create collection: {}", err_text).into())
    }
}

pub fn run_indexer(workspace_dir: &str) -> bool {
    let client = match reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(5))
        .build() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{} Lỗi khởi tạo client: {}", ICON_ERROR, e);
                return false;
            }
        };

    println!("{} Đang thiết lập Schema cho Typesense...", ICON_PROGRESS);
    if let Err(e) = setup_schema(&client) {
        eprintln!("{} Lỗi thiết lập Typesense: {}", ICON_ERROR, e);
        eprintln!("Hãy chắc chắn rằng Typesense Server đang hoạt động tại http://localhost:8108");
        return false;
    }

    let agents_dir = Path::new(workspace_dir).join(".agents");
    if !agents_dir.exists() {
        eprintln!("{} Lỗi: Thư mục .agents không tồn tại tại {}", ICON_ERROR, workspace_dir);
        return false;
    }

    let categories = vec!["rules", "skills", "workflows", ".shared"];
    let mut indexed_count = 0;

    for cat in categories {
        let cat_path = agents_dir.join(cat);
        if !cat_path.exists() {
            continue;
        }

        let mut paths_to_visit = vec![cat_path];
        while let Some(dir_path) = paths_to_visit.pop() {
            if let Ok(entries) = fs::read_dir(dir_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        paths_to_visit.push(path);
                    } else if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                        if let Some((name, desc, trig, content)) = parse_markdown_file(&path) {
                            let rel_path = path
                                .strip_prefix(workspace_dir)
                                .unwrap_or(&path)
                                .to_string_lossy()
                                .replace('\\', "/");

                            let doc = serde_json::json!({
                                "name": name,
                                "description": desc,
                                "trigger": if trig.is_empty() { None } else { Some(trig) },
                                "content": content,
                                "filepath": rel_path,
                                "type": cat
                            });

                            let url = "http://localhost:8108/collections/agents_knowledge/documents";
                            let res = client.post(url)
                                .header("X-TYPESENSE-API-KEY", "antigravity_secret_key_123")
                                .json(&doc)
                                .send();

                            match res {
                                Ok(resp) if resp.status().is_success() => {
                                    indexed_count += 1;
                                    println!("Indexed [{}]: {} -> {}", cat.to_uppercase(), rel_path, name);
                                }
                                Ok(resp) => {
                                    let status = resp.status();
                                    let err_msg = resp.text().unwrap_or_else(|_| "Unknown error".to_string());
                                    eprintln!("{} Failed to index {}: Status {} - {}", ICON_WARNING, rel_path, status, err_msg);
                                }
                                Err(e) => {
                                    eprintln!("{} Failed to index {}: {}", ICON_WARNING, rel_path, e);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("\n{} Lập chỉ mục thành công! Tổng số tệp đã lập: {}", ICON_SUCCESS, indexed_count);
    true
}
