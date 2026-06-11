// Hàm chờ element sẵn sàng (Actionability Checks) - Học từ Playwright
async function waitForActionability(el, timeoutMs = 5000) {
  const start = Date.now();
  let lastRect = null;
  let lastReason = "Unknown reason";
  
  return new Promise((resolve, reject) => {
    const check = () => {
      if (Date.now() - start > timeoutMs) {
        return reject(new Error(`Timeout: Element is not actionable after 5s. Last reason: ${lastReason}`));
      }

      // 1. Attached
      if (!el.isConnected) return retry("Element is detached from DOM");

      // 2. Visible
      const style = window.getComputedStyle(el);
      if (style.display === 'none' || style.visibility === 'hidden') return retry("Element is hidden by style");
      
      const rect = el.getBoundingClientRect();
      if (rect.width === 0 || rect.height === 0) return retry("Element has 0 size");

      // 3. Stable (not animating)
      if (lastRect) {
        if (Math.abs(rect.x - lastRect.x) > 0.5 || Math.abs(rect.y - lastRect.y) > 0.5 || 
            Math.abs(rect.width - lastRect.width) > 0.5 || Math.abs(rect.height - lastRect.height) > 0.5) {
          lastRect = rect;
          return retry("Element is animating/moving");
        }
      } else {
        lastRect = rect;
        return retry("Waiting for stability (need 2 frames)");
      }

      // 4. Enabled
      if (el.hasAttribute('disabled') || el.disabled) return retry("Element is disabled");

      // 5. Unblocked (receives events)
      const x = rect.left + rect.width / 2;
      const y = rect.top + rect.height / 2;
      
      // Check if inside viewport, scroll if needed
      if (x < 0 || y < 0 || x > window.innerWidth || y > window.innerHeight) {
          el.scrollIntoView({ behavior: 'instant', block: 'center' });
          lastRect = null; // Reset stability check after scroll
          return retry("Element scrolled into view, waiting for next frame");
      }

      const topEl = document.elementFromPoint(x, y);
      if (topEl !== el && !el.contains(topEl)) {
         return retry(`Element is blocked by overlay: <${topEl?.tagName.toLowerCase()} class="${topEl?.className}">`);
      }

      resolve(); // All checks passed!
    };

    const retry = (reason) => {
      lastReason = reason;
      // console.log(`[Antigravity Auto-wait] Retry: ${reason}`);
      setTimeout(check, 100);
    };

    check();
  });
}

// Hàm xây dựng Accessibility Tree - Trả về semantic state gọn gàng
function buildAccessibilityTree() {
  let elementId = 1;
  let resultText = [];

  function cleanText(text) {
    return text ? text.replace(/\s+/g, ' ').trim().substring(0, 100) : '';
  }

  const interactiveTags = ['a', 'button', 'input', 'select', 'textarea'];
  
  // Xóa các ID cũ để tránh trùng lặp khi DOM thay đổi
  document.querySelectorAll('[data-antigravity-id]').forEach(e => e.removeAttribute('data-antigravity-id'));

  const walker = document.createTreeWalker(document.body, NodeFilter.SHOW_ELEMENT, {
    acceptNode: function(node) {
      const style = window.getComputedStyle(node);
      if (style.display === 'none' || style.visibility === 'hidden' || node.offsetWidth === 0 || node.offsetHeight === 0) {
        return NodeFilter.FILTER_REJECT;
      }
      return NodeFilter.FILTER_ACCEPT;
    }
  });

  let currentNode = walker.currentNode;
  while(currentNode) {
    const el = currentNode;
    const tagName = el.tagName.toLowerCase();
    const explicitRole = el.getAttribute('role');
    
    // Tự động map Implicit Role
    let role = explicitRole;
    if (!role) {
      if (tagName === 'a' && el.hasAttribute('href')) role = 'link';
      else if (tagName === 'button' || (tagName === 'input' && ['button','submit','reset'].includes(el.type))) role = 'button';
      else if (tagName === 'input' && el.type === 'checkbox') role = 'checkbox';
      else if (tagName === 'input' && el.type === 'radio') role = 'radio';
      else if (tagName === 'input' || tagName === 'textarea') role = 'textbox';
      else if (tagName === 'select') role = 'combobox';
    }

    const isInteractive = role || interactiveTags.includes(tagName) || el.hasAttribute('onclick') || el.tabIndex >= 0;

    if (isInteractive) {
      // Lấy tên đại diện (Accessible Name)
      let name = el.getAttribute('aria-label') || el.getAttribute('title') || el.getAttribute('placeholder') || cleanText(el.innerText);
      
      // Lấy trạng thái (State)
      let state = [];
      if (el.disabled || el.getAttribute('aria-disabled') === 'true') state.push('disabled');
      if (el.checked || el.getAttribute('aria-checked') === 'true') state.push('checked');
      if (el.getAttribute('aria-expanded')) state.push(`expanded:${el.getAttribute('aria-expanded')}`);
      if (el.value && ['textbox', 'combobox'].includes(role)) state.push(`value:"${cleanText(el.value)}"`);
      if (tagName === 'a' && el.hasAttribute('href')) state.push(`href:"${el.getAttribute('href')}"`);

      const displayRole = role || tagName;
      
      let description = `[${elementId}] ${displayRole}`;
      if (name) description += ` "${name}"`;
      if (state.length > 0) description += ` (${state.join(', ')})`;
      
      resultText.push(description);
      el.setAttribute('data-antigravity-id', elementId);
      elementId++;
    }

    currentNode = walker.nextNode();
  }
  
  return resultText.join('\n');
}

// Fallback tương thích ngược
window.getInteractiveDOM = buildAccessibilityTree;
window.getAccessibilitySnapshot = buildAccessibilityTree;

console.log('Antigravity Content Script Loaded (v5 - Auto-waiting enabled)');

chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
  const action = request.action;
  const details = request.details || {};

  (async () => {
    try {
      if (action === 'extract_dom') {
        const domText = buildAccessibilityTree();
        sendResponse({ success: true, domText: domText });
      } else if (action === 'browser_act') {
        const actionType = details.action_type;
        
        if (actionType === 'click_at') {
           const coordType = details.coordinate_type || 'css';
           let clientX, clientY;
           if (coordType === 'physical') {
              clientX = (details.x / window.devicePixelRatio) - window.scrollX;
              clientY = (details.y / window.devicePixelRatio) - window.scrollY;
           } else {
              clientX = details.x - window.scrollX;
              clientY = details.y - window.scrollY;
           }
           
           const el = document.elementFromPoint(clientX, clientY);

           // Bắt ngay case Iframe và gợi ý Agent tự navigate
           if (el && el.tagName === 'IFRAME') {
               const iframeSrc = el.src;
               return sendResponse({ 
                   success: false, 
                   error: `Targeted coordinates fall inside a cross-origin Iframe.`,
                   suggestion: `Please call 'browser_navigate' to URL: "${iframeSrc}" to interact with the content inside this frame.`
               });
           }

           if (!el) {
              return sendResponse({ success: false, error: `No element found at coordinates (${details.x}, ${details.y})` });
           }
           el.focus();
           el.click();
           sendResponse({ success: true, message: `Clicked element at (${details.x}, ${details.y})` });
           
        } else {
           // Dựa trên target_id
           let el = document.querySelector(`[data-antigravity-id="${details.target_id}"]`);
           let driftWarning = null;

           if (!el && (details.fallback_text || details.fallback_role)) {
               // Self-Healing
               const walker = document.createTreeWalker(document.body, NodeFilter.SHOW_ELEMENT, {
                   acceptNode: function(node) {
                       const style = window.getComputedStyle(node);
                       if (style.display === 'none' || style.visibility === 'hidden' || node.offsetWidth === 0 || node.offsetHeight === 0) {
                           return NodeFilter.FILTER_REJECT;
                       }
                       
                       // Check role
                       let role = node.getAttribute('role');
                       if (!role) {
                           const tagName = node.tagName.toLowerCase();
                           if (tagName === 'a' && node.hasAttribute('href')) role = 'link';
                           else if (tagName === 'button' || (tagName === 'input' && ['button','submit','reset'].includes(node.type))) role = 'button';
                           else if (tagName === 'input' && node.type === 'checkbox') role = 'checkbox';
                           else if (tagName === 'input' && node.type === 'radio') role = 'radio';
                           else if (tagName === 'input' || tagName === 'textarea') role = 'textbox';
                           else if (tagName === 'select') role = 'combobox';
                       }
                       
                       const roleMatch = !details.fallback_role || role === details.fallback_role;
                       
                       // Check text
                       let textMatch = true;
                       if (details.fallback_text) {
                           const fallback = details.fallback_text.toLowerCase();
                           const innerText = (node.innerText || "").toLowerCase();
                           const ariaLabel = (node.getAttribute('aria-label') || "").toLowerCase();
                           const placeholder = (node.getAttribute('placeholder') || "").toLowerCase();
                           const title = (node.getAttribute('title') || "").toLowerCase();
                           
                           textMatch = innerText.includes(fallback) || ariaLabel.includes(fallback) || placeholder.includes(fallback) || title.includes(fallback);
                       }
                       
                       if (roleMatch && textMatch) return NodeFilter.FILTER_ACCEPT;
                       return NodeFilter.FILTER_SKIP;
                   }
               });
               
               let matchedElements = [];
               let currentNode = walker.nextNode();
               while (currentNode) {
                   matchedElements.push(currentNode);
                   currentNode = walker.nextNode();
               }
               
               if (matchedElements.length > 0) {
                   // Ambiguity Resolution: Ưu tiên phần tử nằm trong Viewport
                   const visibleElements = matchedElements.filter(node => {
                       const rect = node.getBoundingClientRect();
                       return rect.top < window.innerHeight && rect.bottom > 0 && rect.left < window.innerWidth && rect.right > 0;
                   });
                   
                   const targetSet = visibleElements.length > 0 ? visibleElements : matchedElements;
                   
                   if (targetSet.length >= 1) {
                       // Tìm thẻ interactive tag (a, button) là chính nó, con nó, hoặc cha nó
                       let interactiveMatches = [];
                       for (const node of targetSet) {
                           const tag = node.tagName.toLowerCase();
                           if (['a', 'button'].includes(tag) || node.getAttribute('role') === 'link' || node.getAttribute('role') === 'button') {
                               interactiveMatches.push(node);
                           } else {
                               const childLink = node.querySelector('a, button, [role="link"], [role="button"]');
                               if (childLink) {
                                   interactiveMatches.push(childLink);
                               } else {
                                   const parentLink = node.closest('a, button, [role="link"], [role="button"]');
                                   if (parentLink) {
                                       interactiveMatches.push(parentLink);
                                   }
                               }
                           }
                       }
                       el = interactiveMatches.length > 0 ? interactiveMatches[0] : targetSet[0];
                       driftWarning = `DOM Drift Detected! ID [${details.target_id}] was lost, but Self-Healing recovered the element using fallback (picked interactive or first of ${targetSet.length} matches).`;
                   } else {
                       return sendResponse({
                           success: false,
                           error: `AmbiguousElementError: Self-Healing found 0 matching elements in target set.`,
                           suggestion: "Call browser_observe to get fresh IDs."
                       });
                   }
               }
           }

           if (!el) {
             return sendResponse({ 
               success: false, 
               error: `Element [${details.target_id}] not found, and Self-Healing failed or was not provided.`,
               suggestion: "Call browser_observe to refresh element IDs."
             });
           }
           
           await waitForActionability(el, 5000);
           
           let successMsg = "";
           if (actionType === 'click') {
             el.focus();
             el.click();
             let hrefInfo = "";
             if (el.hasAttribute('href')) {
               hrefInfo = ` (href: "${el.getAttribute('href')}")`;
             } else {
               const parentLink = el.closest('a');
               if (parentLink && parentLink.hasAttribute('href')) {
                 hrefInfo = ` (href: "${parentLink.getAttribute('href')}")`;
               }
             }
             successMsg = `Clicked element [${details.target_id}]${hrefInfo}`;
           } else if (actionType === 'fill') {
             el.focus();
             const nativeInputValueSetter = Object.getOwnPropertyDescriptor(window.HTMLInputElement.prototype, 'value')?.set;
             const nativeTextAreaValueSetter = Object.getOwnPropertyDescriptor(window.HTMLTextAreaElement.prototype, 'value')?.set;
             
             if (el.tagName === 'TEXTAREA' && nativeTextAreaValueSetter) {
               nativeTextAreaValueSetter.call(el, details.text);
             } else if (nativeInputValueSetter) {
               nativeInputValueSetter.call(el, details.text);
             } else {
               el.value = details.text;
             }
             
             el.dispatchEvent(new Event('input', { bubbles: true }));
             el.dispatchEvent(new Event('change', { bubbles: true }));
             el.blur();
             successMsg = `Filled element [${details.target_id}] with text`;
           } else if (actionType === 'hover') {
             const rect = el.getBoundingClientRect();
             const x = rect.left + rect.width / 2;
             const y = rect.top + rect.height / 2;

             // Giả lập chính xác hành vi con trỏ chuột của người dùng
             ['mouseenter', 'mouseover', 'mousemove'].forEach(eventType => {
                 const mouseEvent = new MouseEvent(eventType, { 
                     view: window, 
                     bubbles: true, 
                     cancelable: true,
                     clientX: x,
                     clientY: y
                 });
                 el.dispatchEvent(mouseEvent);
             });

             successMsg = `Hovered element [${details.target_id}] with simulated cursor`;
           } else if (actionType === 'press_key') {
             el.focus();
             const keyEvent = new KeyboardEvent('keydown', { key: details.text, bubbles: true, cancelable: true });
             el.dispatchEvent(keyEvent);
             successMsg = `Pressed key ${details.text} on element [${details.target_id}]`;
           } else {
             return sendResponse({ success: false, error: `Unknown action_type: ${actionType}` });
           }

           if (driftWarning) successMsg += `\nWarning: ${driftWarning}`;
           sendResponse({ success: true, message: successMsg });
        }
      } else if (action === 'browser_extract') {
          let extractedText = [];
          
          // Quét trực tiếp trên Live DOM, siêu nhanh
          const walker = document.createTreeWalker(document.body, NodeFilter.SHOW_ELEMENT | NodeFilter.SHOW_TEXT, {
              acceptNode: function(node) {
                  if (node.nodeType === Node.ELEMENT_NODE) {
                      const tag = node.tagName.toLowerCase();
                      // Bỏ qua thẻ script, style, ẩn
                      if (['script', 'style', 'noscript'].includes(tag)) return NodeFilter.FILTER_REJECT;
                      // Tuyệt chiêu: Đọc offsetWidth siêu tốc thay vì getComputedStyle
                      if (node.offsetWidth === 0 && node.offsetHeight === 0) return NodeFilter.FILTER_REJECT;
                      return NodeFilter.FILTER_SKIP; // Skip thẻ, đi sâu vào lấy Text
                  }
                  if (node.nodeType === Node.TEXT_NODE) {
                      return node.nodeValue.trim() ? NodeFilter.FILTER_ACCEPT : NodeFilter.FILTER_SKIP;
                  }
                  return NodeFilter.FILTER_SKIP;
              }
          });

          let currentNode = walker.nextNode();
          while (currentNode) {
              extractedText.push(currentNode.nodeValue.trim());
              currentNode = walker.nextNode();
          }

          sendResponse({ success: true, data: { text: extractedText.join(' ').replace(/\s+/g, ' ') } });
      } else {
         sendResponse({ success: false, error: `Unknown action: ${action}` });
      }
    } catch (e) {
      sendResponse({ 
        success: false, 
        error: e.message
      });
    }
  })();
  return true; 
});

// Gửi tin nhắn báo hiệu hoạt động của người dùng (với throttle tránh spam)
let lastActivityTime = 0;
const ACTIVITY_THROTTLE_MS = 10000; // 10 giây tối đa 1 lần gửi

function notifyUserActivity() {
  const now = Date.now();
  if (now - lastActivityTime > ACTIVITY_THROTTLE_MS) {
    lastActivityTime = now;
    chrome.runtime.sendMessage({ action: 'user_activity' }, () => {
      // Bỏ qua lỗi runtime nếu Service Worker chưa hoạt động
      if (chrome.runtime.lastError) {}
    });
  }
}

// Lắng nghe các sự kiện tương tác của người dùng trên trang web
document.addEventListener('click', notifyUserActivity, { passive: true });
document.addEventListener('keydown', notifyUserActivity, { passive: true });
document.addEventListener('scroll', notifyUserActivity, { passive: true });
