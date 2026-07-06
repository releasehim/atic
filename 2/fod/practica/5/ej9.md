# Ejercicio 9 - Hashing Extensible

**Enunciado:** Capacidad 2 registros por bloque.
Claves y últimos bits (sufijos):

1. + Guillermo.B: 01100011 -> 11 (2 bits), 011 (3 bits)
2. + Gomez: 00000001 -> 01 (2 bits), 001 (3 bits)
3. + Gustavo.B: 01010110 -> 10 (2 bits), 110 (3 bits)
4. + Sosa: 11110100 -> 00 (2 bits), 100 (3 bits)
5. + Enria: 00110101 -> 01 (2 bits), 101 (3 bits)
6. + Guli: 00101000 -> 00 (2 bits), 000 (3 bits)
7. + Gustavo.B: 01010110
8. + Sosa: 11110100

## 1. Insertar Guillermo.B y Gomez
+ Ambos van a B0 (`p = 0`, `p' = 0`).

| # Bloque | p' | Registro 1 | Registro 2 |
|---|---|---|---|
| **0** | 0 | Guillermo.B | Gomez |

## 2. Insertar Gustavo.B (110)
+ OVERFLOW en B0. `p = 1`.
+ Último bit: Gustavo.B (0) -> B0 (`p' = 1`). Guillermo.B (1), Gomez (1) -> B1 (`p' = 1`).

| Directorio (p=1) | # Bloque |
| --- | --- |
| 0 | 0 |
| 1 | 1 |

| # Bloque | p' | R1 | R2 |
| --- | --- | --- | --- |
| **0** | 1 | Gustavo.B | |
| **1** | 1 | Guillermo.B | Gomez |

## 3. Insertar Sosa (100)
+ Sufijo 0, va a B0. B0 tiene lugar.

| # Bloque | p' | R1 | R2 |
|---|---|---|---|
| **0** | 1 | Gustavo.B | Sosa |

## 4. Insertar Enria (101)
+ Sufijo 1, va a B1. OVERFLOW en B1 (`[Guillermo.B, Gomez]`).
+ `p` sube a 2.
+ Últimos 2 bits: Guillermo.B (11) -> B1 (`p' = 2`). Gomez (01), Enria (01) -> B2 (`p' = 2`).
+ B0 (`[Gustavo.B, Sosa]`) mantiene `p' = 1` y es apuntado por las entradas `00` y `10`.

| Directorio (p=2) | # Bloque |
| --- | --- |
| 00 | 0 |
| 01 | 2 |
| 10 | 0 |
| 11 | 1 |

| # Bloque | p' | R1 | R2 |
| --- | --- | --- | --- |
| **0** | 1 | Gustavo.B | Sosa |
| **1** | 2 | Guillermo.B | |
| **2** | 2 | Gomez | Enria |

## 5. Insertar Guli (000)
+ Sufijo 00, va a B0. OVERFLOW en B0 (`[Gustavo.B, Sosa]`).
+ `p' = 1` para B0, mientras que `p = 2`. **No se duplica el directorio**.
+ Se divide B0 aumentando su `p'` a 2.
+ Últimos 2 bits: Sosa (00), Guli (00) -> B0 (`p' = 2`). Gustavo.B (10) -> B3 (`p' = 2`).

| Directorio (p=2) | # Bloque |
| --- | --- |
| 00 | 0 |
| 01 | 2 |
| 10 | 3 |
| 11 | 1 |

| # Bloque | p' | R1 | R2 |
| --- | --- | --- | --- |
| **0** | 2 | Sosa | Guli |
| **1** | 2 | Guillermo.B | |
| **2** | 2 | Gomez | Enria |
| **3** | 2 | Gustavo.B | |

## 6. Eliminar Gustavo.B y Sosa
+ Gustavo.B (10) se elimina de B3. B3 queda vacío.
+ Sosa (00) se elimina de B0. B0 queda con Guli.

## Estado Final

| Directorio (p=2) | # Bloque |
| --- | --- |
| 00 | 0 |
| 01 | 2 |
| 10 | 3 |
| 11 | 1 |

| # Bloque | p' | R1 | R2 |
| --- | --- | --- | --- |
| **0** | 2 | Guli | |
| **1** | 2 | Guillermo.B | |
| **2** | 2 | Gomez | Enria |
| **3** | 2 | | |
