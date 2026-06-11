let ws = null;
let connectionStatus = 'disconnected';
let reconnectTimeout = null;
let idleTimer = null;
const IDLE_TIMEOUT_MS = 180000; // 3 phút tự động ngủ

// Lấy cài đặt ban đầu
chrome.storage.local.get(['connectionEnabled'], (result) => {
  if (result.connectionEnabled) {
    connectWebSocket();
  }
});

// Quản lý Idle Timer
function resetIdleTimer() {
  if (idleTimer) {
    clearTimeout(idleTimer);
  }
  idleTimer = setTimeout(() => {
    console.log('Connection idle. Disconnecting to save resources...');
    disconnectWebSocket();
  }, IDLE_TIMEOUT_MS);
}

function stopIdleTimer() {
  if (idleTimer) {
    clearTimeout(idleTimer);
    idleTimer = null;
  }
}

// Đánh thức kết nối khi có hoạt động
function wakeUpAndConnect() {
  chrome.storage.local.get(['connectionEnabled'], (result) => {
    if (result.connectionEnabled && connectionStatus === 'disconnected') {
      console.log('User activity detected. Waking up and connecting WebSocket...');
      connectWebSocket();
    } else if (result.connectionEnabled && connectionStatus === 'connected') {
      resetIdleTimer();
    }
  });
}

// Lắng nghe sự kiện tải lại trang để tự động thức dậy
chrome.tabs.onUpdated.addListener((tabId, changeInfo) => {
  if (changeInfo.status === 'complete' || changeInfo.url) {
    wakeUpAndConnect();
  }
});

// Lắng nghe tin nhắn từ popup.js và content.js
chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
  if (message.action === 'connect') {
    connectWebSocket();
    sendResponse({ status: connectionStatus });
  } else if (message.action === 'disconnect') {
    disconnectWebSocket();
    sendResponse({ status: connectionStatus });
  } else if (message.action === 'getStatus') {
    sendResponse({ status: connectionStatus });
  } else if (message.action === 'user_activity') {
    wakeUpAndConnect();
    sendResponse({ success: true });
    return true; // Phản hồi bất đồng bộ
  } else if (message.action === 'navigate_bg') {
    const tabId = sender.tab ? sender.tab.id : null;
    if (!tabId) {
       sendResponse({ success: false, error: "No tab context for navigation" });
       return true;
    }
    
    // Bắt đầu navigate
    chrome.tabs.update(tabId, { url: message.url }, (tab) => {
      // Đợi trạng thái complete
      const listener = (updatedTabId, info) => {
        if (updatedTabId === tabId && info.status === 'complete') {
          chrome.tabs.onUpdated.removeListener(listener);
          // Buffer thêm 500ms để React/Vue hydrate
          setTimeout(() => {
             sendResponse({ success: true, message: `Navigated to ${message.url} successfully` });
          }, 500);
        }
      };
      chrome.tabs.onUpdated.addListener(listener);
    });
    return true; // Asynchronous response
  }
  return true;
});

function connectWebSocket() {
  if (reconnectTimeout) {
    clearTimeout(reconnectTimeout);
    reconnectTimeout = null;
  }

  if (ws) {
    try {
      ws.onclose = null;
      ws.onerror = null;
      ws.close();
    } catch (e) {}
  }

  connectionStatus = 'connecting';
  chrome.storage.local.set({ connectedStatus: connectionStatus });

  ws = new WebSocket('ws://127.0.0.1:9090');

  ws.onopen = () => {
    connectionStatus = 'connected';
    chrome.storage.local.set({ connectedStatus: connectionStatus });
    console.log('Connected to Antigravity Bridge Server');
    resetIdleTimer();
  };

  ws.onmessage = async (event) => {
    resetIdleTimer();
    try {
      const request = JSON.parse(event.data);
      const action = request.action; // browser_observe, browser_act, browser_navigate, browser_extract
      const data = request.data || {};
      
      // 1. BROWSER_OBSERVE
      if (action === 'browser_observe') {
        const tabData = await getActiveTabDetails();
        ws.send(JSON.stringify({ id: request.id, status: 'success', data: tabData }));
      } 
      
      // 2. BROWSER_NAVIGATE
      else if (action === 'browser_navigate') {
        const actionType = data.action_type;
        
        if (actionType === 'new_tab') {
           chrome.tabs.create({ url: data.url || 'chrome://newtab' }, (tab) => {
             const listener = (updatedTabId, info) => {
               if (updatedTabId === tab.id && info.status === 'complete') {
                 chrome.tabs.onUpdated.removeListener(listener);
                 setTimeout(() => {
                   if (ws && ws.readyState === WebSocket.OPEN) {
                     ws.send(JSON.stringify({ id: request.id, status: 'success', data: { success: true, tabId: tab.id } }));
                   }
                 }, 500);
               }
             };
             chrome.tabs.onUpdated.addListener(listener);
             setTimeout(() => chrome.tabs.onUpdated.removeListener(listener), 15000);
           });
        } else if (actionType === 'navigate') {
           chrome.tabs.query({ active: true, currentWindow: true }, (tabs) => {
             if (!tabs.length) {
               ws.send(JSON.stringify({ id: request.id, status: 'error', message: 'No active tab' }));
               return;
             }
             chrome.tabs.update(tabs[0].id, { url: data.url }, (tab) => {
               const listener = (updatedTabId, info) => {
                 if (updatedTabId === tab.id && info.status === 'complete') {
                   chrome.tabs.onUpdated.removeListener(listener);
                   setTimeout(() => {
                     if (ws && ws.readyState === WebSocket.OPEN) {
                       ws.send(JSON.stringify({ id: request.id, status: 'success', data: { success: true } }));
                     }
                   }, 500);
                 }
               };
               chrome.tabs.onUpdated.addListener(listener);
               setTimeout(() => chrome.tabs.onUpdated.removeListener(listener), 15000);
             });
           });
        } else if (actionType === 'list_tabs') {
           chrome.tabs.query({}, (tabs) => {
             const tabList = tabs.map(t => ({ id: t.id, url: t.url, title: t.title, active: t.active }));
             ws.send(JSON.stringify({ id: request.id, status: 'success', data: { tabs: tabList } }));
           });
        } else if (actionType === 'close_tab') {
           chrome.tabs.remove(data.tab_id || -1, () => {
             ws.send(JSON.stringify({ id: request.id, status: 'success', data: { success: true } }));
           });
        }
      }
      
      // 3. BROWSER_ACT & BROWSER_EXTRACT (Giao cho content.js)
      else if (action === 'browser_act' || action === 'browser_extract') {
        chrome.tabs.query({ active: true, currentWindow: true }, (tabs) => {
           if (!tabs.length) {
             ws.send(JSON.stringify({ id: request.id, status: 'error', message: 'No active tab' }));
             return;
           }
           chrome.tabs.sendMessage(tabs[0].id, { action: action, details: data }, (response) => {
             if (chrome.runtime.lastError) {
               ws.send(JSON.stringify({ id: request.id, status: 'error', message: chrome.runtime.lastError.message }));
             } else {
               ws.send(JSON.stringify({ id: request.id, status: 'success', data: response }));
             }
           });
        });
      }
      
    } catch (err) {
      console.error('Error processing message from server:', err);
    }
  };

  ws.onclose = () => {
    connectionStatus = 'disconnected';
    chrome.storage.local.set({ connectedStatus: connectionStatus });
    console.log('Disconnected from Antigravity Bridge Server');
    stopIdleTimer();
    
    // Thử kết nối lại sau 5 giây nếu Switch vẫn bật
    chrome.storage.local.get(['connectionEnabled'], (result) => {
      if (result.connectionEnabled) {
        if (reconnectTimeout) clearTimeout(reconnectTimeout);
        reconnectTimeout = setTimeout(connectWebSocket, 5000);
      }
    });
  };

  ws.onerror = (error) => {
    console.error('WebSocket Error:', error);
    connectionStatus = 'disconnected';
    chrome.storage.local.set({ connectedStatus: connectionStatus });
    stopIdleTimer();
  };
}

function disconnectWebSocket() {
  if (reconnectTimeout) {
    clearTimeout(reconnectTimeout);
    reconnectTimeout = null;
  }
  stopIdleTimer();
  if (ws) {
    try {
      ws.onclose = null;
      ws.onerror = null;
      ws.close();
    } catch (e) {}
    ws = null;
  }
  connectionStatus = 'disconnected';
  chrome.storage.local.set({ connectedStatus: connectionStatus });
}

// Hàm lấy thông tin chi tiết của Tab đang hiển thị
async function getActiveTabDetails() {
  return new Promise((resolve, reject) => {
    chrome.tabs.query({ active: true, currentWindow: true }, async (tabs) => {
      if (!tabs || tabs.length === 0) {
        resolve({ error: 'No active tab found' });
        return;
      }

      const activeTab = tabs[0];
      
      try {
        // 1. Chụp ảnh màn hình (screenshot) dạng base64
        const screenshotDataUrl = await captureTabScreenshot(activeTab.windowId);
        
        // 2. Chạy content script để lấy DOM đã lọc
        const domResult = await getTabDOM(activeTab.id);

        resolve({
          tabId: activeTab.id,
          title: activeTab.title,
          url: activeTab.url,
          dom: domResult,
          screenshot: screenshotDataUrl
        });
      } catch (err) {
        resolve({
          tabId: activeTab.id,
          title: activeTab.title,
          url: activeTab.url,
          dom: '',
          screenshot: '',
          error: err.message
        });
      }
    });
  });
}

function captureTabScreenshot(windowId) {
  return new Promise((resolve) => {
    chrome.tabs.captureVisibleTab(windowId, { format: 'png' }, (dataUrl) => {
      resolve(dataUrl || '');
    });
  });
}

function getTabDOM(tabId) {
  return new Promise((resolve) => {
    chrome.scripting.executeScript({
      target: { tabId: tabId },
      func: () => {
        // Ưu tiên gọi hàm Accessibility Tree mới
        if (typeof window.getAccessibilitySnapshot === 'function') {
          return window.getAccessibilitySnapshot();
        } else if (typeof window.getInteractiveDOM === 'function') {
          return window.getInteractiveDOM();
        }
        
        // Fallback đơn giản nếu content.js chưa load kịp
        return document.body ? document.body.innerText.substring(0, 5000) : 'No content';
      }
    }, (results) => {
      if (results && results[0]) {
        resolve(results[0].result);
      } else {
        resolve('Failed to execute DOM script');
      }
    });
  });
}
