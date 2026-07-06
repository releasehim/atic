# Práctica 4 - Ejercicio 5

## Fundamentos de Organización de Datos - UNLP

### Conceptos: Overflow, Underflow, Redistribución y Fusión en Árboles B

---

## 📖 Definiciones

---

### 🔴 Overflow (Desborde)

**Definición:**
El **overflow** ocurre cuando un nodo recibe una clave adicional que excede su capacidad máxima de almacenamiento. En un árbol B de orden M, un nodo puede contener como máximo **M-1 claves**. Si se intenta insertar una clave en un nodo que ya tiene M-1 claves, el nodo pasa a tener M claves → se produce overflow.

**¿Cuándo ocurre?**

- Durante una operación de **inserción** de una nueva clave.
- La clave nueva se inserta en el nodo hoja correspondiente siguiendo el criterio de ordenamiento.
- Si ese nodo hoja ya está lleno (M-1 claves), al agregar la nueva queda con M claves → overflow.

**¿Cómo se resuelve?**

1. Se **divide** el nodo desbordado en dos nodos.
2. Las M claves se distribuyen equitativamente:
   - Las `⌊M/2⌋` menores quedan en el nodo original.
   - Las `⌊M/2⌋` mayores pasan a un **nuevo nodo** (siempre se crea un nodo nuevo).
3. La clave del **medio** se **promueve** (sube) al nodo padre:
   - En **árbol B**: la clave del medio se mueve al padre (ya no está en los hijos).
   - En **árbol B+**: se copia la clave menor del nodo derecho al padre (la clave queda también en la hoja).
4. Si el padre también hace overflow, el proceso se **propaga hacia arriba**.
5. Si el overflow llega a la raíz, se crea una **nueva raíz** y el árbol gana altura.

> **Nota sobre órdenes pares (ej: orden 4):**
> Con 4 claves temporales se elige la **menor de las claves mayores** (posición ⌊M/2⌋ = 2ª clave) para promocionar.
> El nodo izquierdo queda con las claves menores a la promovida, el derecho con las mayores.

---

### 🟡 Underflow (Subdesborde)

**Definición:**
El **underflow** ocurre cuando un nodo queda con **menos claves del mínimo requerido** tras una eliminación. Para un nodo que NO es la raíz en un árbol B de orden M, el mínimo de claves es:

```
min_claves = ⌈M/2⌉ - 1
```

Por ejemplo, para M=4: min = ⌈4/2⌉ - 1 = 2 - 1 = **1 clave mínima**.
Para M=5: min = ⌈5/2⌉ - 1 = 3 - 1 = **2 claves mínimas**.

**¿Cuándo ocurre?**

- Durante una operación de **eliminación** de una clave.
- Después de eliminar la clave de un nodo, si el nodo queda con menos del mínimo de claves → underflow.
- También puede ocurrir en **cascada**: al resolver un underflow en un nodo hijo mediante fusión, el padre pierde una clave y puede también hacer underflow.

**La raíz** es un caso especial: puede tener **0 claves** (árbol vacío) o al menos 1 clave (con 2 hijos). Si la raíz queda con 0 claves tras una fusión, se elimina y el árbol pierde un nivel.

---

### 🟢 Redistribución

**Definición:**
La **redistribución** es el mecanismo que se aplica para resolver un underflow moviéndole claves desde un **nodo hermano adyacente** que tenga más del mínimo de claves (es decir, que pueda "donar" una clave sin caer en underflow él mismo).

**¿Cuándo se aplica?**

- SIEMPRE se intenta PRIMERO la redistribución antes que la fusión.
- Es aplicable cuando el hermano adyacente (izquierdo o derecho, según la política) tiene **más del mínimo de claves** (es decir, `cant_claves > min_claves`), lo que significa que puede ceder una clave sin caer en underflow.

**¿Cómo funciona?**

*Redistribución con hermano IZQUIERDO:*

1. La mayor clave del hermano izquierdo **sube** al padre (reemplaza al separador).
2. El separador del padre **baja** al nodo con underflow (se inserta como primera clave).
3. El árbol permanece balanceado y no se crean ni destruyen nodos.

*Redistribución con hermano DERECHO:*

1. La menor clave del hermano derecho **sube** al padre (reemplaza al separador).
2. El separador del padre **baja** al nodo con underflow (se inserta como última clave).
3. El árbol permanece balanceado y no se crean ni destruyen nodos.

> **Ventaja:** La redistribución NO cambia la cantidad de nodos en el árbol.
> Es una operación "barata" que solo requiere leer/escribir 3 nodos:
> el nodo deficiente, el hermano y el padre.

---

### 🔵 Fusión (o Concatenación)

**Definición:**
La **fusión** (o concatenación) es el mecanismo que se aplica para resolver un underflow cuando la redistribución NO es posible. Consiste en combinar el nodo deficiente, el separador del padre y el nodo hermano en un único nodo.

**¿Cuándo se aplica?**

- Cuando la redistribución NO es posible porque el hermano adyacente tiene exactamente el **mínimo de claves** (no puede ceder ninguna sin caer en underflow él mismo).
- Se aplica DESPUÉS de intentar (y fallar) la redistribución.

**¿Cómo funciona?**

1. Se toma el nodo con underflow (vacío o deficiente).
2. Se toma el **separador** del nodo padre que separa al nodo deficiente de su hermano.
3. Se toma el **nodo hermano** (con el mínimo de claves).
4. Los tres se **fusionan** en un único nodo:
   - `nodo_fusionado = claves_nodo_deficiente + separador_del_padre + claves_hermano`
5. El separador **se elimina del padre** y el nodo hermano se **libera** (queda vacío).
6. Si el padre pierde el separador y queda con menos del mínimo → el padre hace **underflow**, y el proceso se propaga hacia arriba.
7. Si el padre era la raíz y queda con 0 claves → la raíz se elimina y el árbol pierde un nivel.

> **Importante:** La fusión SIEMPRE libera (elimina) un nodo del árbol.
> Al contrario del overflow (que siempre crea un nodo), la fusión siempre elimina uno.
> La reutilización de nodos liberados se hace con política **LIFO** (último en entrar, primero en salir).

---

## 📋 Resumen Comparativo

| Concepto | Causa | Efecto | Nodos creados/destruidos |
| --- | --- | --- | --- |
| **Overflow** | Inserción en nodo lleno | División del nodo y promoción | Siempre crea 1 nodo nuevo |
| **Underflow** | Eliminación en nodo con mínimo | Nodo deficiente | Puede causar redistribución o fusión |
| **Redistribución** | Underflow con hermano "rico" | Claves se redistribuyen vía padre | No crea ni destruye nodos |
| **Fusión** | Underflow con hermano en mínimo | Nodos se unen, separador baja | Siempre elimina 1 nodo |

---

## 🔄 Orden de Aplicación ante Underflow

Según la **política de resolución** definida para el árbol:

1. **Intentar redistribuir** con el hermano indicado por la política (izquierdo, derecho, o ambos).
2. **Si la redistribución no es posible** (el hermano también tiene el mínimo):
   - **Fusionar** con el hermano indicado por la política.
3. **Propagación**: si la fusión causa underflow en el padre, repetir desde el paso 1 con el padre.

### Políticas de underflow

- **Política izquierda**: intentar redistribución con hermano izquierdo → si no se puede, fusión con hermano izquierdo.
- **Política derecha**: intentar redistribución con hermano derecho → si no se puede, fusión con hermano derecho.
- **Política izquierda o derecha**: intentar redistribución izq → derecho → si no, fusión izquierda.
- **Política derecha o izquierda**: intentar redistribución der → izquierdo → si no, fusión derecha.
- **Caso especial**: si el nodo es extremo (no tiene hermano de un lado), se usa el único hermano disponible.
