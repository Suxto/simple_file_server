# 权限配置指南

## 概述

文件管理器使用 TOML 配置文件来管理用户和权限。配置文件 `config.toml` 定义了：
- 哪些用户可以登录
- 每个用户的密码
- 每个用户可以访问的文件和文件夹

## 配置文件位置

项目根目录：`config.toml`

## 配置文件结构

```toml
[[users]]
username = "用户名"
password = "密码"
permissions = ["权限1", "权限2", "权限3"]
```

## 权限说明

### 1. 全部权限 (*)

```toml
[[users]]
username = "admin"
password = "admin123"
permissions = ["*"]
```

- 用户可以访问所有文件和文件夹
- 可以上传、下载、浏览任何文件

### 2. 精确文件权限

```toml
[[users]]
username = "user1"
password = "user1pass"
permissions = ["file1.txt", "document.pdf"]
```

- 用户只能访问明确列出的文件
- 可以下载、上传这些指定文件

### 3. 文件夹权限 (通配符)

```toml
[[users]]
username = "user2"
password = "user2pass"
permissions = ["documents/*", "images/*"]
```

- 用户可以访问 `documents` 和 `images` 文件夹内的所有文件
- 使用 `folder/*` 模式表示文件夹内的所有文件

### 4. 混合权限

```toml
[[users]]
username = "user3"
password = "user3pass"
permissions = ["*", "shared.txt", "private/*"]
```

## 实际配置示例

### 示例 1：简单配置

```toml
[[users]]
username = "admin"
password = "admin123"
permissions = ["*"]

[[users]]
username = "guest"
password = "guest123"
permissions = ["public/*"]
```

此配置中：
- `admin` 可以访问所有文件
- `guest` 只能访问 `public` 文件夹内的文件

### 示例 2：多用户分权限

```toml
[[users]]
username = "admin"
password = "securepass123"
permissions = ["*"]

[[users]]
username = "manager"
password = "managerpass"
permissions = ["reports/*", "analytics/*", "shared.xlsx"]

[[users]]
username = "intern"
password = "internpass"
permissions = ["training/*", "templates/template.docx"]
```

此配置中：
- `admin` - 管理员，全部权限
- `manager` - 经理，可以访问报表和分析文件夹，以及共享的Excel文件
- `intern` - 实习生，只能访问培训材料和模板

### 示例 3：按部门分权限

```toml
[[users]]
username = "hr_admin"
password = "hr123"
permissions = ["hr/*", "shared/*"]

[[users]]
username = "finance_admin"
password = "finance123"
permissions = ["finance/*", "shared/*"]

[[users]]
username = "it_admin"
password = "it123"
permissions = ["it/*", "shared/*"]

[[users]]
username = "all_employee"
password = "employee123"
permissions = ["shared/*"]
```

此配置适合部门级别的访问控制。

## 权限匹配规则

1. **精确匹配**：`file.txt` 匹配 `files/file.txt` (同名文件)
2. **通配符匹配**：`folder/*` 匹配 `folder/` 目录下的所有文件
3. **全部权限**：`*` 匹配所有文件和文件夹
4. **优先级**：检查顺序为 全部权限 → 精确匹配 → 通配符匹配

## 修改配置的步骤

1. 编辑 `config.toml` 文件
2. 修改用户信息或权限
3. 保存文件
4. **重启服务器** （重新运行 `cargo run`）
5. 新配置将立即生效

## 常见问题

### Q: 如何删除用户？
A: 从 `users` 数组中移除该用户对象，然后重启服务器。

### Q: 如何改密码？
A: 修改用户对象中的 `password` 字段，然后重启服务器。

### Q: 如何添加新文件夹权限？
A: 在用户的 `permissions` 数组中添加 `"foldername/*"` 格式的权限。

### Q: 配置不生效怎么办？
A: 确保：
1. 配置文件格式正确（TOML 格式）
2. 已重启服务器
3. 权限字符串格式正确（区分大小写）
4. 用户已重新登录

### Q: 忘记密码怎么办？
A: 编辑 `config.toml`，修改 `password` 字段为新密码，重启服务器。

## 安全建议

1. **强密码**：使用复杂的密码，包含大小写字母、数字和特殊字符
2. **最小权限原则**：只给用户必要的权限
3. **定期审查**：定期检查和更新权限配置
4. **备份**：定期备份 `config.json` 文件
5. **HTTPS**：在生产环境中使用 HTTPS 加密传输
6. **密码加密**：考虑对密码使用加密存储而非明文

## 权限测试

### 使用 curl 测试登录

```bash
curl -X POST http://127.0.0.1:8080/api/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"password"}'
```

### 使用 curl 测试文件列表

```bash
curl -H "Authorization: Bearer token-admin" \
  http://127.0.0.1:8080/api/files
```

### 使用 curl 测试下载

```bash
curl "http://127.0.0.1:8080/api/download?name=file1.txt&token=token-admin" \
  -O
```
