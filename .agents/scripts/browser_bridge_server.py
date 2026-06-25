import asyncio
import json
import uuid
import sys
from websockets.server import serve

# State lưu trữ kết nối
extension_ws = None
pending_requests = {}

async def process_http_request(path, headers):
    """
    Xử lý các HTTP GET request gửi tới /tab-info trên cùng một cổng 9090
    """
    global extension_ws, pending_requests

    if path == "/tab-info":
        # 1. Kiểm tra kết nối từ Browser Extension
        if extension_ws is None:
            response_body = json.dumps({"error": "Browser Extension chưa kết nối hoặc đang ở trạng thái OFF."})
            return (
                503,
                [
                    ("Content-Type", "application/json"),
                    ("Content-Length", str(len(response_body))),
                    ("Access-Control-Allow-Origin", "*"), # Cho phép CORS
                ],
                response_body.encode("utf-8"),
            )

        # 2. Tạo Request ID ngẫu nhiên
        request_id = str(uuid.uuid4())
        loop = asyncio.get_running_loop()
        future = loop.create_future()
        pending_requests[request_id] = future

        # 3. Gửi yêu cầu qua WebSocket tới Extension
        ws_message = json.dumps({
            "id": request_id,
            "action": "get_tab_info"
        })
        
        try:
            await extension_ws.send(ws_message)
        except Exception as e:
            pending_requests.pop(request_id, None)
            response_body = json.dumps({"error": f"Không thể gửi lệnh tới Extension: {str(e)}"})
            return (
                500,
                [
                    ("Content-Type", "application/json"),
                    ("Content-Length", str(len(response_body))),
                    ("Access-Control-Allow-Origin", "*"),
                ],
                response_body.encode("utf-8"),
            )

        # 4. Đợi phản hồi từ Extension (Timeout 10 giây)
        try:
            data = await asyncio.wait_for(future, timeout=10.0)
            response_body = json.dumps(data)
            return (
                200,
                [
                    ("Content-Type", "application/json"),
                    ("Content-Length", str(len(response_body))),
                    ("Access-Control-Allow-Origin", "*"),
                ],
                response_body.encode("utf-8"),
            )
        except asyncio.TimeoutError:
            pending_requests.pop(request_id, None)
            response_body = json.dumps({"error": "Hết thời gian chờ phản hồi từ trình duyệt."})
            return (
                504,
                [
                    ("Content-Type", "application/json"),
                    ("Content-Length", str(len(response_body))),
                    ("Access-Control-Allow-Origin", "*"),
                ],
                response_body.encode("utf-8"),
            )

    # Các router khác trả về 404
    response_body = "Not Found"
    return (
        404,
        [
            ("Content-Type", "text/plain"),
            ("Content-Length", str(len(response_body))),
        ],
        response_body.encode("utf-8"),
    )

async def handle_websocket(websocket):
    """
    Xử lý kết nối WebSocket từ Browser Extension
    """
    global extension_ws, pending_requests
    print("Browser Extension đã kết nối WebSocket thành công.")
    extension_ws = websocket

    try:
        async for message in websocket:
            try:
                response = json.loads(message)
                req_id = response.get("id")
                data = response.get("data")
                
                # Định tuyến phản hồi về HTTP request đang đợi qua ID
                if req_id in pending_requests:
                    future = pending_requests.pop(req_id)
                    if not future.cancelled():
                        future.set_result(data)
            except Exception as e:
                print(f"Lỗi xử lý tin nhắn từ Extension: {e}", file=sys.stderr)
    except Exception as e:
        print(f"Lỗi kết nối WebSocket: {e}", file=sys.stderr)
    finally:
        print("Browser Extension đã ngắt kết nối WebSocket.")
        extension_ws = None

async def main():
    # Khởi chạy server trên cổng 9090
    print("Antigravity Bridge Server (Python Backup) đang khởi chạy tại http://127.0.0.1:9090")
    
    # serve của thư viện websockets hỗ trợ cả WebSocket và xử lý HTTP fallback qua process_request
    async with serve(
        handle_websocket, 
        "127.0.0.1", 
        9090,
        process_request=process_http_request
    ):
        await asyncio.Future() # Chạy vĩnh viễn

if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        print("\nĐã dừng Antigravity Bridge Server.")
