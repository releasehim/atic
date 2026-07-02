# Ejercicio 13 - Hashing Extensible (Estado Inicial)

**Enunciado:** Indique cuál de las siguientes opciones representa un estado inicial válido en un archivo organizado mediante la técnica de hashing extensible. Justifique su respuesta.

## Análisis de las Opciones

Un **estado inicial** en Hashing Extensible es el estado de la estructura de datos justo después de haber sido creada, **antes** de que se inserte cualquier registro.
Por definición, el estado inicial se caracteriza por:
1. **Directorio:** Tiene tamaño mínimo, es decir, `p = 0` (profundidad global = 0). Esto significa que el directorio tiene `2^0 = 1` única entrada. Al no usar bits para discriminar (0 bits), cualquier clave mapea a esa única entrada (representada a menudo con el sufijo `-`).
2. **Archivo de Datos:** Se inicializa con un único bloque vacío.
3. **Profundidad local del bloque:** El bloque inicial debe tener una profundidad local `Bits = 0` (`p' = 0`), coincidiendo con la profundidad global.

Veamos las opciones:
- **Opción A:** Tiene `Bits = 1` (p=1) para la tabla. Esto implica que ya ocurrió un split que duplicó el directorio, lo cual es imposible sin haber insertado registros (y el bloque figura vacío). Es inválido.
- **Opción B:** Tiene `p = 0`, `p' = 0` y 1 bloque. Sin embargo, el bloque **ya contiene un registro** ("Boca"). Este es un estado válido, pero representa el estado **después de la primera inserción**, no el estado "inicial" (vacío).
- **Opción D:** Tiene `Bits = 1` (p=1) y 2 bloques vacíos con `p'=1`. El hashing extensible no inicializa múltiples bloques vacíos, los crea bajo demanda. Es inválido.

### Opción Correcta: Opción C
**Justificación:**
La **Opción C** representa perfectamente las condiciones teóricas del estado inicial:
- La tabla de dispersión tiene `Bits: 0` (`p=0`) con una única entrada (`-`) que apunta al Bloque 0.
- El Archivo de datos tiene un único bloque (Bloque 0) con `Bits: 0` (`p'=0`), y se encuentra completamente **vacío**.

Esta configuración asegura que las primeras inserciones vayan directamente al bloque 0, y el crecimiento dinámico (splits y duplicación de directorio) comience desde lo más básico a medida que ocurran desbordes.
