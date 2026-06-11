document.addEventListener('DOMContentLoaded', () => {
  const toggleInput = document.getElementById('toggle-connection');
  const statusText = document.getElementById('status');
  const indicator = document.getElementById('indicator');

  // Lấy trạng thái hiện tại từ storage
  chrome.storage.local.get(['connectionEnabled', 'connectedStatus'], (result) => {
    toggleInput.checked = result.connectionEnabled || false;
    updateUI(result.connectedStatus || 'disconnected');
  });

  // Lắng nghe sự kiện bật/tắt switch
  toggleInput.addEventListener('change', () => {
    const isEnabled = toggleInput.checked;
    chrome.storage.local.set({ connectionEnabled: isEnabled });

    // Gửi tin nhắn cho background.js để bật/tắt kết nối
    chrome.runtime.sendMessage({ 
      action: isEnabled ? 'connect' : 'disconnect' 
    });

    if (!isEnabled) {
      updateUI('disconnected');
    } else {
      updateUI('connecting');
    }
  });

  // Định kỳ cập nhật trạng thái kết nối từ background.js
  function checkStatus() {
    chrome.runtime.sendMessage({ action: 'getStatus' }, (response) => {
      if (response) {
        updateUI(response.status);
      }
    });
  }

  setInterval(checkStatus, 1000);
  checkStatus();

  function updateUI(status) {
    if (status === 'connected') {
      statusText.textContent = 'ON';
      statusText.style.color = '#4CAF50';
      indicator.className = 'status-indicator connected';
    } else if (status === 'connecting') {
      statusText.textContent = 'CONNECTING...';
      statusText.style.color = '#FF9800';
      indicator.className = 'status-indicator';
    } else {
      statusText.textContent = 'OFF';
      statusText.style.color = '#6C7278';
      indicator.className = 'status-indicator';
    }
  }
});
