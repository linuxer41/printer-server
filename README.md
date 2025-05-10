# Servidor de Impresión

Un servidor HTTP simple que te permite imprimir PDFs directamente desde URLs y gestionar tus impresoras. Compatible con Windows y Linux.

## ¿Qué hace?

- Muestra todas tus impresoras conectadas
- Imprime PDFs desde cualquier URL
- Funciona tanto en Windows como en Linux
- Se puede configurar como servicio en Windows

## Instalación rápida

Necesitas tener Rust instalado. Si no lo tienes, descárgalo desde [rust-lang.org](https://www.rust-lang.org/).

Para compilar:

```bash
cargo build --release
```

El ejecutable estará en `target/release/printer`.

## Dependencias

Para Windows, recomendamos instalar [SumatraPDF](https://www.sumatrapdfreader.org/download-free-pdf-viewer) para la impresión de PDFs. Puedes colocar SumatraPDF.exe en:
- El mismo directorio que tu aplicación
- En cualquiera de estos subdirectorios: bin, tools, apps, utils
- En su ubicación de instalación predeterminada

## Tecnología

Este servidor utiliza:
- La biblioteca [printers](https://crates.io/crates/printers) para detectar impresoras
- SumatraPDF para imprimir PDFs en Windows
- CUPS para imprimir en Linux

## Cómo usarlo

Para iniciar el servidor:

```bash
./target/release/printer
```

Por defecto, el servidor se inicia en el puerto 8081.

## API sencilla

### Ver tus impresoras

```
GET http://localhost:8081/printers
```

Te mostrará algo como:

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

### Imprimir un PDF

Para imprimir un PDF desde una URL:

```
GET http://localhost:8081/print?url=https://ejemplo.com/documento.pdf
```

Esto usará tu impresora predeterminada. Si quieres usar una específica:

```
GET http://localhost:8081/print?url=https://ejemplo.com/documento.pdf&printer=HP LaserJet
```

## Configurarlo como servicio en Windows

1. Compila el proyecto
2. Instala el servicio:

```powershell
sc.exe create "RustPrintService" binPath= "\"C:\ruta\a\printer.exe\" --run-as-service" start= auto
```

3. Inicia el servicio:

```powershell
sc.exe start "RustPrintService"
```

Para más detalles, consulta `WINDOWS_SERVICE.md`.

## Configuración en Linux

Para configurarlo como servicio en Linux usando systemd:

```bash
sudo nano /etc/systemd/system/printer-service.service
```

Con este contenido:

```ini
[Unit]
Description=Servidor de impresión
After=network.target

[Service]
ExecStart=/ruta/a/printer
Restart=always
User=tuusuario

[Install]
WantedBy=multi-user.target
```

Luego:

```bash
sudo systemctl daemon-reload
sudo systemctl enable printer-service
sudo systemctl start printer-service
```

## Tecnologías

Este proyecto utiliza:
- axum para el servidor web
- tokio para el manejo asíncrono
- reqwest para descargar PDFs
- printers para detección de impresoras
- SumatraPDF para impresión de PDFs en Windows

## Licencia

MIT - Úsalo como quieras. 