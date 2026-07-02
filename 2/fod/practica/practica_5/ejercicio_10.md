# Ejercicio 10 - Hashing Extensible

**Enunciado:** Capacidad 2 registros por bloque.
Claves:
1. + Mansilla: 01100010 -> 0 (1 bit), 10 (2 bits), 010 (3 bits)
2. + Cetré: 10001000 -> 0 (1 bit), 00 (2 bits), 000 (3 bits)
3. + Ascacibar: 01010111 -> 1 (1 bit), 11 (2 bits), 111 (3 bits)
4. + Carrillo: 11110101 -> 1 (1 bit), 01 (2 bits), 101 (3 bits)
5. + Manyoma: 00110100 -> 0 (1 bit), 00 (2 bits), 100 (3 bits)
6. + Méndez: 00101001 -> 1 (1 bit), 01 (2 bits), 001 (3 bits)
7. + Alario: 11000101 -> 1 (1 bit), 01 (2 bits), 101 (3 bits)
8. - Mansilla

## 1. Insertar Mansilla y Cetré
- Ambos van a B0 (`p = 0`, `p' = 0`).

| # Bloque | p' | R1 | R2 |
|---|---|---|---|
| **0** | 0 | Mansilla | Cetré |

## 2. Insertar Ascacibar
- OVERFLOW en B0. `p = 1`.
- Último bit: Mansilla (0) y Cetré (0) quedan en B0 (`p' = 1`). Ascacibar (1) va a B1 (`p' = 1`).

| Directorio (p=1) | # Bloque |
|---|---|
| 0 | 0 |
| 1 | 1 |

| # Bloque | p' | R1 | R2 |
|---|---|---|---|
| **0** | 1 | Mansilla | Cetré |
| **1** | 1 | Ascacibar | |

## 3. Insertar Carrillo
- Sufijo 1, va a B1. B1 tiene lugar.

| # Bloque | p' | R1 | R2 |
|---|---|---|---|
| **1** | 1 | Ascacibar | Carrillo |

## 4. Insertar Manyoma
- Sufijo 0, va a B0. OVERFLOW en B0 (`[Mansilla, Cetré]`).
- `p` sube a 2.
- Últimos 2 bits: Cetré (00) y Manyoma (00) -> B0 (`p' = 2`). Mansilla (10) -> nuevo B2 (`p' = 2`).

| Directorio (p=2) | # Bloque |
|---|---|
| 00 | 0 |
| 01 | 1 |
| 10 | 2 |
| 11 | 1 |

| # Bloque | p' | R1 | R2 |
|---|---|---|---|
| **0** | 2 | Cetré | Manyoma |
| **1** | 1 | Ascacibar | Carrillo |
| **2** | 2 | Mansilla | |

## 5. Insertar Méndez
- Sufijo 01, va a B1. OVERFLOW en B1 (`[Ascacibar, Carrillo]`).
- `p' = 1` y `p = 2`. No se duplica el directorio, se divide B1 y su `p'` sube a 2.
- Últimos 2 bits: Carrillo (01) y Méndez (01) -> B1 (`p' = 2`). Ascacibar (11) -> B3 (`p' = 2`).

| Directorio (p=2) | # Bloque |
|---|---|
| 00 | 0 |
| 01 | 1 |
| 10 | 2 |
| 11 | 3 |

| # Bloque | p' | R1 | R2 |
|---|---|---|---|
| **0** | 2 | Cetré | Manyoma |
| **1** | 2 | Carrillo | Méndez |
| **2** | 2 | Mansilla | |
| **3** | 2 | Ascacibar | |

## 6. Insertar Alario
- Sufijo 01, va a B1. OVERFLOW en B1 (`[Carrillo, Méndez]`).
- `p'` de B1 es 2, y `p` es 2. Se duplica el directorio (`p` sube a 3).
- Últimos 3 bits: Méndez (001) -> B1 (`p' = 3`). Carrillo (101) y Alario (101) -> B4 (`p' = 3`).

| Directorio (p=3) | # Bloque |
|---|---|
| 000 | 0 |
| 001 | 1 |
| 010 | 2 |
| 011 | 3 |
| 100 | 0 |
| 101 | 4 |
| 110 | 2 |
| 111 | 3 |

| # Bloque | p' | R1 | R2 |
|---|---|---|---|
| **0** | 2 | Cetré | Manyoma |
| **1** | 3 | Méndez | |
| **2** | 2 | Mansilla | |
| **3** | 2 | Ascacibar | |
| **4** | 3 | Carrillo | Alario |

## 7. Eliminar Mansilla
- Se elimina de B2. B2 queda vacío.

## Estado Final

| Directorio (p=3) | # Bloque |
|---|---|
| 000 | 0 |
| 001 | 1 |
| 010 | 2 |
| 011 | 3 |
| 100 | 0 |
| 101 | 4 |
| 110 | 2 |
| 111 | 3 |

| # Bloque | p' | R1 | R2 |
|---|---|---|---|
| **0** | 2 | Cetré | Manyoma |
| **1** | 3 | Méndez | |
| **2** | 2 | | |
| **3** | 2 | Ascacibar | |
| **4** | 3 | Carrillo | Alario |
