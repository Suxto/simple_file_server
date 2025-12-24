mod handler;
mod model;
mod router;
mod utils;

use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{error, info};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() {
    before_start();

    // 加载配置
    let config = match model::Config::from_toml("config.toml").await {
        Ok(c) => c,
        Err(e) => {
            error!("配置文件加载失败: {}", e);
            std::process::exit(1);
        }
    };

    let state = model::AppState::new_form_config(Arc::new(config));
    let app = router::create_router(state);

    // 启动服务器
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("Server running at http://{}", addr);
    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(l) => l,
        Err(e) => {
            error!("端口绑定失败: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = axum::serve(listener, app).await {
        error!("服务器错误: {}", e);
        std::process::exit(1);
    }
}

fn before_start() {
    init_tracing();
}

fn init_tracing() {
    // 创建日志目录
    let log_dir = "log";
    if !std::path::Path::new(log_dir).exists() {
        std::fs::create_dir(log_dir).expect("创建日志目录失败");
    }

    // 文件层
    let file_appender = tracing_appender::rolling::RollingFileAppender::new(
        tracing_appender::rolling::Rotation::DAILY,
        log_dir,
        "app.log",
    );
    let (non_blocking_file, _guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking_file)
        .with_span_events(FmtSpan::CLOSE)
        .with_target(true)
        .with_level(true);

    // 控制台层
    let stdout_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stdout)
        .with_span_events(FmtSpan::CLOSE)
        .with_target(true)
        .with_level(true);

    // 组合两个层
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with(stdout_layer)
        .with(file_layer)
        .init();
}
