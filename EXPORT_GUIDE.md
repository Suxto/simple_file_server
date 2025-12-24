# Rust 模块导出快速参考

## 问题：如何导出结构体？

**答：使用 `pub use` 重新导出**

## 三步导出法

### 1️⃣ 在 `config.rs` 中定义（带 `pub`）
```rust
// src/dao/config.rs
pub struct Config {
    pub users: Vec<UserConfig>,
}
```

### 2️⃣ 在 `mod.rs` 中声明模块并导出
```rust
// src/dao/mod.rs
pub mod config;              // 声明模块
pub use config::Config;      // 导出结构体
```

### 3️⃣ 在其他地方使用
```rust
// 任何文件中
use crate::dao::Config;      // 直接从dao导入

fn my_func(config: &Config) {
    // 使用Config
}
```

---

## 完整示例

### 现有项目结构

```
src/dao/
├── mod.rs           ← 导出点
│   pub use config::{Config, UserConfig};
│
└── config.rs        ← 定义点
    pub struct Config { ... }
    pub struct UserConfig { ... }
```

### 在其他文件中的使用

**handler/login.rs**
```rust
use crate::dao::AppState;  // 使用导出的AppState

async fn login(State(state): State<AppState>) {
    // state.config 的类型是 Arc<Config>
    // Config来自dao模块
}
```

**utils.rs**
```rust
use crate::dao::Config;  // 使用导出的Config

pub fn has_permission(config: &Config, ...) {
    // 直接使用Config
}
```

---

## 关键语法对比

| 代码 | 含义 | 可见范围 |
|------|------|--------|
| `struct Config` | 私有结构体 | 仅当前文件 |
| `pub struct Config` | 公开结构体 | 仅当前模块 |
| `pub use config::Config` | 重新导出 | 上级模块及外部 |
| `pub mod config` | 公开模块 | 上级模块及外部 |

---

## 常见场景

### ✅ 导出单个项
```rust
// mod.rs
pub use config::Config;

// 使用
use crate::dao::Config;
```

### ✅ 导出多个项
```rust
// mod.rs
pub use config::{Config, UserConfig};

// 使用
use crate::dao::{Config, UserConfig};
```

### ✅ 导出整个模块
```rust
// mod.rs
pub mod config;

// 使用
use crate::dao::config::Config;
```

### ❌ 错误写法（不导出）
```rust
// mod.rs (没有pub use)
mod config;  // ← 私有模块

// 其他文件 (无法访问)
use crate::dao::Config;  // ✗ 编译错误！
```

---

## 项目中的实际应用

当前项目中：

**`src/dao/mod.rs`** - 导出数据结构
```rust
pub mod config;
pub use config::{Config, UserConfig};  // ← 关键导出
```

**`src/utils.rs`** - 使用导出的结构体
```rust
use crate::dao::Config;  // ← 导入导出的Config

pub fn has_permission(config: &Config, ...) { ... }
```

**`src/handler/login.rs`** - 使用状态中的结构体
```rust
use crate::dao::AppState;  // ← AppState包含Arc<Config>

async fn login(State(state): State<AppState>) {
    // state.config 是 Arc<Config>
}
```

---

## 检查清单

- [ ] 在 `config.rs` 中用 `pub` 标记结构体
- [ ] 在 `config.rs` 中用 `pub` 标记结构体字段（需要外部访问时）
- [ ] 在 `mod.rs` 中用 `pub use` 导出结构体
- [ ] 在其他模块中用 `use crate::dao::StructName;` 导入

✅ 完成这些就可以正常使用导出的结构体了！
