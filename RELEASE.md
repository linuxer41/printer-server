# Crear y publicar nuevas versiones

Este proyecto utiliza GitHub Actions para automatizar la compilación y publicación de releases para Windows y Linux.

## Cómo publicar una nueva versión

1. Asegúrese de que todos los cambios estén confirmados y subidos al repositorio.

2. Etiquete la nueva versión utilizando git. Siga el formato de versionado semántico (`vX.Y.Z`).

   ```bash
   # Etiquete la versión actual
   git tag -a v1.0.0 -m "Versión 1.0.0"
   
   # Suba la etiqueta al repositorio
   git push origin v1.0.0
   ```

3. Una vez que haya subido la etiqueta, GitHub Actions ejecutará automáticamente el flujo de trabajo de release.

4. El flujo de trabajo:
   - Compilará la aplicación para Windows y Linux
   - Creará archivos zip/tarball con los ejecutables y la documentación
   - Publicará una nueva release en GitHub con estos archivos adjuntos

5. Cuando el flujo de trabajo termine, podrá encontrar la nueva release en la sección "Releases" de su repositorio de GitHub.

## Distribución de binarios

Los binarios generados se distribuyen de la siguiente manera:

### Windows

Un archivo ZIP (`printer-windows-x86_64.zip`) que contiene:
- El ejecutable (`printer-windows-x86_64.exe`)
- El archivo README.md
- El archivo WINDOWS_SERVICE.md con instrucciones detalladas para la instalación del servicio

### Linux

Un archivo tarball (`printer-linux-x86_64.tar.gz`) que contiene:
- El ejecutable (`printer-linux-x86_64`) con permisos de ejecución
- El archivo README.md

## Problemas comunes

### Error al crear la release

Si hay problemas al crear la release, verifique:

1. Que tiene los permisos adecuados en el repositorio para ejecutar Actions
2. Que la etiqueta sigue el formato correcto (`v` seguido de números y puntos)
3. Que el workflow tiene los permisos necesarios para crear releases (ajuste la configuración de Actions en la configuración del repositorio)

### Error en la compilación

Si hay errores en la compilación:

1. Asegúrese de que el código compile localmente para ambas plataformas
2. Verifique los registros de GitHub Actions para identificar errores específicos

## Versionado semántico

Se recomienda seguir las convenciones de versionado semántico:

- **MAJOR (X)**: Cambios incompatibles con versiones anteriores
- **MINOR (Y)**: Nuevas funcionalidades compatibles
- **PATCH (Z)**: Correcciones de errores compatibles

Para más información visite [semver.org](https://semver.org/) 