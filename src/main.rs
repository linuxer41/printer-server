// src/main.rs
#[cfg(windows)]
mod windows_service_runner;

mod printer_service;
mod models;

use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

#[tokio::main]
async fn main() {
    // Configurar el logger con fecha y hora
    Builder::new()
        .format(|buf, record| {
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
            writeln!(
                buf,
                "[{}] {} [{}] - {}",
                timestamp,
                record.level(),
                record.target(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    log::info!("Iniciando servidor de impresión");
    
    #[cfg(windows)]
    {
        if std::env::args().any(|a| a == "--run-as-service") {
            log::info!("Iniciando como servicio de Windows");
            windows_service_runner::run_service();
            return;
        }
    }

    // Modo normal (debug, consola)
    log::info!("Iniciando en modo consola");
    printer_service::run_server().await;
} 