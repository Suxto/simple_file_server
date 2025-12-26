#!/bin/bash

# Bash 脚本用于在 Linux/macOS 上生成自签名证书
# 用于为 Simple File Manager 启用 HTTPS

echo "正在为 Simple File Manager 生成 HTTPS 证书..."

# 检查是否安装了 mkcert
if ! command -v mkcert &> /dev/null; then
    echo "错误: 未找到 mkcert。请先安装 mkcert:"
    echo "  macOS: brew install mkcert"
    echo "  Linux: 请参考 https://github.com/FiloSottile/mkcert#installation"
    echo "  或者从 https://github.com/FiloSottile/mkcert/releases 下载"
    exit 1
fi

# 创建 certs 目录
CERTS_DIR="certs"
if [ ! -d "$CERTS_DIR" ]; then
    mkdir -p "$CERTS_DIR"
    echo "创建目录: $CERTS_DIR"
fi

# 安装本地 CA (仅首次运行需要)
echo "正在安装本地 CA..."
mkcert -install

# 生成证书
echo "正在生成证书..."
CERT_PATH="$CERTS_DIR/cert.pem"
KEY_PATH="$CERTS_DIR/key.pem"

# 为 localhost, 127.0.0.1 和 ::1 生成证书
mkcert -key-file "$KEY_PATH" -cert-file "$CERT_PATH" "localhost" "127.0.0.1" "::1"

if [ -f "$CERT_PATH" ] && [ -f "$KEY_PATH" ]; then
    echo "证书已成功生成!"
    echo "证书文件: $CERT_PATH"
    echo "私钥文件: $KEY_PATH"
    echo ""
    echo "现在可以使用 HTTPS 启动服务器了。"
else
    echo "证书生成失败!"
    exit 1
fi