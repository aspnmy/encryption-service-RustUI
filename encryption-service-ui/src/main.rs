use eframe::NativeOptions;

mod app;
mod models;
mod api;
mod services;
mod config;

fn main() -> Result<(), eframe::Error> {
    // 初始化日志
    tracing_subscriber::fmt::init();
    
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_resizable(true)
            .with_title("加密服务管理器"),
        ..Default::default()
    };
    
    eframe::run_native(
        "加密服务管理器",
        options,
        Box::new(|cc| Box::new(app::App::new(cc))),
    )
}

