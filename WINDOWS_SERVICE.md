# Guía detallada para configurar como servicio en Windows

## Introducción

Este documento explica cómo configurar la aplicación para que se ejecute como un servicio de Windows, lo que permite que se inicie automáticamente con el sistema y se ejecute en segundo plano.

## Requisitos previos

- Windows 7 o superior
- Derechos de administrador para instalar servicios
- La aplicación compilada en modo release

## Paso 1: Compilar la aplicación

Primero, compile la aplicación en modo release:

```powershell
cargo build --release
```

Esto generará el ejecutable en `target/release/printer.exe`.

## Paso 2: Instalar el servicio

Para instalar el servicio, necesitamos usar el comando `sc.exe` que viene con Windows. Abra una terminal de PowerShell con derechos de administrador y ejecute:

```powershell
sc.exe create "RustPrintService" binPath= "\"C:\ruta\completa\a\printer.exe\" --run-as-service" start= auto DisplayName= "Rust Print Service"
```

Asegúrese de reemplazar `C:\ruta\completa\a\printer.exe` con la ruta real a su ejecutable.

> **Nota importante:** El espacio después de `=` en los parámetros de `sc.exe` es necesario.

Puede añadir una descripción al servicio:

```powershell
sc.exe description "RustPrintService" "Servicio para imprimir documentos PDF a través de una API REST"
```

## Paso 3: Configurar permisos

Es importante asegurarse de que el servicio tenga permisos para acceder a las impresoras:

```powershell
sc.exe privs "RustPrintService" SeLoadDriverPrivilege/SePrintPrivilege
```

## Paso 4: Iniciar el servicio

Ahora puede iniciar el servicio:

```powershell
sc.exe start "RustPrintService"
```

## Paso 5: Verificar que el servicio esté funcionando

Puede verificar que el servicio esté funcionando correctamente:

```powershell
sc.exe query "RustPrintService"
```

También puede comprobar que el servidor web está funcionando intentando acceder a:

```
http://localhost:8081/printers
```

## Gestión del servicio

### Detener el servicio

```powershell
sc.exe stop "RustPrintService"
```

### Reiniciar el servicio

```powershell
sc.exe stop "RustPrintService"
sc.exe start "RustPrintService"
```

### Eliminar el servicio

Si necesita desinstalar el servicio:

```powershell
sc.exe stop "RustPrintService"
sc.exe delete "RustPrintService"
```

### Cambiar el modo de inicio

Para cambiar el modo de inicio (automático, manual, deshabilitado):

```powershell
# Automático
sc.exe config "RustPrintService" start= auto

# Manual
sc.exe config "RustPrintService" start= demand

# Deshabilitado
sc.exe config "RustPrintService" start= disabled
```

## Solución de problemas

Si tiene problemas con el servicio, puede consultar los registros de eventos de Windows:

1. Abra el "Visor de eventos" (`eventvwr.msc`)
2. Navegue a "Registros de Windows" > "Aplicación"
3. Busque eventos relacionados con "RustPrintService"

### Problemas comunes

1. **El servicio no se inicia**
   - Verifique que la ruta al ejecutable sea correcta
   - Asegúrese de que el usuario del servicio tenga permisos suficientes
   - Compruebe los registros de eventos para ver mensajes de error específicos

2. **No se puede acceder al servidor web**
   - Verifique que el puerto 8081 no esté bloqueado por el firewall
   - Asegúrese de que otro servicio no esté usando el mismo puerto

3. **Problemas de permisos de impresora**
   - Asegúrese de que el servicio se ejecute con una cuenta que tenga permisos para acceder a las impresoras

## Recursos adicionales

- [Documentación oficial de Microsoft sobre SC](https://docs.microsoft.com/en-us/windows-server/administration/windows-commands/sc-create)
- [Mejores prácticas para servicios de Windows](https://docs.microsoft.com/en-us/windows/win32/services/service-security-and-access-rights) 