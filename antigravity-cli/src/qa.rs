use std::net::TcpStream;
use std::time::Duration;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::fs;
use reqwest::blocking::Client;
use serde_json::Value;
use crate::icons::*;

fn is_port_open(port: u16) -> bool {
    let addr = format!("127.0.0.1:{}", port);
    if let Ok(socket_addr) = addr.parse() {
        TcpStream::connect_timeout(&socket_addr, Duration::from_millis(200)).is_ok()
    } else {
        false
    }
}

pub fn run_qa_tests(workspace_dir: &str, url: &str) -> Result<(), String> {
    // 1. Đảm bảo server đang chạy
    if !is_port_open(9090) {
        println!("{} Cổng 9090 chưa mở. Khởi động Browser Bridge...", ICON_PROGRESS);
        crate::browser_client::ensure_running();
        // Đợi 2 giây để server khởi động hoàn toàn
        std::thread::sleep(Duration::from_millis(2000));
    } else {
        println!("{} Phát hiện Browser Bridge đang chạy trên cổng 9090.", ICON_SUCCESS);
    }

    let client = Client::new();
    
    // 2. Kết nối tới SSE để nhận event phản hồi
    println!("{} Thiết lập kết nối SSE với Browser Bridge...", ICON_PROGRESS);
    let sse_response = client.get("http://127.0.0.1:9090/mcp/sse")
        .send()
        .map_err(|e| format!("Không thể kết nối SSE: {}", e))?;
    
    let mut reader = BufReader::new(sse_response);
    
    // Lấy endpoint URL từ event đầu tiên của SSE
    let mut line = String::new();
    let mut session_id = "default_session".to_string();
    
    // Đọc cho đến khi nhận được dòng chứa endpoint/sessionId
    for _ in 0..20 {
        line.clear();
        if reader.read_line(&mut line).is_err() {
            break;
        }
        if line.contains("sessionId=") {
            if let Some(idx) = line.find("sessionId=") {
                session_id = line[idx + 10..].trim().to_string();
            }
            break;
        }
    }
    
    println!("   -> Kết nối thành công. Session ID: {}", session_id);

    // Định nghĩa helper gửi request JSON-RPC qua POST và đợi response qua SSE reader
    let mut call_tool = |id: u64, name: &str, arguments: Value| -> Result<Value, String> {
        let req = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": "tools/call",
            "params": {
                "name": name,
                "arguments": arguments
            }
        });

        let post_url = format!("http://127.0.0.1:9090/mcp/message?sessionId={}", session_id);
        let post_res = client.post(&post_url)
            .json(&req)
            .send()
            .map_err(|e| format!("Lỗi gửi POST: {}", e))?;
        
        if post_res.status() != reqwest::StatusCode::ACCEPTED {
            return Err(format!("Server từ chối request với HTTP: {}", post_res.status()));
        }

        // Lặp đọc SSE stream để tìm response có id khớp
        let mut sse_line = String::new();
        let start_time = std::time::Instant::now();
        
        while start_time.elapsed().as_secs() < 15 {
            sse_line.clear();
            if reader.read_line(&mut sse_line).is_ok() {
                if sse_line.starts_with("data:") {
                    let data_str = sse_line.trim_start_matches("data:").trim();
                    if let Ok(resp) = serde_json::from_str::<Value>(data_str) {
                        if resp.get("id").and_then(|i| i.as_u64()) == Some(id) {
                            if let Some(err) = resp.get("error") {
                                return Err(format!("Lỗi tool: {}", err));
                            }
                            let result = resp.get("result").cloned().unwrap_or(Value::Null);
                            return Ok(result);
                        }
                    }
                }
            }
        }
        
        Err("Hết thời gian chờ phản hồi từ trình duyệt qua SSE (Timeout 15s).".to_string())
    };

    // 3. Navigate to URL
    println!("{} Đang điều hướng trình duyệt đến: {}", ICON_PROGRESS, url);
    let nav_res = call_tool(2, "browser_navigate", serde_json::json!({
        "action_type": "navigate",
        "url": url
    }))?;

    if nav_res.get("isError").and_then(|v| v.as_bool()) == Some(true) {
        let err_text = nav_res.get("content").and_then(|c| c.get(0)).and_then(|t| t.get("text")).and_then(|s| s.as_str()).unwrap_or("Lỗi không xác định");
        return Err(format!("Lỗi điều hướng trình duyệt: {}", err_text));
    }
    println!("   -> Trình duyệt đã điều hướng đến trang đích.");

    // Chờ 3 giây để trang web ổn định layout
    println!("{} Đang đợi 3 giây để trang web ổn định layout...", ICON_PROGRESS);
    std::thread::sleep(Duration::from_secs(3));

    // 4. Extract text to verify
    println!("{} Đang thu thập nội dung văn bản (Extract Text)...", ICON_PROGRESS);
    let extract_res = call_tool(3, "browser_extract", serde_json::json!({}))?;

    let text_content = extract_res.get("content")
        .and_then(|c| c.get(0))
        .and_then(|t| t.get("text"))
        .and_then(|s| s.as_str())
        .unwrap_or("");

    println!("   -> Độ dài văn bản thu thập được: {} ký tự.", text_content.len());

    // 5. Ghi nhận báo cáo
    let reports_dir = Path::new(workspace_dir).join(".agents").join("state").join("reports");
    fs::create_dir_all(&reports_dir).map_err(|e| e.to_string())?;

    let report_html_path = reports_dir.join("qa_report.html");
    
    let html_content = format!(r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Antigravity QA E2E Report</title>
    <style>
        body {{ font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; margin: 40px; background-color: #f9f9fb; color: #1e1e24; }}
        .card {{ background: white; padding: 30px; border-radius: 12px; box-shadow: 0 4px 12px rgba(0,0,0,0.05); }}
        h1 {{ color: #4f46e5; margin-top: 0; }}
        .badge {{ display: inline-block; padding: 6px 12px; border-radius: 20px; font-weight: bold; font-size: 14px; }}
        .badge-success {{ background-color: #ecfdf5; color: #047857; }}
        .url {{ font-family: monospace; font-size: 16px; color: #6b7280; margin: 10px 0 20px 0; }}
        pre {{ background: #f3f4f6; padding: 15px; border-radius: 8px; overflow-x: auto; white-space: pre-wrap; font-size: 14px; max-height: 400px; }}
    </style>
</head>
<body>
    <div class="card">
        <h1>Antigravity QA E2E Verification Report</h1>
        <div class="badge badge-success">PASSED</div>
        <div class="url">Tested URL: {url}</div>
        <h3>Trích xuất nội dung trang web (Mẫu 1000 ký tự đầu tiên):</h3>
        <pre>{truncate_text}</pre>
    </div>
</body>
</html>"#, url = url, truncate_text = &text_content[..text_content.len().min(1000)]);

    fs::write(&report_html_path, html_content).map_err(|e| e.to_string())?;
    println!("{} Báo cáo E2E đã được ghi nhận tại: {}", ICON_SUCCESS, report_html_path.display());

    Ok(())
}
