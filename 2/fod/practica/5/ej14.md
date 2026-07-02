# Ejercicio 14 - Hashing Extensible (Inserción con Desborde)

**Enunciado:** Se tiene un archivo organizado mediante hashing extensible (`p = 2`). ¿Qué sucede al agregar la clave Verde (00111100)?

## Análisis del Estado Previo

**Tabla de dispersión (Bits: 2)**
- `00 -> B0`
- `01 -> B1`
- `10 -> B2`
- `11 -> B1`

**Archivo de datos**
- B0 (`p' = 2`): `[Blanco (11001100), Rojo (00001100)]`
- B1 (`p' = 1`): `[Rosa (000000011), ]`
- B2 (`p' = 2`): `[Azul (10101010), ]`

## Inserción de la clave "Verde"
La clave a insertar es: Verde (`00111100`).
1. Extraemos el sufijo de acuerdo a la profundidad global actual `p = 2`.
2. Los últimos 2 bits de la clave son **`00`**.
3. Consultamos el directorio: la entrada `00` apunta al **Bloque 0**.
4. Intentamos insertar Verde en el **Bloque 0**.
5. Revisamos el Bloque 0: contiene a Blanco y Rojo. Está lleno (capacidad de 2 claves). Se produce un **OVERFLOW (desborde)** en el Bloque 0.

## Resolución del Desborde
Cuando un bloque se desborda, debemos comparar su profundidad local (`p'`) con la profundidad global (`p`):
- Para el Bloque 0: `p' = 2`.
- Profundidad global: `p = 2`.
- Dado que `p' == p`, **no basta con dividir únicamente el bloque**. Para poder agregar un bit extra que discrimine a los registros de este bloque (pasando de usar 2 bits a 3 bits), es mandatorio **duplicar la tabla de dispersión** (pasando `p` a 3).
- Al duplicar la tabla, el Bloque 0 se dividirá en dos: el mismo Bloque 0 y un nuevo **Bloque 3**. Ambos pasarán a tener `p' = 3`.
- Los registros del bloque original y el nuevo registro (Verde) se distribuirán entre el Bloque 0 y el nuevo Bloque 3 según sus 3 últimos bits.

## Opción Correcta
Analizando las opciones dadas:
- **a.** Falso (sí produce desborde, debe ir al bloque 0, no al 2).
- **b.** Falso (no existe el "desborde hacia bloques hermanos con espacio" en hashing extensible; no se usa saturación progresiva).
- **c.** Falso (como `p' = p`, ES OBLIGATORIO duplicar la tabla, no se puede simplemente dividir el bloque).
- **d.** **Correcto.** Se produce desborde en el B0, se crea el B3 y al ser `p' == p`, se duplica la cantidad de direcciones de la tabla (p pasa de 2 a 3).

**Respuesta Correcta:** **Opción D**.
