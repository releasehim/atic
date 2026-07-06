# Árboles Generales

## 1. Definición y Terminología
Un **árbol general** es una colección de nodos donde cada nodo puede tener **cualquier cantidad de hijos** (a diferencia del árbol binario, que tiene como máximo 2).

*   **Raíz**: El nodo distinguido del cual cuelga el resto del árbol.
*   **Hijo**: Cualquier nodo que cuelga de otro nodo (padre).
*   **Hoja**: Nodo que no tiene hijos.
*   **Grado de un nodo**: Número de hijos que posee.
*   **Grado del árbol**: El mayor grado entre todos sus nodos.
*   **Profundidad / Nivel**: Longitud del camino único desde la raíz hasta el nodo. La raíz tiene nivel 0.
*   **Altura**: Longitud del camino más largo desde el nodo hasta una hoja.
*   **Camino**: Secuencia de nodos donde cada uno es padre del siguiente.

### Árboles Llenos y Completos (Grado \(k\), Altura \(h\))
*   **Árbol Lleno**: Todos los nodos internos tienen **exactamente grado \(k\)** y todas las hojas están al mismo nivel (nivel \(h\)).
    *   **Cantidad de nodos**: \(N = \frac{k^{h+1} - 1}{k - 1}\).
*   **Árbol Completo**: Es lleno hasta el nivel \(h-1\), y el último nivel (\(h\)) se completa de izquierda a derecha.
    *   **Cantidad de nodos**: Varía entre \(\frac{k^h + k - 2}{k - 1}\) y \(\frac{k^{h+1} - 1}{k - 1}\).

## 2. Representación en Java: La Clase `GeneralTree<T>`
La implementación proporcionada por la cátedra utiliza una **lista dinámica** de hijos (`List<GeneralTree<T>>`).

```text
Diagrama UML (ASCII) de GeneralTree<T>:
+------------------------------------+
|          GeneralTree<T>             |
+------------------------------------+
| - data: T                           |
| - children: List<GeneralTree<T>>    |
+------------------------------------+
| + GeneralTree(data: T)              |
| + GeneralTree(data: T, children: List)|
| + getData(): T                      |
| + setData(data: T): void            |
| + getChildren(): List<GeneralTree>  |
| + addChild(child: GeneralTree): void|
| + isLeaf(): boolean                 |
| + hasChildren(): boolean            |
| + isEmpty(): boolean                |
| + removeChild(child: GeneralTree)   |
| + printPreOrder(): void             |
| + preOrder(): List<T>               |
+------------------------------------+
```

**Métodos Clave:**
*   `getChildren()`: Retorna una `List<GeneralTree<T>>` con los hijos del nodo.
*   `hasChildren()`: Verifica si el nodo no es una hoja.
*   `addChild(child)`: Agrega un nuevo hijo a la lista de hijos.

### Ejemplo de Creación y Recorrido Preorden
```java
package tp3;

public class GeneralTree<T> {
    // ... atributos y constructores ...

    public void printPreOrder() {
        System.out.println(this.getData());
        List<GeneralTree<T>> children = this.getChildren();
        for (GeneralTree<T> child : children) {
            child.printPreOrder();
        }
    }
}
```
```text
Ejemplo de árbol creado y su salida Preorden:
           [0]
          / | \
       [1] [2] [3]
          /|\   /\
       [21][22][23][31][32]
Salida: 0, 1, 2, 21, 22, 23, 3, 31, 32
```

## 3. Recorridos y Algoritmos Fundacionales

### 3.1. Recorrido en Profundidad (Preorden y Postorden)
En árboles generales no existe un *Inorden* estándar como en binarios, pero una variante podría ser: primero el hijo más izquierdo, luego la raíz, luego los demás hijos.

**Preorden:** Procesa la raíz, luego los hijos en orden.
```java
public List<T> preOrder() {
    List<T> lista = new LinkedList<T>();
    preOrderRecursivo(lista);
    return lista;
}

private void preOrderRecursivo(List<T> l) {
    l.add(this.getData());
    for (GeneralTree<T> child : this.getChildren()) {
        child.preOrderRecursivo(l);
    }
}
```
**Postorden:** Procesa los hijos recursivamente, luego la raíz.

### 3.2. Recorrido Por Niveles (Iterativo con Cola)
Se utiliza una cola (`Queue`) para visitar los nodos nivel por nivel, de izquierda a derecha.

```java
public List<T> traversalLevel(GeneralTree<T> tree) {
    List<T> result = new LinkedList<T>();
    Queue<GeneralTree<T>> queue = new Queue<GeneralTree<T>>();
    queue.enqueue(tree);

    while (!queue.isEmpty()) {
        GeneralTree<T> current = queue.dequeue();
        result.add(current.getData());
        for (GeneralTree<T> child : current.getChildren()) {
            queue.enqueue(child);
        }
    }
    return result;
}
```

---

## 4. Aplicaciones y Ejercicios Prácticos (Resueltos en Video)

### 4.1. Ejercicio: Primer Camino con Todos los Valores Negativos
*Descripción:* Encontrar el primer camino (Raíz → Hoja) donde todos los valores sean números negativos.
*Estrategias:*
1.  **Top-Down (con Backtracking)**: Se construye el camino de arriba hacia abajo. Si el valor es negativo, se agrega. Si se llega a una hoja y todo fue negativo, se retorna `true`. Si falla, se saca el último elemento de la lista (**Backtracking**) antes de volver. Se puede optimizar verificando la negatividad *antes* de la llamada recursiva.
2.  **Bottom-Up (Sin Backtracking)**: Se recorre en Postorden. Solo se baja por hijos negativos. Si el hijo devuelve `true` (encontró camino), el padre se agrega al **inicio** de la lista y retorna `true`. De esta forma, el camino se va armando desde la hoja hacia arriba.

**Código Esencial (Versión Bottom-Up):**
```java
private boolean buscarCaminoNegativo(GeneralTree<Integer> arbol, List<Integer> camino) {
    boolean encontre = false;
    if (arbol.isLeaf()) {
        camino.add(0, arbol.getData());
        return true;
    }
    List<GeneralTree<Integer>> children = arbol.getChildren();
    Iterator<GeneralTree<Integer>> it = children.iterator();
    while (it.hasNext() && !encontre) {
        GeneralTree<Integer> child = it.next();
        if (child.getData() < 0) { // Validación antes de llamar
            encontre = buscarCaminoNegativo(child, camino);
        }
    }
    if (encontre) {
        camino.add(0, arbol.getData()); // Agregar el padre al inicio
    }
    return encontre;
}
```

### 4.2. Ejercicio: Camino de Suma Mínima
*Descripción:* Encontrar el camino (Raíz → Hoja) cuya suma de valores sea la menor posible. Si hay varios, devolver el primero hallado (izquierda a derecha).
*Estrategia:* Top-Down con Backtracking.
*   Se mantiene un `caminoActual` y un `caminoMinimo`.
*   Se va sumando el valor al `costoActual`.
*   Al llegar a una hoja, se compara con el `costoMinimo` (inicializado en `Integer.MAX_VALUE`).
*   Si es mejor, se limpia `caminoMinimo` y se copia `caminoActual`.
*   Se usa el patrón **Backtracking** para eliminar el nodo actual de la lista `caminoActual` al salir del método.

### 4.3. Ejercicio: Frontera Par (Hojas Pares)
*Descripción:* Devolver una lista con los valores pares de las hojas del árbol, recorridas de izquierda a derecha.
*Estrategia:* Recorrido Preorden simple.
```java
public static List<Integer> fronteraPar(GeneralTree<Integer> arbol) {
    List<Integer> resultado = new LinkedList<>();
    if (arbol != null && !arbol.isEmpty()) {
        fronteraParRec(arbol, resultado);
    }
    return resultado;
}
private static void fronteraParRec(GeneralTree<Integer> arbol, List<Integer> resultado) {
    if (arbol.isLeaf()) {
        if (arbol.getData() % 2 == 0) {
            resultado.add(arbol.getData());
        }
    } else {
        for (GeneralTree<Integer> child : arbol.getChildren()) {
            fronteraParRec(child, resultado);
        }
    }
}
```

### 4.4. Ejercicio de Parcial: Árbol Sustituto
*Descripción:* Determinar si `arbol1` es sustituto de `arbol2`.
*Condiciones:*
1.  `arbol1` debe ser **Par** y **Mayor** que `arbol2`.
2.  Si `arbol1` es hoja y cumple (1) -> Verdadero.
3.  Si `arbol1` no es hoja y `arbol2` es hoja -> Falso.
4.  Si ambos tienen hijos: `arbol1` debe tener **más** hijos pares que `arbol2`.
5.  Validar recursivamente cada par de hijos en la misma posición.

**Código Esencial:**
```java
private boolean esSustitutoRec(GeneralTree<Integer> a1, GeneralTree<Integer> a2) {
    // Valida Par y Mayor
    if (a1.getData() % 2 != 0 || a1.getData() <= a2.getData()) return false;
    
    if (a1.isLeaf()) return true; // Aplica regla 2
    
    if (a2.isLeaf()) return false; // Aplica regla 3
    
    // Aplica regla 4
    if (contarHijosPares(a1) <= contarHijosPares(a2)) return false;
    
    // Recursión para todos los hijos
    Iterator<GeneralTree<Integer>> it1 = a1.getChildren().iterator();
    Iterator<GeneralTree<Integer>> it2 = a2.getChildren().iterator();
    boolean cumple = true;
    while (it1.hasNext() && it2.hasNext() && cumple) {
        cumple = esSustitutoRec(it1.next(), it2.next());
    }
    return cumple;
}
```

### 4.5. Ejercicio de Teoría (Abeto Navideño - Codeforces)
*Descripción:* Verificar si todos los nodos internos (no hojas) tienen **al menos 3** hijos que sean **hojas**.
*Entrada:* `N` (nodos), seguido de `N-1` líneas con el índice del padre del nodo `i+1`.

```text
Input (Ejemplo 1):
4
1
1
1
Output: Yes

Representación ASCII:
    [1]
   / | \
 [2][3][4]  <-- Todos son hojas del nodo 1. 3 >= 3. Yes.
```

---

## 5. Notas sobre Backtracking en Árboles Generales
Cuando se recorren caminos con listas en Java, el manejo de la referencia es fundamental.

1.  **Hijo Empoderado (Autónomo)**:
    *   El padre llama al hijo.
    *   El hijo se agrega a la lista, busca su camino y **se remueve a sí mismo** de la lista antes de retornar, tanto si encontró el camino como si no.
    *   Esto se hace típicamente al final de la ejecución del método recursivo del hijo.
2.  **Hijo Perezoso (Padre controlador)**:
    *   El padre llama al hijo.
    *   El hijo se agrega a la lista, busca su camino y retorna.
    *   **El padre** elimina al hijo de la lista después de que el hijo haya retornado (ya sea porque encontró o no el camino).

```java
// Ejemplo de Hijo Empoderado (remueve al final del método)
private boolean buscar(GeneralTree<Integer> n, List<Integer> camino) {
    camino.add(n.getData()); // Agregar
    if (n.isLeaf()) return true; // (Condición cumplida)
    
    for (GeneralTree<Integer> h : n.getChildren()) {
        if (buscar(h, camino)) return true;
    }
    
    // Backtracking: Si llegó acá, no encontró nada en esta rama
    camino.remove(camino.size() - 1); 
    return false;
}
```

---

## 6. Ejercitación Teórica (AyED 2026)

1.  **Dado Inorden (A B G E F D J H I C) y Postorden (A G F E B J I H C D), ¿Cuántos descendientes tiene C?**
    *   *Resolución:* Postorden: Último elemento es Raíz `D`. Buscamos C en Inorden a la derecha de `D` (J H I C). C es raíz de subárbol derecho. Hijos de C son I y H (según Postorden J I H C...). C tiene 3 descendientes (I, H, J). **Opción (c) 3**.

2.  **Definir Árbol Binario Completo y Lleno. Ejemplos.**
    *   *Lleno:* Todos los nodos internos tienen 2 hijos y todas las hojas al mismo nivel.
    *   *Completo:* Lleno hasta el penúltimo nivel, el último nivel se llena de izquierda a derecha.

3.  **Expresión Postfija: `6 5 * 7 3 - 4 8 * + +`**
    *   *Resultado:* 34. (30 + (7-3) + (4*8) + 4?). Evaluemos: 6*5=30. 7-3=4. 4*8=32. 30+4+32=66. **Opción (b) 66**. (¡Cuidado con el cálculo!).

4.  **Número mínimo de nodos en un árbol binario completo de altura 4:**
    *   *Resolución:* \(2^{h} = 2^{4} = 16\) nodos mínimo. **Opción (e) 16**.

5.  **Ejercicio 15 (Árbol general):**
    *   *a) Completar blancos:*
        *   i. `A` es la raíz del árbol.
        *   ii. `A` es padre de B, C y D.
        *   iii. `C` y `D` son hermanos (o E y F, G y H...).
        *   iv. `E, J, K, L, N, O, P, Q, R` son las hojas del árbol.
        *   v. Camino A-J es `A, B, E, J`.
        *   vi. `D` es ancestro de P (y D es ancestro de P, por lo tanto P es descendiente de D).
        *   vii. `L` no es descendiente de C, puesto que no existe camino desde C a L.
        *   viii. Profundidad de C es `1`, F es `2`, y `R` es `4`.
        *   ix. Altura de C es `0` (hoja), y `1` y D es `2`.
        *   x. Altura del árbol es `4` (largo del camino entre la raíz A y la hoja R).
    *   *b) Recorridos:*
        *   Preorden: A B E J K F L G M P Q H N O C D I R
        *   Inorden (General): J E K B L F G M P Q H N O C A R I D
        *   Postorden: J K E L P Q M G H N O F B C A R I D
        *   Por niveles: A B C D E F G H I J K L M N O P Q R

6.  **Si un árbol general lleno de grado 5 tiene 125 hojas. ¿Cuántos nodos internos?**
    *   *Resolución:* \(5^h = 125 \rightarrow 5^h = 5^3 \rightarrow h = 3\). Nodos totales = \(5^3 + 5^2 + 5^1 + 5^0 = 125 + 25 + 5 + 1 = 156\). Nodos internos = Total - Hojas = 156 - 125 = **31 nodos internos**.