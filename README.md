# Simple File Manager

一个简单的Rust文件管理服务器，提供前端UI和后端API，支持用户认证和细粒度权限控制。

## 功能

- ✅ 用户登录认证
- ✅ 文件浏览（支持权限控制）
- ✅ 文件上传（支持权限控制）
- ✅ 文件下载（支持权限控制）
- ✅ 灵活的权限配置系统
- ✅ HTTPS 支持

## 配置文件

配置文件位于项目根目录的 `config.toml`，用于定义用户和其权限。

### 配置格式

```toml
# 文件管理器用户权限配置

[[users]]
username = "admin"
password = "password"
permissions = ["*"]

[[users]]
username = "user1"
password = "user1pass"
permissions = ["file1.txt", "folder1/*"]

[[users]]
username = "user2"
password = "user2pass"
permissions = ["folder2/*", "shared.pdf"]
```

### 权限配置说明

- `["*"]` - 全部权限，可以访问所有文件
- `["file1.txt"]` - 精确文件名，只能访问该文件
- `["folder1/*"]` - 通配符，可以访问 `folder1` 文件夹内的所有文件

## 启用 HTTPS

要启用 HTTPS 支持，需要准备 TLS 证书和私钥文件：

1. 在项目根目录下创建 `certs` 文件夹：
   ```bash
   mkdir certs
   ```

2. 生成自签名证书（仅用于测试）：
   ```bash
   # 安装 mkcert（如果尚未安装）
   # macOS: brew install mkcert
   # Windows: choco install mkcert
   # Linux: sudo apt install libnss3-tools && curl -JLO "https://github.com/FiloSottile/mkcert/releases/download/v1.4.3/mkcert-v1.4.3-linux-amd64" && chmod +x mkcert-v1.4.3-linux-amd64 && sudo mv mkcert-v1.4.3-linux-amd64 /usr/local/bin/mkcert

   # 生成证书
   mkcert -install
   mkcert -key-file certs/key.pem -cert-file certs/cert.pem localhost 127.0.0.1 ::1
   ```

3. 启动服务器后，它将自动检测证书文件并同时运行 HTTP 和 HTTPS 服务

## 使用方法

### 启动服务器

```bash
cargo run
```

服务器将在 `http://127.0.0.1:8080` 和 `https://127.0.0.1:8443` 启动

> 注意：如果项目根目录的 `certs` 文件夹中存在 `cert.pem` 和 `key.pem`，HTTPS 服务将自动启用。

### 前端使用

1. 打开浏览器访问 `http://127.0.0.1:8080` 或 `https://127.0.0.1:8443`（如果启用了 HTTPS）
2. 在登录框中输入用户名和密码
3. 登录成功后，可以看到该用户有权限访问的文件列表
4. 支持上传和下载操作

### 默认用户

- **username**: `admin`
- **password**: `password`
- **权限**: 全部文件（*）

### 测试用户

- **username**: `user1`
- **password**: `user1pass`
- **权限**: `file1.txt`, `folder1/*`

## 项目结构

```
simple_file_manager/
├── src/
│   └── main.rs           # 后端API实现
├── static/
│   └── index.html        # 前端页面
├── files/                # 文件存储目录
├── certs/                # HTTPS 证书目录（可选）
│   ├── cert.pem          # 证书文件
│   └── key.pem           # 私钥文件
├── config.toml           # 用户和权限配置（TOML格式）
├── Cargo.toml            # 依赖配置
└── README.md             # 本文件
```

## 技术栈

- **框架**: Axum (Rust web框架)
- **异步运行时**: Tokio
- **序列化**: Serde + serde_json
- **前端**: 原生HTML + JavaScript
- **HTTPS 支持**: rustls + axum-server

## API 文档

### 登录
**POST** `/api/login`
```json
{
  "username": "admin",
  "password": "password"
}
```

**响应**:
```json
{
  "success": true,
  "token": "token-admin"
}
```

### 获取文件列表
**GET** `/api/files`

**Header**: `Authorization: Bearer token-admin`

**响应**:
```json
{
  "files": [
    {"name": "file1.txt", "is_dir": false},
    {"name": "folder1", "is_dir": true}
  ]
}
```

### 上传文件
**POST** `/api/upload`

**Header**: `Authorization: Bearer token-admin`

**Body**: multipart/form-data (file field)

### 下载文件
**GET** `/api/download?name=file1.txt&token=token-admin`

## 权限检查机制

- 所有API操作都需要有效的token认证