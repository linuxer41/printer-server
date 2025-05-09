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
use std::io::Write;
use tempfile::NamedTempFile;

use crate::models::{PrinterInfo, PrintParams};

// Custom error response type
struct AppError(String);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": self.0 })),
        ).into_response()
    }
}

pub async fn run_server() {
    let app = Router::new()
        .route("/printers", get(list_printers))
        .route("/print", get(print_pdf));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));
    println!("✅ Servidor iniciado en http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Print PDF function
async fn print_pdf(
    Query(params): Query<PrintParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Obtenemos la impresora a usar (la proporcionada o la predeterminada)
    let printer_name = match params.printer {
        Some(name) if !name.is_empty() => name,
        _ => {
            // Buscar la impresora predeterminada
            let printers = get_printers_list()
                .map_err(|e| AppError(format!("Error al obtener impresoras: {}", e)))?;
            
            let default_printer = printers.iter()
                .find(|p| p.is_default)
                .ok_or_else(|| AppError("No se encontró ninguna impresora predeterminada".to_string()))?;
            
            default_printer.name.clone()
        }
    };

    // Download the PDF
    let temp_path = download_pdf(&params.url).await
        .map_err(|e| AppError(format!("Error downloading PDF: {}", e)))?;
    
    // Send to printer
    send_to_printer(&temp_path, &printer_name).await
        .map_err(|e| AppError(format!("Error printing: {}", e)))?;
    
    // Return success
    Ok(Json(json!({ 
        "status": "success",
        "printer": printer_name
    })))
}

// List printers endpoint
async fn list_printers() -> Json<Vec<PrinterInfo>> {
    match get_printers_list() {
        Ok(printers) => Json(printers),
        Err(_) => Json(vec![]),
    }
}

async fn download_pdf(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;
    
    let mut file = NamedTempFile::new()?;
    file.write_all(&bytes)?;
    let path = file.into_temp_path();
    
    Ok(path.to_str().unwrap().to_string())
}

async fn send_to_printer(path: &str, printer: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output = if cfg!(target_os = "windows") {
        Command::new("powershell")
            .args([
                "-Command",
                &format!("Start-Process -FilePath '{}' -Verb Print -ArgumentList '/p /h /t {}'", path, printer),
            ])
            .output()?
    } else {
        Command::new("lp")
            .args(["-d", printer, path])
            .output()?
    };

    if output.status.success() {
        Ok(())
    } else {
        Err(format!("Printer error: {}", String::from_utf8_lossy(&output.stderr)).into())
    }
}

fn get_printers_list() -> Result<Vec<PrinterInfo>, Box<dyn std::error::Error>> {
    let mut printers = Vec::new();

    if cfg!(target_os = "windows") {
        // Intentamos primero con PowerShell (si está disponible)
        let powershell_result = get_printers_with_powershell();
        
        if let Ok(ps_printers) = powershell_result {
            if !ps_printers.is_empty() {
                return Ok(ps_printers);
            }
        }
        
        // Si PowerShell falló o no devolvió impresoras, intentamos con wmic
        let wmic_result = get_printers_with_wmic();
        
        if let Ok(wmic_printers) = wmic_result {
            if !wmic_printers.is_empty() {
                return Ok(wmic_printers);
            }
        }
        
        // Último recurso: devolver al menos la impresora PDF
        printers.push(PrinterInfo {
            name: "Microsoft Print to PDF".to_string(),
            driver: "Microsoft PDF Writer".to_string(),
            is_default: true,
        });
    } else {
        let default_output = Command::new("lpstat").arg("-d").output()?;
        let default_printer = String::from_utf8_lossy(&default_output.stdout)
            .split_whitespace().last().unwrap_or("").to_string();

        let list_output = Command::new("lpstat").arg("-v").output()?;
        let stdout = String::from_utf8_lossy(&list_output.stdout);

        for line in stdout.lines() {
            if let Some(name_part) = line.strip_prefix("device for ") {
                if let Some((name, _)) = name_part.split_once(":") {
                    printers.push(PrinterInfo {
                        name: name.trim().to_string(),
                        driver: "Unknown".to_string(),
                        is_default: name.trim() == default_printer,
                    });
                }
            }
        }
    }

    Ok(printers)
}

// Función para obtener impresoras usando PowerShell
fn get_printers_with_powershell() -> Result<Vec<PrinterInfo>, Box<dyn std::error::Error>> {
    let mut printers = Vec::new();
    
    // Intentar usar PowerShell para obtener la lista de impresoras
    let output = Command::new("powershell")
        .args([
            "-Command",
            "Get-Printer | Select-Object Name, DriverName, Default | ConvertTo-Csv -NoTypeInformation"
        ])
        .output();
    
    match output {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = stdout.lines().collect();
            
            // Necesitamos al menos el encabezado y una línea de datos
            if lines.len() > 1 {
                // Saltamos la primera línea que contiene los encabezados
                for line in lines.iter().skip(1) {
                    // Parsear el formato CSV
                    let parts: Vec<&str> = line.split(',').collect();
                    if parts.len() >= 3 {
                        let name = parts[0].trim_matches('"').to_string();
                        let driver = parts[1].trim_matches('"').to_string();
                        let is_default = parts[2].trim_matches('"').eq_ignore_ascii_case("True");
                        
                        printers.push(PrinterInfo {
                            name,
                            driver,
                            is_default,
                        });
                    }
                }
            }
        }
        _ => {
            // PowerShell falló o no está disponible, devolvemos un error para que 
            // se pruebe el siguiente método
            return Err("PowerShell not available or failed".into());
        }
    }
    
    Ok(printers)
}

// Función para obtener impresoras usando wmic
fn get_printers_with_wmic() -> Result<Vec<PrinterInfo>, Box<dyn std::error::Error>> {
    let mut printers = Vec::new();
    
    // Intentamos obtener la lista de impresoras con wmic
    let output = Command::new("wmic")
        .args(["printer", "get", "Name,DriverName,Default", "/format:csv"])
        .output();
    
    match output {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = stdout.lines().collect();
            
            for line in lines.iter().skip(1) {  // Skip the header line
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() >= 4 {  // Typical format is: Node,Name,DriverName,Default
                    let name = parts[1].trim().to_string();
                    let driver = parts[2].trim().to_string();
                    let is_default = parts[3].trim().eq_ignore_ascii_case("TRUE");
                    
                    if !name.is_empty() {
                        printers.push(PrinterInfo {
                            name,
                            driver,
                            is_default,
                        });
                    }
                }
            }
        }
        _ => {
            // wmic falló o no está disponible
            return Err("WMIC not available or failed".into());
        }
    }
    
    Ok(printers)
} 