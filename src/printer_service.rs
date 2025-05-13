use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get, Json, Router
};
use serde_json::json;
use std::net::SocketAddr;
use std::process::Command;
use tokio::net::TcpListener;
use log::{info, error};
use printers::{get_printer_by_name, get_default_printer, get_printers};
use tower_http::cors::{Any, CorsLayer};

use crate::models::{PrinterInfo, PrintParams};

// Error personalizado
struct AppError(String);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        error!("Error: {}", self.0);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": self.0 })),
        ).into_response()
    }
}

pub async fn run_server() {
    // Configurar CORS para permitir peticiones de cualquier origen
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    let app = Router::new()
        .route("/printers", get(list_printers))
        .route("/print", get(print_pdf))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));
    info!("Servidor iniciado en http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Imprimir PDF
async fn print_pdf(
    Query(params): Query<PrintParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    info!("Solicitud de impresión recibida: {:?}", params);
    
    // Validar URL
    if params.url.is_empty() {
        return Err(AppError("No se proporcionó URL del PDF".to_string()));
    }
    
    // Determinar impresora a usar
    let printer_name = match &params.printer {
        Some(name) if !name.is_empty() => {
            info!("Usando impresora: {}", name);
            name.clone()
        },
        _ => {
            // Buscar impresora predeterminada
            let default_printer = get_default_printer()
                .ok_or_else(|| AppError("No hay impresora predeterminada disponible".to_string()))?;
            
            info!("Usando impresora predeterminada: {}", default_printer.name);
            default_printer.name
        }
    };

    // Descargar PDF
    info!("Descargando PDF desde: {}", params.url);
    let temp_path = download_pdf(&params.url).await
        .map_err(|e| AppError(format!("Error al descargar PDF: {}", e)))?;
    
    // Enviar a impresora 
    info!("Enviando a impresora: {}", printer_name);
    
    if cfg!(target_os = "windows") {
        // En Windows, usar SumatraPDF para imprimir PDFs
        if let Err(e) = print_with_sumatra(&temp_path, &printer_name).await {
            return Err(AppError(format!("Error al imprimir con SumatraPDF: {}", e)));
        }
    } else {
        // En Linux, usar la biblioteca printers
        let printer = get_printer_by_name(&printer_name)
            .ok_or_else(|| AppError(format!("No se pudo encontrar la impresora: {}", printer_name)))?;
        
        if let Err(e) = printer.print_file(&temp_path, Some(&format!("PDF de {}", params.url))) {
            return Err(AppError(format!("Error al imprimir: {}", e)));
        }
    }
    
    info!("Documento enviado a imprimir correctamente");
    Ok(Json(json!({ 
        "status": "success",
        "printer": printer_name
    })))
}

// Función para imprimir con SumatraPDF en Windows
async fn print_with_sumatra(path: &str, printer: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Buscar SumatraPDF en ubicaciones comunes
    let mut sumatra_paths = vec![
        "C:\\Program Files\\SumatraPDF\\SumatraPDF.exe".to_string(),
        "C:\\Program Files (x86)\\SumatraPDF\\SumatraPDF.exe".to_string(),
        "C:\\SumatraPDF\\SumatraPDF.exe".to_string()
    ];
    
    // Añadir directorio actual
    if let Ok(current_dir) = std::env::current_dir() {
        let current_dir_sumatra = current_dir.join("SumatraPDF.exe");
        if current_dir_sumatra.exists() {
            sumatra_paths.insert(0, current_dir_sumatra.to_string_lossy().to_string());
            info!("SumatraPDF encontrado en el directorio actual");
        }
        
        // También buscar en subdirectorios comunes
        for subdir in &["bin", "tools", "apps", "utils"] {
            let subdir_path = current_dir.join(subdir).join("SumatraPDF.exe");
            if subdir_path.exists() {
                sumatra_paths.insert(0, subdir_path.to_string_lossy().to_string());
                info!("SumatraPDF encontrado en subdirectorio: {}", subdir);
            }
        }
    }
    
    // Buscar SumatraPDF en las rutas y usarlo para imprimir
    for sumatra_exe in &sumatra_paths {
        if std::path::Path::new(sumatra_exe).exists() {
            info!("Imprimiendo con SumatraPDF: {}", sumatra_exe);
            
            let output = Command::new("powershell")
                .args([
                    "-Command",
                    &format!("Start-Process -FilePath \"{}\" -ArgumentList \"-print-to \"\"{}\"\" \"\"{}\"\"\"", 
                        sumatra_exe, printer, path)
                ])
                .output()?;
            
            if output.status.success() {
                return Ok(());
            }
            
            // Si falló, mostrar la salida para diagnóstico
            let stderr = String::from_utf8_lossy(&output.stderr);
            if !stderr.is_empty() {
                error!("Error al ejecutar SumatraPDF: {}", stderr);
            }
        }
    }
    
    // Si no se encontró SumatraPDF o todos los intentos fallaron, mostrar error
    Err("No se pudo encontrar SumatraPDF o falló la impresión. Asegúrate de tener SumatraPDF instalado.".into())
}

// Listar impresoras
async fn list_printers() -> Json<Vec<PrinterInfo>> {
    info!("Solicitud de listado de impresoras");
    
    // Usar la biblioteca printers para obtener todas las impresoras
    let mut printers_list = Vec::new();
    
    // Obtener la impresora predeterminada para marcarla
    let default_printer = get_default_printer();
    let default_printer_name = default_printer.as_ref().map(|p| p.name.clone()).unwrap_or_default();
    
    // Convertir la lista de impresoras al formato de nuestra API
    for printer in get_printers() {
        printers_list.push(PrinterInfo {
            name: printer.name.clone(),
            driver: printer.driver_name.clone(),
            is_default: printer.name == default_printer_name
        });
    }
    
    info!("Se encontraron {} impresoras", printers_list.len());
    Json(printers_list)
}

async fn download_pdf(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Descargar el PDF
    let response = reqwest::get(url).await?;
    
    if !response.status().is_success() {
        return Err(format!("Error HTTP {}", response.status()).into());
    }
    
    let bytes = response.bytes().await?;
    
    // Crear un directorio temporal para almacenar el PDF
    let temp_dir = std::env::temp_dir().join("printer_server_files");
    std::fs::create_dir_all(&temp_dir)?;
    
    // Generar un nombre de archivo único con extensión .pdf
    let file_name = format!("document_{}.pdf", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs());
    
    let file_path = temp_dir.join(file_name);
    std::fs::write(&file_path, &bytes)?;
    
    info!("PDF guardado en {}", file_path.display());
    
    Ok(file_path.to_string_lossy().into_owned())
} 
