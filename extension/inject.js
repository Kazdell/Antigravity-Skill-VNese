// inject.js
// Chạy trong MAIN world tại thời điểm document_start
// Mục tiêu: Vô hiệu hóa các dialog đồng bộ (blocking) để bảo vệ luồng tự động hóa của Agent.

(function() {
    console.log('[Antigravity] Intercepting synchronous dialogs...');
    
    window.alert = (msg) => {
        console.warn(`[Antigravity Alert Intercepted]: ${msg}`);
    };
    
    window.confirm = (msg) => {
        console.warn(`[Antigravity Confirm Intercepted]: ${msg}`);
        return true; // Auto-accept
    };
    
    window.prompt = (msg, defaultText) => {
        console.warn(`[Antigravity Prompt Intercepted]: ${msg}`);
        return defaultText || ""; // Trả về chuỗi mặc định hoặc rỗng
    };
})();
