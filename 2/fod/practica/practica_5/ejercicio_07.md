# Ejercicio 7 - Hashing Extensible

**Enunciado:** Para las siguientes claves, realizar el proceso de dispersión mediante hashing extensible sabiendo que cada bloque/nodo tiene capacidad para 2 registros. Se utilizan los últimos bits del código binario (sufijo).

**Claves:**
1. + Darin (00111111) -> Últimos bits: ...11
2. + Alterio (11110100) -> Últimos bits: ...00
3. + Sbaraglia (10100101) -> Últimos bits: ...01
4. + De la Serna (01010111) -> Últimos bits: ...11
5. + Altavista (01101011) -> Últimos bits: ...11
6. + Grandinetti (10101010) -> Últimos bits: ...10
7. - Altavista (01101011)
8. - Sbaraglia (10100101)

## Estado Inicial
- Directorio de profundidad global `p = 0`.
- 1 Bloque inicial `B0` con profundidad local `p' = 0`.
- Capacidad: 2 registros por bloque.

## 1. Insertar Darin
- `Darin` va al Bloque 0.

| Directorio (p=0) | # Bloque |
|---|---|
| - | 0 |

| # Bloque | p' | Registro 1 | Registro 2 |
|---|---|---|---|
| **0** | 0 | Darin | |

## 2. Insertar Alterio
- `Alterio` va al Bloque 0.

| # Bloque | p' | Registro 1 | Registro 2 |
|---|---|---|---|
| **0** | 0 | Darin | Alterio |

## 3. Insertar Sbaraglia
- `Sbaraglia` va al Bloque 0. El bloque está lleno -> **OVERFLOW**.
- Como `p' = p = 0`, duplicamos el directorio incrementando `p` a 1.
- Evaluamos el último bit (1 bit):
  - Darin (...1) -> B1
  - Alterio (...0) -> B0
  - Sbaraglia (...1) -> B1
- Dividimos `B0` en `B0` y nuevo `B1`. Ambos quedan con `p' = 1`.

| Directorio (p=1) | # Bloque |
|---|---|
| 0 | 0 |
| 1 | 1 |

| # Bloque | p' | Registro 1 | Registro 2 |
|---|---|---|---|
| **0** | 1 | Alterio | |
| **1** | 1 | Darin | Sbaraglia |

## 4. Insertar De la Serna
- `De la Serna` termina en 1, va al Bloque 1.
- Bloque 1 está lleno -> **OVERFLOW**.
- Como `p' = p = 1`, duplicamos el directorio (`p` pasa a 2).
- Dividimos el `B1` (que causó el overflow) considerando los 2 últimos bits:
  - Darin (...11) -> nuevo B2
  - Sbaraglia (...01) -> queda en B1
  - De la Serna (...11) -> nuevo B2
- El bloque B0 (`Alterio` ...00) sigue con `p' = 1`, por lo que apuntarán a él tanto la entrada `00` como la `10`.
*(Nota de nomenclatura: renombraremos el bloque de Sbaraglia como B3 y el nuevo de Darin como B1 para claridad)*.
- B1 (p'=2, sufijo 11): `[Darin, De la Serna]`
- B3 (p'=2, sufijo 01): `[Sbaraglia]`

| Directorio (p=2) | # Bloque |
|---|---|
| 00 | 0 |
| 01 | 3 |
| 10 | 0 |
| 11 | 1 |

| # Bloque | p' | Registro 1 | Registro 2 |
|---|---|---|---|
| **0** | 1 | Alterio | |
| **1** | 2 | Darin | De la Serna |
| **3** | 2 | Sbaraglia | |

## 5. Insertar Altavista
- `Altavista` termina en ...11, va al Bloque 1.
- Bloque 1 está lleno (`[Darin, De la Serna]`) -> **OVERFLOW**.
- Como `p' = p = 2`, duplicamos el directorio (`p` pasa a 3).
- Dividimos `B1` considerando los 3 últimos bits:
  - Darin (...111) -> B1
  - De la Serna (...111) -> B1
  - Altavista (...011) -> nuevo B4
- B1 queda con `p' = 3`. B4 se crea con `p' = 3`.

| Directorio (p=3) | # Bloque |
|---|---|
| 000 | 0 |
| 001 | 3 |
| 010 | 0 |
| 011 | 4 |
| 100 | 0 |
| 101 | 3 |
| 110 | 0 |
| 111 | 1 |

| # Bloque | p' | Registro 1 | Registro 2 |
|---|---|---|---|
| **0** | 1 | Alterio | |
| **1** | 3 | Darin | De la Serna |
| **3** | 2 | Sbaraglia | |
| **4** | 3 | Altavista | |

## 6. Insertar Grandinetti
- `Grandinetti` termina en ...010, va a la entrada `010` que apunta al Bloque 0.
- Bloque 0 tiene lugar (`p' = 1`). Insertamos sin problema.

| # Bloque | p' | Registro 1 | Registro 2 |
|---|---|---|---|
| **0** | 1 | Alterio | Grandinetti |
| ... | | | |

## 7. Eliminar Altavista
- `Altavista` termina en ...011, vamos al Bloque 4 y lo eliminamos.
- El bloque 4 queda vacío.

| # Bloque | p' | Registro 1 | Registro 2 |
|---|---|---|---|
| **4** | 3 | | |

## 8. Eliminar Sbaraglia
- `Sbaraglia` termina en ...101, vamos al Bloque 3 y lo eliminamos.
- El bloque 3 queda vacío.

## Estado Final

| Directorio (p=3) | # Bloque |
|---|---|
| 000 | 0 |
| 001 | 3 |
| 010 | 0 |
| 011 | 4 |
| 100 | 0 |
| 101 | 3 |
| 110 | 0 |
| 111 | 1 |

| # Bloque | p' | Registro 1 | Registro 2 |
|---|---|---|---|
| **0** | 1 | Alterio | Grandinetti |
| **1** | 3 | Darin | De la Serna |
| **3** | 2 | | |
| **4** | 3 | | |
