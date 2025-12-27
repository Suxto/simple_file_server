mod extractors;
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

// 全局变量保存 tracing 的 guard，防止其被丢弃
static mut LOG_GUARD: Option<tracing_appender::non_blocking::WorkerGuard> = None;

#[tokio::main]
async fn main() {
    before_start();

    // 加载配置
    let config = match model::ConfigFromFile::from_toml("config.toml").await {
        Ok(c) => Arc::new(c),
        Err(e) => {
            error!("配置文件加载失败: {}", e);
            std::process::exit(1);
        }
    };

    let state = model::AppState::new_form_config(config).await;
    let app = router::create_router(state);

    // 从命令行参数或环境变量获取端口配置
    let https_port: u16 = std::env::var("HTTPS_PORT")
        .unwrap_or_else(|_| "8443".to_string())
        .parse()
        .expect("HTTPS 端口必须是数字");

    let http_port: u16 = std::env::var("HTTP_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("HTTP 端口必须是数字");

    let https_addr = SocketAddr::from(([127, 0, 0, 1], https_port));
    let http_addr = SocketAddr::from(([127, 0, 0, 1], http_port));

    // 检查是否有证书文件，如果有则启动 HTTPS 服务器
    if std::path::Path::new("certs/cert.pem").exists()
        && std::path::Path::new("certs/key.pem").exists()
    {
        info!("检测到证书文件，启动 HTTPS 服务器...");

        // // 创建 TLS 配置
        // let rustls_config =
        //     axum_server::tls_rustls::RustlsConfig::from_pem_file("certs/cert.pem", "certs/key.pem")
        //         .await
        //         .expect("无法加载 TLS 证书/私钥");

        // // 启动 HTTPS 服务器
        // info!("HTTPS 服务器运行在 https://{}", https_addr);
        // let https_listener = match tokio::net::TcpListener::bind(&https_addr).await {
        //     Ok(l) => l,
        //     Err(e) => {
        //         error!("HTTPS 端口绑定失败: {}", e);
        //         std::process::exit(1);
        //     }
        // };

        // let https_server = axum_server::from_tcp_rustls(
        //     https_listener.into_std().expect("转换 TcpListener 失败"),
        //     rustls_config,
        // )
        // .serve(app.clone().into_make_service());

        // // 同时启动 HTTP 服务器作为备选
        // info!("HTTP 服务器运行在 http://{}", http_addr);
        // let http_listener = match tokio::net::TcpListener::bind(&http_addr).await {
        //     Ok(l) => l,
        //     Err(e) => {
        //         error!("HTTP 端口绑定失败: {}", e);
        //         std::process::exit(1);
        //     }
        // };

        // let http_server = axum::serve(http_listener, app.into_make_service());

        // // 并行运行 HTTP 和 HTTPS 服务器
        // tokio::select! {
        //     result = https_server => {
        //         if let Err(e) = result {
        //             error!("HTTPS 服务器错误: {}", e);
        //             std::process::exit(1);
        //         }
        //     },
        //     result = http_server => {
        //         if let Err(e) = result {
        //             error!("HTTP 服务器错误: {}", e);
        //             std::process::exit(1);
        //         }
        //     }
        // }
    } else {
        info!("未找到证书文件，仅启动 HTTP 服务器");
        info!("服务器运行在 http://{}", http_addr);
        let listener = match tokio::net::TcpListener::bind(&http_addr).await {
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

    // 文件层 - 不使用颜色格式
    let file_appender = tracing_appender::rolling::RollingFileAppender::new(
        tracing_appender::rolling::Rotation::DAILY,
        log_dir,
        "app.log",
    );
    let (non_blocking_file, guard) = tracing_appender::non_blocking(file_appender);

    // 将 guard 存储到全局变量中，防止其被丢弃
    unsafe {
        LOG_GUARD = Some(guard);
    }

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking_file)
        .with_span_events(FmtSpan::CLOSE)
        .with_target(true)
        .with_level(true)
        .with_ansi(false);

    // 控制台层 - 可以选择是否显示颜色
    let stdout_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stdout)
        .with_span_events(FmtSpan::CLOSE)
        .with_target(true)
        .with_level(true)
        .with_ansi(true);

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
