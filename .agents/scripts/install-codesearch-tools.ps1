<#
.SYNOPSIS
    Cài đặt CodeGraph và Semble - Bộ công cụ tìm kiếm code thông minh cho AI Agent
    
.DESCRIPTION
    Script này cài đặt:
    - @colbymchenry/codegraph: Xây dựng đồ thị quan hệ code (call graph, dependency)
    - semble: Tìm kiếm code theo ngữ nghĩa, tiết kiệm ~98% token so với grep thô
    
.PARAMETER SkipNodeCheck
    Bỏ qua kiểm tra Node.js (dùng nếu đã chắc chắn Node đã cài)
    
.PARAMETER SkipPythonCheck
    Bỏ qua kiểm tra Python (dùng nếu đã chắc chắn Python đã cài)
    
.EXAMPLE
    .\install-codesearch-tools.ps1
    .\install-codesearch-tools.ps1 -SkipNodeCheck -SkipPythonCheck

.NOTES
    Antigravity-Skill-VNese — MK11 Code Search Tools
    GitHub: https://github.com/colbymchenry/codegraph | https://github.com/MinishLab/semble
#>

param(
    [switch]$SkipNodeCheck,
    [switch]$SkipPythonCheck
)

# ── Color helpers ──────────────────────────────────────────────────────────────
function Write-Header($msg) { Write-Host "`n══════════════════════════════════════" -ForegroundColor Cyan
                               Write-Host "  $msg" -ForegroundColor Cyan
                               Write-Host "══════════════════════════════════════`n" -ForegroundColor Cyan }
function Write-Step($msg)    { Write-Host "🔄 $msg" -ForegroundColor Blue }
function Write-Success($msg) { Write-Host "✅ $msg" -ForegroundColor Green }
function Write-Warn($msg)    { Write-Host "⚠️  $msg" -ForegroundColor Yellow }
function Write-Fail($msg)    { Write-Host "❌ $msg" -ForegroundColor Red }

# ── Main ───────────────────────────────────────────────────────────────────────
Write-Header "🚀 ANTIGRAVITY — Code Search Tools Installer"
Write-Host "Cài đặt: CodeGraph (npm) + Semble (pip)" -ForegroundColor Gray
Write-Host "Docs: skills/mk11/codegraph-semble/GUIDE.md`n" -ForegroundColor Gray

$hasError = $false

# ── FIX: PowerShell ExecutionPolicy ──────────────────────────────────────
Write-Step "Kiểm tra PowerShell ExecutionPolicy..."
$policy = Get-ExecutionPolicy -Scope CurrentUser
if ($policy -eq "Restricted") {
    Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser -Force
    Write-Success "ExecutionPolicy đã đổi thành RemoteSigned (cần thiết cho npm scripts)"
} else {
    Write-Success "ExecutionPolicy: $policy (OK)"
}

# ── CHECK: Node.js ──────────────────────────────────────────────────────────
if (-not $SkipNodeCheck) {
    Write-Step "Kiểm tra Node.js..."
    try {
        $nodeVersion = node --version 2>&1
        if ($LASTEXITCODE -eq 0) {
            Write-Success "Node.js $nodeVersion đã sẵn sàng"
        } else {
            throw "Node not found"
        }
    } catch {
        Write-Fail "Node.js chưa được cài đặt!"
        Write-Warn "Tải Node.js tại: https://nodejs.org/ (cần v18+)"
        $hasError = $true
    }
}

# ── CHECK: Python ──────────────────────────────────────────────────────────
if (-not $SkipPythonCheck) {
    Write-Step "Kiểm tra Python..."
    try {
        $pythonVersion = python --version 2>&1
        if ($LASTEXITCODE -eq 0) {
            Write-Success "$pythonVersion đã sẵn sàng"
        } else {
            throw "Python not found"
        }
    } catch {
        # Thử python3
        try {
            $pythonVersion = python3 --version 2>&1
            Write-Success "$pythonVersion đã sẵn sàng (dùng python3)"
        } catch {
            Write-Fail "Python chưa được cài đặt!"
            Write-Warn "Tải Python tại: https://python.org/ (cần v3.10+)"
            $hasError = $true
        }
    }
}

# Dừng nếu thiếu prerequisites
if ($hasError) {
    Write-Host "`n❌ Cài đặt thất bại: Thiếu prerequisites ở trên." -ForegroundColor Red
    Write-Host "Sau khi cài xong Node.js/Python, chạy lại script này." -ForegroundColor Yellow
    exit 1
}

# ── INSTALL: CodeGraph ─────────────────────────────────────────────────────
Write-Header "📦 Cài đặt CodeGraph"
Write-Step "Chạy: npm install -g @colbymchenry/codegraph"

try {
    npm install -g @colbymchenry/codegraph
    if ($LASTEXITCODE -eq 0) {
        $cgVersion = codegraph --version 2>&1
        Write-Success "CodeGraph đã cài xong! Version: $cgVersion"
    } else {
        throw "npm install failed"
    }
} catch {
    Write-Fail "Cài CodeGraph thất bại: $_"
    Write-Warn "Thử chạy lại với quyền Administrator"
    $hasError = $true
}

# ── INSTALL: Semble ─────────────────────────────────────────────────────────
Write-Header "📦 Cài đặt Semble"
Write-Step "Chạy: pip install semble"

try {
    pip install semble
    if ($LASTEXITCODE -eq 0) {
        # Fix: Thêm Python Scripts vào PATH nếu chưa có (Windows pip quirk)
        $pythonScripts = "$env:APPDATA\Python\Python313\Scripts"
        if (Test-Path $pythonScripts) {
            $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
            if ($currentPath -notlike "*$pythonScripts*") {
                [Environment]::SetEnvironmentVariable("Path", "$currentPath;$pythonScripts", "User")
                $env:PATH = "$env:PATH;$pythonScripts"
                Write-Success "Đã thêm Python Scripts vào PATH: $pythonScripts"
            }
        }
        Write-Success "Semble đã cài xong!"
    } else {
        throw "pip install failed"
    }
} catch {
    Write-Fail "Cài Semble thất bại: $_"
    Write-Warn "Nếu lỗi PATH, thêm '%APPDATA%\Python\Scripts' vào biến môi trường PATH"
    $hasError = $true
}

# ── VERIFY ─────────────────────────────────────────────────────────────────
Write-Header "🔍 Kiểm tra Cài đặt"

Write-Step "Verify codegraph..."
$cgCheck = Get-Command codegraph -ErrorAction SilentlyContinue
if ($cgCheck) { Write-Success "codegraph: $($cgCheck.Source)" }
else          { Write-Fail "codegraph không tìm thấy trong PATH"; $hasError = $true }

Write-Step "Verify semble..."
$sCheck = Get-Command semble -ErrorAction SilentlyContinue
if ($sCheck) { Write-Success "semble: $($sCheck.Source)" }
else         { Write-Fail "semble không tìm thấy trong PATH"; $hasError = $true }

# ── SUMMARY ────────────────────────────────────────────────────────────────
Write-Header "📊 Kết quả"

if (-not $hasError) {
    Write-Success "Tất cả công cụ đã sẵn sàng!"
    Write-Host ""
    Write-Host "📖 Bước tiếp theo:" -ForegroundColor Cyan
    Write-Host "  1. Khởi tạo CodeGraph cho project:" -ForegroundColor White
    Write-Host "     .\scripts\init-codegraph.ps1" -ForegroundColor Gray
    Write-Host "  2. Đọc hướng dẫn sử dụng:" -ForegroundColor White
    Write-Host "     skills\mk11\codegraph-semble\GUIDE.md" -ForegroundColor Gray
    Write-Host "  3. Xem rule ưu tiên dùng tool:" -ForegroundColor White
    Write-Host "     rules\GEMINI.md → Section 7" -ForegroundColor Gray
} else {
    Write-Fail "Cài đặt chưa hoàn thành. Kiểm tra lỗi ở trên."
    exit 1
}
