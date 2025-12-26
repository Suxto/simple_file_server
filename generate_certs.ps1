# PowerShell 脚本用于在 Windows 上生成自签名证书
# 用于为 Simple File Manager 启用 HTTPS

Write-Host "正在为 Simple File Manager 生成 HTTPS 证书..." -ForegroundColor Green

# 检查是否安装了 mkcert
if (!(Get-Command mkcert -ErrorAction SilentlyContinue)) {
    Write-Host "错误: 未找到 mkcert。请先安装 mkcert:" -ForegroundColor Red
    Write-Host "  Windows: choco install mkcert" -ForegroundColor Red
    Write-Host "  或者从 https://github.com/FiloSottile/mkcert/releases 下载" -ForegroundColor Red
    exit 1
}

# 创建 certs 目录
$certsDir = "certs"
if (!(Test-Path $certsDir)) {
    New-Item -ItemType Directory -Path $certsDir | Out-Null
    Write-Host "创建目录: $certsDir" -ForegroundColor Yellow
}

# 安装本地 CA (仅首次运行需要)
Write-Host "正在安装本地 CA..." -ForegroundColor Yellow
mkcert -install

# 生成证书
Write-Host "正在生成证书..." -ForegroundColor Yellow
$certPath = "$certsDir/cert.pem"
$keyPath = "$certsDir/key.pem"

# 为 localhost, 127.0.0.1 和 ::1 生成证书
mkcert -key-file "$keyPath" -cert-file "$certPath" "localhost" "127.0.0.1" "::1"

if (Test-Path $certPath -PathType Leaf -and Test-Path $keyPath -PathType Leaf) {
    Write-Host "证书已成功生成!" -ForegroundColor Green
    Write-Host "证书文件: $certPath" -ForegroundColor Cyan
    Write-Host "私钥文件: $keyPath" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "现在可以使用 HTTPS 启动服务器了。" -ForegroundColor Green
} else {
    Write-Host "证书生成失败!" -ForegroundColor Red
    exit 1
}