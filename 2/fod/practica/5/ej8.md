# Ejercicio 8 - Hashing Extensible

**Enunciado:** Capacidad de 2 claves por bloque.
Claves:
1. + Buenos Aires: ....1001 (1)
2. + San Juan: ....0100 (0)
3. + Entre Ríos: ....1110 (0)
4. + Corrientes: ....0010 (0)
5. + San Luis: ....0101 (1)
6. + Tucumán: ....0111 (1)
7. + Río Negro: ....0011 (1)
8. + Jujuy: ....1111 (1)
9. + Salta: ....1010 (0)
10. - Río Negro: ....0011 (1)

## 1. Insertar Buenos Aires y San Juan
- `p = 0`. Ambos van al Bloque 0 (`p' = 0`).

| # Bloque | p' | R1 | R2 |
|---|---|---|---|
| **0** | 0 | Buenos Aires | San Juan |

## 2. Insertar Entre Ríos (1110)
- OVERFLOW en B0. `p = 1`.
- Evaluando el último bit:
  - San Juan (0), Entre Ríos (0) -> B0 (`p' = 1`)
  - Buenos Aires (1) -> B1 (`p' = 1`)

| Directorio (p=1) | # Bloque |
|---|---|
| 0 | 0 |
| 1 | 1 |

| # Bloque | p' | R1 | R2 |
|---|---|---|---|
| **0** | 1 | San Juan | Entre Ríos |
| **1** | 1 | Buenos Aires | |

## 3. Insertar Corrientes (0010)
- Sufijo 0, va a B0. OVERFLOW en B0. `p = 2`.
- Evaluando los 2 últimos bits:
  - San Juan (00) -> B2 (`p' = 2`)
  - Entre Ríos (10), Corrientes (10) -> B3 (`p' = 2`)
- B1 se mantiene con `p' = 1`.

| Directorio (p=2) | # Bloque |
|---|---|
| 00 | 2 |
| 01 | 1 |
| 10 | 3 |
| 11 | 1 |

| # Bloque | p' | R1 | R2 |
|---|---|---|---|
| **1** | 1 | Buenos Aires | |
| **2** | 2 | San Juan | |
| **3** | 2 | Entre Ríos | Corrientes |

## 4. Insertar San Luis (0101)
- Sufijo 01, va a B1. B1 tiene lugar.

| # Bloque | p' | R1 | R2 |
|---|---|---|---|
| **1** | 1 | Buenos Aires | San Luis |

## 5. Insertar Tucumán (0111)
- Sufijo 11, va a B1. OVERFLOW en B1.
- `p'` de B1 es 1, y `p` es 2. **NO duplicamos el directorio**. Sólo dividimos B1 y aumentamos su `p'` a 2.
- 2 últimos bits:
  - Buenos Aires (01), San Luis (01) -> B1 (`p' = 2`)
  - Tucumán (11) -> B4 (`p' = 2`)
- Actualizamos la entrada `11` del directorio para apuntar a B4.

| Directorio (p=2) | # Bloque |
|---|---|
| 00 | 2 |
| 01 | 1 |
| 10 | 3 |
| 11 | 4 |

| # Bloque | p' | R1 | R2 |
|---|---|---|---|
| **1** | 2 | Buenos Aires | San Luis |
| **4** | 2 | Tucumán | |

## 6. Insertar Río Negro (0011)
- Sufijo 11, va a B4. B4 tiene lugar.

| # Bloque | p' | R1 | R2 |
|---|---|---|---|
| **4** | 2 | Tucumán | Río Negro |

## 7. Insertar Jujuy (1111)
- Sufijo 11, va a B4. OVERFLOW en B4. `p'` de B4 es 2. `p` es 2.
- Duplicamos el directorio: `p = 3`.
- Evaluando 3 últimos bits para el split de B4:
  - Tucumán (011), Río Negro (011) -> B4 (`p' = 3`)
  - Jujuy (111) -> B5 (`p' = 3`)

| Directorio (p=3) | # Bloque |
|---|---|
| 000 | 2 |
| 001 | 1 |
| 010 | 3 |
| 011 | 4 |
| 100 | 2 |
| 101 | 1 |
| 110 | 3 |
| 111 | 5 |

| # Bloque | p' | R1 | R2 |
|---|---|---|---|
| **4** | 3 | Tucumán | Río Negro |
| **5** | 3 | Jujuy | |

## 8. Insertar Salta (1010)
- Sufijo 010, va a B3 (`[Entre Ríos, Corrientes]`). OVERFLOW en B3.
- `p'` de B3 es 2 < `p` (3). Sólo dividimos B3, no duplicamos directorio.
- 3 últimos bits:
  - Corrientes (010), Salta (010) -> B3 (`p' = 3`)
  - Entre Ríos (110) -> B6 (`p' = 3`)

| Directorio (p=3) | # Bloque |
|---|---|
| ... | |
| 010 | 3 |
| 110 | 6 |
| ... | |

| # Bloque | p' | R1 | R2 |
|---|---|---|---|
| **3** | 3 | Corrientes | Salta |
| **6** | 3 | Entre Ríos | |

## 9. Eliminar Río Negro (0011)
- Eliminamos de B4.

## Estado Final

| Directorio (p=3) | # Bloque |
|---|---|
| 000 | 2 |
| 001 | 1 |
| 010 | 3 |
| 011 | 4 |
| 100 | 2 |
| 101 | 1 |
| 110 | 6 |
| 111 | 5 |

| # Bloque | p' | R1 | R2 |
|---|---|---|---|
| **1** | 2 | Buenos Aires | San Luis |
| **2** | 2 | San Juan | |
| **3** | 3 | Corrientes | Salta |
| **4** | 3 | Tucumán | |
| **5** | 3 | Jujuy | |
| **6** | 3 | Entre Ríos | |
