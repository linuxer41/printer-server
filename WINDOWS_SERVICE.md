# Configuración como servicio de Windows

Esta guía te ayudará a configurar el servidor de impresión como un servicio de Windows para que se inicie automáticamente con tu sistema.

## Lo que necesitas

- Windows 7 o más reciente
- Permisos de administrador
- El ejecutable compilado

## Pasos para la instalación

### 1. Compila el programa

Si aún no lo has hecho, compila el programa:

```powershell
cargo build --release
```

Encontrarás el ejecutable en `target/release/printer.exe`.

### 2. Instala el servicio

Abre PowerShell como administrador y ejecuta:

```powershell
sc.exe create "RustPrintService" binPath= "\"C:\ruta\a\printer.exe\" --run-as-service" start= auto DisplayName= "Servidor de Impresión"
```

> **Importante:** No olvides el espacio después de cada `=` en los comandos sc.exe. Es raro, pero así funciona.

Puedes añadir una descripción:

```powershell
sc.exe description "RustPrintService" "Servidor para imprimir PDFs desde URLs"
```

### 3. Dale permisos al servicio

Para que pueda acceder a tus impresoras:

```powershell
sc.exe privs "RustPrintService" SeLoadDriverPrivilege/SePrintPrivilege
```

### 4. Inicia el servicio

```powershell
sc.exe start "RustPrintService"
```

### 5. Comprueba que funciona

Verifica el estado del servicio:

```powershell
sc.exe query "RustPrintService"
```

También puedes probar si responde visitando:
http://localhost:8081/printers

## Gestión básica

Para detenerlo:
```powershell
sc.exe stop "RustPrintService"
```

Para reiniciarlo:
```powershell
sc.exe stop "RustPrintService"
sc.exe start "RustPrintService"
```

Para desinstalarlo:
```powershell
sc.exe stop "RustPrintService"
sc.exe delete "RustPrintService"
```

## Modos de inicio

Puedes cambiar cuándo se inicia:

```powershell
# Inicio automático con Windows
sc.exe config "RustPrintService" start= auto

# Inicio manual
sc.exe config "RustPrintService" start= demand

# Desactivado
sc.exe config "RustPrintService" start= disabled
```

## Solución de problemas

Si algo no funciona:

1. Revisa el Visor de eventos de Windows (ejecuta `eventvwr.msc`)
2. Busca en "Registros de Windows" > "Aplicación" eventos de "RustPrintService"

### Problemas típicos

* **El servicio no arranca**: Comprueba la ruta del ejecutable y los permisos
* **No puedes conectar al servidor**: Revisa si el puerto 8081 está bloqueado por el firewall
* **No detecta impresoras**: Asegúrate que el servicio corre con una cuenta con permisos de impresión

## Recursos adicionales

- [Documentación oficial de Microsoft sobre SC](https://docs.microsoft.com/en-us/windows-server/administration/windows-commands/sc-create)
- [Mejores prácticas para servicios de Windows](https://docs.microsoft.com/en-us/windows/win32/services/service-security-and-access-rights) 