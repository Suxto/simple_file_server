# Simple File Manager

一个简单的Rust文件管理服务器，提供前端UI和后端API，支持用户认证和细粒度权限控制。

## 功能

- ✅ 用户登录认证
- ✅ 文件浏览（支持权限控制）
- ✅ 文件上传（支持权限控制）
- ✅ 文件下载（支持权限控制）
- ✅ 灵活的权限配置系统

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

## 使用方法

### 启动服务器

```bash
cargo run
```

服务器将在 `http://127.0.0.1:8080` 启动

### 前端使用

1. 打开浏览器访问 `http://127.0.0.1:8080`
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
├── config.toml           # 用户和权限配置（TOML格式）
├── Cargo.toml            # 依赖配置
└── README.md             # 本文件
```

## 技术栈

- **框架**: Axum (Rust web框架)
- **异步运行时**: Tokio
- **序列化**: Serde + serde_json
- **前端**: 原生HTML + JavaScript

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
- 文件浏览、上传、下载操作都受权限控制
- 权限不足时返回 `403 Forbidden`
- 用户只能访问config.json中定义的权限范围内的文件

## 修改配置

修改 `config.toml` 后，需要重启服务器使新配置生效。

## 安全建议

- 在生产环境中，请不要使用简单的明文密码存储，建议使用加密或哈希处理
- 建议为每个用户配置最小必要权限
- 定期更新依赖包以获得安全更新
