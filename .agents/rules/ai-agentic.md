---
trigger: model_decision
description: "When the user asks to implement AI features, write LLM prompts, setup agents, RAG, tool integrations, or configure MCPs."
---

# AI-AGENTIC.MD - Intelligence & Agents

> **Mục tiêu**: Chuẩn hóa quá trình tích hợp AI, tối ưu Prompt, và điều phối hệ thống Đa Tác Nhân (Multi-Agent).

---

##  1. LLM & PROMPT ENGINEERING

1. **System Instructions**: Rõ ràng, định nghĩa nhân dạng (Role) và giới hạn (Guardrails).
2. **Context Optimization**: Chỉ nhồi những context thực sự cần thiết để tiết kiệm Token và tránh Hallucination.
3. **JSON Output**: Ép buộc LLM trả về JSON structure schema thay vì text tự do nếu dùng để parsing.

---

##  2. AGENT & MCP ARCHITECTURE

1. **Tool Access**: Cấp quyền tối thiểu cho các công cụ (Tools) của Agent.
2. **Stateless Workflows**: Agent không lưu trữ state vĩnh viễn trong memory trừ khi có vector DB.
3. **MCP Standards**: Tuân thủ giao thức Model Context Protocol khi viết server mới.

---


