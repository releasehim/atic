# Práctica 4 - Parte 3 (Preguntas de Opción Múltiple)

## Pregunta 1
**Enunciado:** Dado un Árbol B de orden 5 vacío, se insertan las claves 12, 7, 25, 30, 3, 15, 18 y 20 en ese orden. Indique cuál de las siguientes afirmaciones sobre el estado final del árbol es correcta.

**Análisis paso a paso:**
- Orden 5: máximo 4 claves, mínimo 2 claves.
- `+12`: `[12]`
- `+7`: `[7, 12]`
- `+25`: `[7, 12, 25]`
- `+30`: `[7, 12, 25, 30]` (Lleno)
- `+3`: `[3, 7, 12, 25, 30]` -> **OVERFLOW**. Se divide en: hijo izq `[3, 7]`, clave promovida `12`, hijo der `[25, 30]`. Raíz pasa a ser `[12]`.
- `+15`: Va al hijo derecho -> `[15, 25, 30]`
- `+18`: Va al hijo derecho -> `[15, 18, 25, 30]` (Lleno)
- `+20`: Va al hijo derecho -> `[15, 18, 20, 25, 30]` -> **OVERFLOW**. Se divide en: hijo izq `[15, 18]`, clave promovida `20`, hijo der `[25, 30]`. El 20 sube a la raíz. Raíz pasa a ser `[12, 20]`.

**Respuesta Correcta:** **B) La raíz tiene 2 claves y 3 hijos.**
(La raíz tiene las claves 12 y 20, y apunta a los nodos `[3, 7]`, `[15, 18]` y `[25, 30]`).

---

## Pregunta 2
**Enunciado:** En un Árbol B, ¿cuál de las siguientes operaciones puede provocar una redistribución de claves entre nodos?

**Análisis:**
- La inserción con desborde (overflow) genera una **división** (split) del nodo, promoviendo una clave al padre. Nunca genera redistribución.
- La búsqueda es una operación de lectura, no altera la estructura.
- La división de la raíz aumenta la altura del árbol, pero no redistribuye claves.
- El **borrado** puede causar que un nodo quede con menos claves del mínimo permitido (underflow). En ese caso, la primera acción para solucionarlo es intentar una **redistribución** con un nodo hermano adyacente.

**Respuesta Correcta:** **B) Borrado.**

---

## Pregunta 3
**Enunciado:** Considere un Árbol B de orden 5 con un único nodo raíz que contiene las claves `[10, 20, 30, 40, 50]`. ¿Qué sucede al insertar la clave 25?

**Análisis:**
- En un Árbol B de orden 5, el máximo de claves permitidas por nodo es 4.
- El enunciado indica que el nodo inicial tiene `[10, 20, 30, 40, 50]`, lo cual significa que **ya** se encuentra en estado de overflow (5 claves).
- Asumiendo que el árbol recién supera el máximo al insertar 50 o el 25 en realidad causa el desborde (el estado dado es hipotético para forzar el desborde previo), el nodo se debe dividir inmediatamente.
- Al dividir `[10, 20, 30, 40, 50]`, se promueve la clave del medio (30).
- El árbol queda con una raíz `[30]` y dos hojas: `[10, 20]` y `[40, 50]`.
- Al insertar el `25`, este baja por el árbol (25 < 30) y se inserta en la hoja izquierda, que ahora queda `[10, 20, 25]`.

**Respuesta Correcta:** **A) La raíz se divide en dos nodos hijos.** (Este es el efecto visible del desborde que ya presentaba el nodo, generando la estructura en la cual el 25 finalmente se aloja).

---

## Pregunta 4
**Enunciado:** Baja de 94 en el árbol B+ dado (Orden 4).

**Análisis:**
- El 94 en este árbol actúa únicamente como un separador en el nodo 6 `4(94)5`. No existe como dato real en las hojas (las hojas son 0, 1, 3, 4 y 5, conteniendo los datos 5, 10, 30, 51, 60, 69, 80 y 104).
- Dado que el 94 es solo un separador histórico, su baja implica actualizar el separador en el nodo interno.
- Al "borrar" 94, se actualiza el separador del nodo 6 con la nueva primera clave del nodo derecho (nodo 5), pero en este escenario, las opciones (especialmente la C) sugieren un movimiento de claves (redistribución o actualización de separadores).
- Evaluando las opciones de la cátedra, la opción que refleja la actualización correcta de la estructura (usualmente redistribuyendo cuando el borrado se asume sobre un dato existente o actualizando el separador a 80) es la **C**.

**Respuesta Correcta:** **C)**

---

## Pregunta 5
**Enunciado:** ¿Cuál es la secuencia de L/E para la baja de 80 en el árbol B+ (Orden 4) con política izquierda?

**Análisis:**
- Árbol inicial: Nodo 4 tiene `[80]`.
- Al borrar 80, el nodo 4 queda vacío `[]` -> **UNDERFLOW**.
- Política Izquierda: Intentamos con hermano izquierdo. Nodo 4 es hijo izquierdo de nodo 6 `4(94)5`, por lo tanto, no tiene hermano izquierdo dentro de ese padre.
- Caso especial: Se usa el hermano derecho (nodo 5, con `[104]`). Nodo 5 no puede donar (tiene el mínimo de 1 clave).
- Se **fusiona** el nodo vacío con el nodo 5. Nodo 4 pasa a tener `[104]`. Nodo 5 se libera.
- Nodo 6 pierde el separador y su puntero, quedando vacío -> **UNDERFLOW** en nodo interno.
- Padre de nodo 6 es nodo 7. Hermano izquierdo de nodo 6 es nodo 2 `0(30)1(51)3`.
- Nodo 2 tiene 2 claves (`30` y `51`), puede donar.
- **Redistribución:** El hijo más derecho de nodo 2 (nodo 3) pasa al nodo 6. El separador `65` de nodo 7 baja a nodo 6. La última clave de nodo 2 (`51`) sube a nodo 7.
- Operaciones en disco: Leer nodos 7, 6, 4 y 5. Escribir 4. Leer 2. Escribir 2, 6, y 7.
- L/E: L7, L6, L4, L5, E4, L2, E2, E6, E7.

**Respuesta Correcta:** **D) L7,L6,L4,L5,E4,L2,E2,E6,E7.**
