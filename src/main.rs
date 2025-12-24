mod dao;
mod handler;
mod model;
mod router;
mod utils;

// 重新导出dao中的公开项，便于其他模块直接导入
pub use dao::{AppState, Config};

use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // 加载配置
    let config = match dao::load_config().await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("配置文件加载失败: {}", e);
            std::process::exit(1);
        }
    };

    // 创建应用状态
    let state = dao::app_state::AppState {
        config: Arc::new(config),
    };

    // 确保文件目录存在
    if let Err(e) = tokio::fs::create_dir_all(&state.root_dir).await {
        eprintln!("创建文件目录失败: {}", e);
        std::process::exit(1);
    }

    // 创建路由
    let app = router::create_router(state);

    // 启动服务器
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Server running at http://{}", addr);
    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("端口绑定失败: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("服务器错误: {}", e);
        std::process::exit(1);
    }
}
