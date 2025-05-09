# Servidor de Impresión

Este proyecto es un servidor HTTP que permite listar impresoras y enviar archivos PDF para impresión.

## Características

- Lista todas las impresoras disponibles en el sistema
- Imprime archivos PDF desde una URL
- Funciona en Windows y Linux
- Puede ejecutarse como un servicio de Windows

## Instalación

### Requisitos previos

- Rust (última versión estable)
- Cargo (viene con Rust)

### Compilación

```bash
cargo build --release
```

El ejecutable se creará en `target/release/printer`.

## Uso

### Iniciar el servidor en modo consola

```bash
cargo run
```

o usando el ejecutable:

```bash
./target/release/printer
```

### Endpoints del API

#### Listar impresoras

```
GET /printers
```

Ejemplo de respuesta:

```json
[
  {
    "name": "HP LaserJet",
    "driver": "HPLJ4250.dll",
    "is_default": true
  },
  {
    "name": "Canon PIXMA",
    "driver": "CNPXM.dll",
    "is_default": false
  }
]
```

#### Imprimir un PDF

```
GET /print?url=URL_DEL_PDF&printer=NOMBRE_DE_IMPRESORA
```

Parámetros:
- `url`: URL del archivo PDF a imprimir (requerido)
- `printer`: Nombre de la impresora (opcional). Si no se proporciona, se usará la impresora predeterminada del sistema. Si no hay impresora predeterminada, se devolverá un error.

Ejemplos:

Con impresora específica:
```
GET /print?url=https://example.com/documento.pdf&printer=HP%20LaserJet
```

Usando la impresora predeterminada:
```
GET /print?url=https://example.com/documento.pdf
```

Respuesta exitosa:

```json
{
  "status": "success",
  "printer": "HP LaserJet"
}
```

Respuesta de error:

```json
{
  "error": "Error downloading PDF: connection refused"
}
```

o

```json
{
  "error": "No se encontró ninguna impresora predeterminada"
}
```

## Configuración como servicio de Windows

### Instalación del servicio

1. Compila el proyecto en modo release:

```bash
cargo build --release
```

2. Instala el servicio usando el comando sc:

```powershell
sc.exe create "RustPrintService" binPath= "\"C:\ruta\completa\a\printer.exe\" --run-as-service" start= auto
```

3. Inicia el servicio:

```powershell
sc.exe start "RustPrintService"
```

### Gestión del servicio

- Para detener el servicio:

```powershell
sc.exe stop "RustPrintService"
```

- Para eliminar el servicio:

```powershell
sc.exe delete "RustPrintService"
```

## Configuración como servicio en Linux (systemd)

1. Crea un archivo de servicio:

```bash
sudo nano /etc/systemd/system/printer-service.service
```

2. Añade el siguiente contenido:

```ini
[Unit]
Description=Rust Print Server
After=network.target

[Service]
ExecStart=/ruta/completa/a/printer
WorkingDirectory=/ruta/a/directorio
Restart=always
User=usuario
Group=grupo

[Install]
WantedBy=multi-user.target
```

3. Recarga los servicios de systemd:

```bash
sudo systemctl daemon-reload
```

4. Habilita e inicia el servicio:

```bash
sudo systemctl enable printer-service
sudo systemctl start printer-service
```

## Desarrollo

Este proyecto utiliza las siguientes dependencias:
- axum: Framework web
- tokio: Runtime asíncrono
- reqwest: Cliente HTTP
- serde: Serialización/deserialización JSON
- tempfile: Manejo de archivos temporales
- windows-service: Soporte para servicios de Windows

## Licencia

Este proyecto está licenciado bajo la licencia MIT. 