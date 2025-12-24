# 模块结构和导出说明

## 项目模块架构

```
src/
├── main.rs               # 应用入口
├── model.rs              # 全局数据模型
├── utils.rs              # 工具函数
├── router.rs             # 路由配置
├── dao/                  # 数据访问层
│   ├── mod.rs            # DAO模块入口（导出Config、UserConfig、AppState）
│   └── config.rs         # 配置相关结构体定义
└── handler/              # 请求处理器
    ├── mod.rs            # Handler模块入口
    ├── login.rs          # 登录处理
    ├── files.rs          # 文件列表处理
    ├── upload.rs         # 上传处理
    └── download.rs       # 下载处理
```

## 结构体导出方式

### 1. 在 `src/dao/config.rs` 中定义结构体

```rust
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct UserConfig {
    pub username: String,
    pub password: String,
    pub permissions: Vec<String>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    pub users: Vec<UserConfig>,
}
```

**关键点**：
- 使用 `pub` 标记结构体为公开
- 结构体字段也用 `pub` 标记，使其可在外部访问

### 2. 在 `src/dao/mod.rs` 中导出结构体

```rust
pub mod config;  // 声明config模块

// 导出公开的结构体供外部使用
pub use config::{Config, UserConfig};
```

**导出方式说明**：
- `pub mod config;` - 声明config模块为公开
- `pub use config::Config;` - 重新导出Config，使其可从dao直接访问

### 3. 在其他模块中使用

#### 方式一：通过dao模块导入
```rust
use crate::dao::{Config, UserConfig, AppState};
```

#### 方式二：通过完整路径
```rust
use crate::dao::config::Config;
```

#### 方式三：在utils.rs中使用
```rust
use crate::dao::Config;

pub fn has_permission(config: &Config, username: &str, file_name: &str) -> bool {
    // 使用Config结构体
}
```

## 导出访问权限说明

| 项目 | 位置 | 导出方式 | 访问范围 |
|------|------|---------|---------|
| `Config` | dao/config.rs | `pub use config::Config` | 整个项目 |
| `UserConfig` | dao/config.rs | `pub use config::UserConfig` | 整个项目 |
| `AppState` | dao/mod.rs | 直接定义 `pub struct` | 整个项目 |
| 内部函数 | utils.rs | `pub fn` | 整个项目 |

## 私有结构体

如果不想导出某个结构体，只需删除 `pub` 关键字：

```rust
// 私有结构体，只能在config.rs内部使用
struct InternalConfig {
    ...
}
```

## 完整使用示例

### handler/login.rs
```rust
use crate::dao::AppState;  // 导入AppState
use crate::utils;

async fn login(
    State(state): State<AppState>,  // 使用AppState
    Json(payload): Json<LoginRequest>,
) -> Json<LoginResponse> {
    // state.config 可以直接访问，类型为 Arc<Config>
    state.config.users.iter()...
}
```

### handler/files.rs
```rust
use crate::utils;

pub fn check_permission(config: &crate::dao::Config, user: &str, file: &str) {
    // 使用导出的Config结构体
}
```

## 总结

✅ **导出步骤**：
1. 在 `config.rs` 中用 `pub` 定义结构体
2. 在 `mod.rs` 中用 `pub use` 重新导出
3. 在其他模块中用 `use crate::dao::StructName;` 导入使用

✅ **优势**：
- 结构清晰，便于维护
- 访问权限控制灵活
- 易于理解模块间的依赖关系
- 支持未来的重构和扩展
