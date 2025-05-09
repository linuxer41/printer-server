// src/main.rs
#[cfg(windows)]
mod windows_service_runner;

mod printer_service;
mod models;

#[tokio::main]
async fn main() {
    #[cfg(windows)]
    {
        if std::env::args().any(|a| a == "--run-as-service") {
            windows_service_runner::run_service();
            return;
        }
    }

    // Modo normal (debug, consola)
    printer_service::run_server().await;
} 