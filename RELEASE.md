# Publicar nuevas versiones

Este proyecto usa GitHub Actions para automatizar la compilación y publicación de versiones para Windows y Linux.

## Pasos para lanzar una versión

1. Asegúrate de haber subido todos tus cambios a GitHub.

2. Crea un tag con la versión. Usamos formato semántico (vX.Y.Z):

   ```bash
   # Crea el tag
   git tag -a v1.0.0 -m "Versión 1.0.0"
   
   # Sube el tag a GitHub
   git push origin v1.0.0
   ```

3. ¡Eso es todo! GitHub Actions se encargará automáticamente del resto:
   - Compilará el programa para Windows y Linux
   - Empaquetará todo (ejecutables y documentación)
   - Publicará la release en GitHub

4. Podrás ver tu nueva versión en la pestaña "Releases" de tu repositorio.

## Archivos generados

### Para Windows

Un ZIP (`printer-windows-x86_64.zip`) con:
- El ejecutable para Windows
- Documentación básica
- Guía de instalación como servicio

### Para Linux

Un tarball (`printer-linux-x86_64.tar.gz`) con:
- El ejecutable para Linux (con permisos de ejecución)
- Documentación básica

## Solución de problemas

### Si la release no se crea

Revisa:
- Que tienes permisos de escritura en el repo
- Que el tag sigue el formato correcto (v1.2.3)
- Que GitHub Actions tiene permisos para crear releases

### Si la compilación falla

- Asegúrate que el código compila en tu máquina
- Revisa los logs de GitHub Actions para ver el error específico

## Sobre el versionado

Seguimos estas reglas:

- **X (MAJOR)**: Cambios que rompen compatibilidad
- **Y (MINOR)**: Nuevas funciones compatibles
- **Z (PATCH)**: Corrección de bugs

Más info en [semver.org](https://semver.org/) 