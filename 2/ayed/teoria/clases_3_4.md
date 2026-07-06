# Clase 3: Árboles Binarios

## 1. Conceptos Fundamentales y Terminología

Un **árbol binario** es una colección de nodos que puede estar vacía o formada por un nodo distinguido llamado **raíz**, y dos subárboles (izquierdo y derecho) que también son árboles binarios.

### Terminología Clave
*   **Nodo**: Elemento del árbol que contiene un dato y referencias a sus hijos.
*   **Raíz**: El nodo superior del árbol (profundidad 0).
*   **Hijo**: Nodo que cuelga directamente de otro nodo (padre).
*   **Hermano**: Nodos que comparten el mismo padre.
*   **Hoja**: Nodo que no tiene hijos.
*   **Camino**: Secuencia de nodos donde cada uno es padre del siguiente. La longitud es el número de aristas.
*   **Profundidad**: Longitud del camino desde la raíz hasta el nodo.
*   **Altura**: Longitud del camino más largo desde el nodo hasta una hoja. La altura de una hoja es 0. La altura de un árbol es la altura de su raíz.
*   **Ancestro / Descendiente**: Si existe un camino de \(n_1\) a \(n_2\), \(n_1\) es ancestro de \(n_2\), y \(n_2\) es descendiente de \(n_1\).

### Tipos de Árboles Binarios
1.  **Árbol Lleno (Full Binary Tree)**: Cada nodo interno tiene exactamente 2 hijos y todas las hojas están al mismo nivel (altura `h`).
2.  **Árbol Completo (Complete Binary Tree)**: Es lleno hasta el nivel `h-1`, y el nivel `h` se completa de izquierda a derecha.

```text
Árbol Lleno (Altura 2):          Árbol Completo (Altura 2):
        [A]                               [A]
       /   \                             /   \
     [B]   [C]                         [B]   [C]
    /  \   /  \                       /  \   /
  [D] [E] [F] [G]                   [D] [E] [F]
```

---

## 2. Representación en Java: La Clase `BinaryTree<T>`

La cátedra proporciona una implementación genérica de un árbol binario (sin propiedad de orden). Un nodo tiene tres atributos: el dato, el hijo izquierdo y el hijo derecho.

### Diagrama de Clases (UML en ASCII)

```text
+------------------------------------+
|          <<Class>>                  |
|          BinaryTree<T>              |
+------------------------------------+
| - data: T                           |
| - leftChild: BinaryTree<T>          |
| - rightChild: BinaryTree<T>         |
+------------------------------------+
| + BinaryTree()                      |
| + BinaryTree(data: T)               |
| + getData(): T                      |
| + setData(data: T): void            |
| + getLeftChild(): BinaryTree<T>     |
| + getRightChild(): BinaryTree<T>    |
| + addLeftChild(child: BinaryTree): void |
| + addRightChild(child: BinaryTree): void |
| + removeLeftChild(): void           |
| + removeRightChild(): void          |
| + isEmpty(): boolean                |
| + isLeaf(): boolean                 |
| + hasLeftChild(): boolean           |
| + hasRightChild(): boolean          |
| + toString(): String                |
| + contarHojas(): int                |
| + espejo(): BinaryTree<T>           |
| + entreNiveles(n, m): void          |
+------------------------------------+
```

### Métodos Importantes
*   **Constructores**: `BinaryTree()` crea un árbol vacío (nulo); `BinaryTree(T data)` crea un nodo raíz con el dato especificado.
*   **Getters/Setters**: Para `data`, `leftChild`, y `rightChild`.
*   **Verificaciones**: `hasLeftChild()` y `hasRightChild()` son fundamentales antes de intentar acceder a `getLeftChild()` o `getRightChild()`, para evitar `NullPointerException`.
*   **Estado**: `isEmpty()` verifica si el nodo y sus hijos son `null`; `isLeaf()` verifica si ambos hijos son `null`.

### Ejemplo de Instanciación y Recorrido Preorden (dentro de la clase)

```java
// Creación del árbol del video: Raíz 40, hijoIzq 25, hijoDer 78.
BinaryTree<Integer> ab = new BinaryTree<Integer>(40);
BinaryTree<Integer> hijoIzquierdo = new BinaryTree<Integer>(25);
hijoIzquierdo.addLeftChild(new BinaryTree<Integer>(10));
hijoIzquierdo.addRightChild(new BinaryTree<Integer>(32));
BinaryTree<Integer> hijoDerecho = new BinaryTree<Integer>(78);
ab.addLeftChild(hijoIzquierdo);
ab.addRightChild(hijoDerecho);

// Recorrido preorden (definido en la misma clase BinaryTree)
public void printPreorden() {
    System.out.println(this.getData());
    if (this.hasLeftChild()) {
        this.getLeftChild().printPreorden();
    }
    if (this.hasRightChild()) {
        this.getRightChild().printPreorden();
    }
}
```

```text
Representación ASCII del árbol creado:
            [40]
           /    \
        [25]    [78]
       /    \
    [10]    [32]
```

---

## 3. Recorridos sobre Árboles Binarios

Existen dos grandes familias de recorridos:
1.  **Recorridos en Profundidad**: Se implementan con **recursión**.
2.  **Recorrido por Niveles**: Se implementa con **iteración** usando una cola.

### 3.1 Recorridos en Profundidad (Recursivos)

Los tres métodos clásicos para recorrer un árbol en profundidad:

| Recorrido | Orden de Visita | Código (Dentro de la clase) | Orden para el árbol de ejemplo |
| :--- | :--- | :--- | :--- |
| **Preorden** | Raíz → Izquierdo → Derecho | `imprimir(dato); hijoIzq.preorden(); hijoDer.preorden();` | 40, 25, 10, 32, 78 |
| **Inorden** | Izquierdo → Raíz → Derecho | `hijoIzq.inorden(); imprimir(dato); hijoDer.inorden();` | 10, 25, 32, 40, 78 |
| **Postorden** | Izquierdo → Derecho → Raíz | `hijoIzq.postorden(); hijoDer.postorden(); imprimir(dato);` | 10, 32, 25, 78, 40 |

### 3.2 Recorrido por Niveles (Iterativo)

Este recorrido utiliza una cola para visitar los nodos nivel por nivel, de izquierda a derecha.

```java
public void printLevelTraversal() {
    if (this.isEmpty()) return;
    Queue<BinaryTree<T>> cola = new Queue<>();
    cola.enqueue(this);
    while (!cola.isEmpty()) {
        BinaryTree<T> actual = cola.dequeue();
        System.out.print(actual.getData() + " ");
        if (actual.hasLeftChild()) cola.enqueue(actual.getLeftChild());
        if (actual.hasRightChild()) cola.enqueue(actual.getRightChild());
    }
}
```

```text
Diagrama del Recorrido Por Niveles (Usando Cola):
Paso 1: Encolar Raíz (40). Cola: [40]
Paso 2: Desencolar 40, imprimir 40. Encolar 25 y 78. Cola: [25, 78]
Paso 3: Desencolar 25, imprimir 25. Encolar 10 y 32. Cola: [78, 10, 32]
Paso 4: Desencolar 78, imprimir 78. (Sin hijos). Cola: [10, 32]
...
```

### 3.3 Recorrido en una Clase Externa

Si el método de recorrido debe definirse en una clase separada (ej. `ImpresorArbolBinario`), el árbol debe pasarse como parámetro en cada llamada recursiva.

```java
public class ImpresorArbolBinario<T> {
    public void preorden(BinaryTree<T> ab) {
        System.out.println(ab.getData());
        if (ab.hasLeftChild()) {
            preorden(ab.getLeftChild());
        }
        if (ab.hasRightChild()) {
            preorden(ab.getRightChild());
        }
    }
}
```

---

## 4. Árboles de Expresión

Un **árbol de expresión** es un árbol binario que representa una expresión aritmética:
*   **Nodos internos**: Representan **operadores** (+, -, *, /).
*   **Nodos externos (hojas)**: Representan **operandos** (números o variables).

### Aplicaciones
*   Compiladores (análisis, optimización, traducción).
*   Evaluación de expresiones algebraicas o lógicas.
*   Generación de notaciones prefija, infija y postfija.

### Recorridos y Notaciones

```text
Ejemplo de Árbol de Expresión: ((a + b) * (c - d)) / (e + f)

              [/]
             /   \
           [*]   [+]
          /  \   /  \
        [+] [-] [e] [f]
       /  \ /  \
     [a] [b][c][d]
```

*   **Inorden (Infija)**: `(((a + b) * (c – d)) / (e + f))`. Requiere paréntesis para preservar el orden.
*   **Preorden (Prefija)**: `/*+ab-cd+ef`. No necesita paréntesis.
*   **Postorden (Postfija)**: `ab+cd-*ef+/`. No necesita paréntesis.

### 4.1 Construcción desde Expresión Postfija (Estrategia 1: Pila)

Se lee la expresión postfija de izquierda a derecha usando una **pila**.
1.  Si es **operando**: Crear nodo y apilarlo.
2.  Si es **operador**:
    *   Crear nodo `R` con el operador.
    *   Desapilar y asignar como `hijo derecho` de `R`.
    *   Desapilar y asignar como `hijo izquierdo` de `R`.
    *   Apilar `R`.
3.  Al final, la pila contiene un único nodo que es la raíz del árbol.

```java
public BinaryTree<Character> convertirPostfija(String exp) {
    Stack<BinaryTree<Character>> p = new Stack<>();
    for (int i = 0; i < exp.length(); i++) {
        Character c = exp.charAt(i);
        BinaryTree<Character> resultado = new BinaryTree<>(c);
        if (esOperador(c)) {
            resultado.addRightChild(p.pop());
            resultado.addLeftChild(p.pop());
        }
        p.push(resultado);
    }
    return p.pop();
}
```

### 4.2 Construcción desde Expresión Prefija (Estrategia 2: Recursión)

Se lee la expresión prefija de izquierda a derecha usando **recursión**.
1.  Tomar el primer carácter de la expresión.
2.  Crear un nodo `R`.
3.  Si es **operador**:
    *   `R.addLeftChild(convertirPrefija(resto))`.
    *   `R.addRightChild(convertirPrefija(resto))`.
4.  Si es **operando**: Retornar el nodo `R`.

```java
public BinaryTree<Character> convertirPrefija(StringBuffer exp) {
    Character c = exp.charAt(0);
    BinaryTree<Character> resultado = new BinaryTree<>(c);
    if (esOperador(c)) {
        exp.delete(0, 1);
        resultado.addLeftChild(convertirPrefija(exp));
        exp.delete(0, 1);
        resultado.addRightChild(convertirPrefija(exp));
    }
    return resultado;
}
```

### 4.3 Construcción desde Expresión Infija (Estrategia 3: Dos Pasos)
1.  Convertir la expresión infija a postfija utilizando una pila y considerando las **prioridades de operadores**.
2.  Aplicar la Estrategia 1 (Postfija → Árbol).

*   **Prioridades**: `^` > `* /` > `+ -`. Los `(` se apilan siempre y se desapilan solo al encontrar un `)`.

### 4.4 Evaluación de un Árbol de Expresión (Recursivo)

```java
public Integer evaluar(BinaryTree<Character> arbol) {
    Character c = arbol.getData();
    if (esOperador(c)) {
        int operador1 = evaluar(arbol.getLeftChild());
        int operador2 = evaluar(arbol.getRightChild());
        switch (c) {
            case '+': return operador1 + operador2;
            case '-': return operador1 - operador2;
            case '*': return operador1 * operador2;
            case '/': return operador1 / operador2;
        }
    }
    return Integer.parseInt(c.toString()); // Caso base: hoja
}
```

---

## 5. Ejercicios y Aplicaciones (Explicados en Video)

### 5.1 Ejercicio: Suma de Árboles Binarios (Ejemplo 1 del video)

**Problema**: Dados dos árboles binarios `arbol1` y `arbol2` de enteros, implementar un método que devuelva un nuevo árbol que contenga la suma de los nodos en la misma posición.
*   **Condición**: Solo se pueden sumar si tienen la **misma estructura**. Si las estructuras difieren, se debe devolver un árbol vacío.
*   **Estrategia**: Usar un método auxiliar privado que utiliza recursión y una variable booleana `estructuraValida` para propagar el éxito/fracaso de la operación.

**Código Esencial**:
```java
public BinaryTree<Integer> sumarArboles(BinaryTree<Integer> arbol1, BinaryTree<Integer> arbol2) {
    BinaryTree<Integer> suma = new BinaryTree<>(); // Árbol vacío
    if (arbol1.isEmpty() && arbol2.isEmpty()) return suma;
    // Chequeo previo de vacíos...
    boolean exito = sumarArbolesAux(arbol1, arbol2, suma);
    return exito ? suma : new BinaryTree<>();
}

private boolean sumarArbolesAux(BinaryTree<Integer> a1, BinaryTree<Integer> a2, BinaryTree<Integer> suma) {
    if (a1.isEmpty() && a2.isEmpty()) return true;
    if (a1.isEmpty() != a2.isEmpty()) return false; // Estructura diferente

    suma.setData(a1.getData() + a2.getData());
    boolean exito = true;

    if (a1.hasLeftChild() && a2.hasLeftChild()) {
        BinaryTree<Integer> sumaIzq = new BinaryTree<>();
        suma.addLeftChild(sumaIzq);
        exito = exito && sumarArbolesAux(a1.getLeftChild(), a2.getLeftChild(), sumaIzq);
    } else if (a1.hasLeftChild() != a2.hasLeftChild()) {
        return false;
    }

    if (exito && a1.hasRightChild() && a2.hasRightChild()) {
        BinaryTree<Integer> sumaDer = new BinaryTree<>();
        suma.addRightChild(sumaDer);
        exito = exito && sumarArbolesAux(a1.getRightChild(), a2.getRightChild(), sumaDer);
    } else if (a1.hasRightChild() != a2.hasRightChild()) {
        return false;
    }
    return exito;
}
```

```text
Diagrama del ejercicio (Suma de Árboles):
Árbol 1:       Árbol 2:       Árbol Suma (Resultado):
   [3]            [5]              [8]
  /   \          /   \            /   \
[4]   [2]      [7]   [1]        [11]  [3]
/               /               /
[8]            [9]             [17]
```

### 5.2 Ejercicio de Parcial: Primer Camino con `min` Pares (Ejemplo 2 del video)

**Problema**: Dado un árbol binario de enteros, devolver el primer **camino** (desde la raíz hasta una hoja) que contenga una cantidad de números pares mayor o igual a un parámetro `min`.

**Estrategia**: Recorrido en profundidad (Preorden) con **backtracking**.
*   Se mantiene una lista `camino` con los nodos visitados hasta el momento.
*   Se lleva un contador `paresAcumulados`.
*   Al llegar a una **hoja**, se verifica `paresAcumulados >= min`. Si se cumple, se retorna la lista. Si no, se elimina el último elemento de la lista (`backtracking`) antes de regresar.
*   Se usa una variable booleana `encontrado` para cortar la recursión y no recorrer el resto del árbol cuando se halla la solución.
*   **Atención**: `encontrado` debe retornarse en el método auxiliar para propagar el éxito (porque los tipos primitivos se pasan por valor y no se modificaría la variable original si no se retorna).

**Código Esencial**:
```java
public List<Integer> resolver(BinaryTree<Integer> arbol, int min) {
    List<Integer> camino = new LinkedList<>();
    if (arbol != null && !arbol.isEmpty()) {
        buscarCamino(arbol, min, camino, 0);
    }
    return camino;
}

private boolean buscarCamino(BinaryTree<Integer> arbol, int min, List<Integer> camino, int pares) {
    boolean encontrado = false;
    camino.add(arbol.getData());
    if (arbol.getData() % 2 == 0) pares++;

    if (arbol.isLeaf()) {
        if (pares >= min) return true; // Encontrado
    } else {
        if (arbol.hasLeftChild()) {
            encontrado = buscarCamino(arbol.getLeftChild(), min, camino, pares);
        }
        if (!encontrado && arbol.hasRightChild()) {
            encontrado = buscarCamino(arbol.getRightChild(), min, camino, pares);
        }
    }

    if (!encontrado) {
        camino.remove(camino.size() - 1); // Backtracking
    }
    return encontrado;
}
```

### 5.3 Problema: Encontrar la Valencia Total (Ejemplo 3 del PDF)

**Problema (SPOJ - UCV2013J)**: Se recibe un *stream* de enteros que representa un árbol binario completo (llenando los niveles de izquierda a derecha). El primer número es `N`, seguido de `N` valencias. Se debe calcular la suma de las valencias de las **hojas**.

**Estrategia (Construcción Iterativa)**: Dado que el árbol es **completo**, se puede usar una cola para construirlo nivel por nivel.
*   El primer valor después de `N` es la raíz.
*   Los siguientes valores se asignan secuencialmente a los hijos izquierdos y derechos de los nodos en la cola.
*   **Estrategia (Construcción Recursiva en Array)**: Como el árbol es completo, si almacenamos los datos en un arreglo (índice 1 en adelante), el hijo izquierdo de un nodo en la posición `i` está en `2*i`, y el hijo derecho en `2*i+1`.

```java
// Construcción Recursiva a partir de un arreglo `stream`
private static BinaryTree<Integer> createBinaryTreeRec(int[] stream, int i) {
    if (i >= stream.length) return null;
    BinaryTree<Integer> ab = new BinaryTree<>(stream[i]);
    ab.addLeftChild(createBinaryTreeRec(stream, 2 * i));
    ab.addRightChild(createBinaryTreeRec(stream, 2 * i + 1));
    return ab;
}

// Cálculo de Valencia
public static int calcularValencia(BinaryTree<Integer> arbol) {
    if (arbol.isLeaf()) return arbol.getData();
    int suma = 0;
    if (arbol.hasLeftChild()) suma += calcularValencia(arbol.getLeftChild());
    if (arbol.hasRightChild()) suma += calcularValencia(arbol.getRightChild());
    return suma;
}
```

```text
Stream: [6, 4, 3, 2, 6, 0, 3]
Índice: [0, 1, 2, 3, 4, 5, 6]
N=6, Raíz=4, Nivel1=3 y 2, Nivel2=6, 0, 3.

Representación del árbol completo en el array (con raíz en índice 1):
          Índice 1: 4
         /          \
 Índice 2: 3   Índice 3: 2
   /    \             \
4:6   5:0           6:3
```

### 5.4 Ejercitación Adicional (PDF 3.1 - Recorridos)

Dados los siguientes árboles, calcular los recorridos solicitados:

```text
   a)       b)              c)
    7        3               8
   / \        \              /
  9   8        7             10
 / \          / \           /  \
10 11        8   9         13   15
             / \   \
            10 11  18

Resolución (Preorden, Inorden, Postorden):
a) Pre: 7, 9, 10, 11, 8  | In: 10, 9, 11, 7, 8  | Post: 10, 11, 9, 8, 7
b) Pre: 3, 7, 8, 10, 11, 9, 18 | In: 3, 10, 8, 11, 7, 9, 18 | Post: 10, 11, 8, 18, 9, 7, 3
c) Pre: 8, 10, 13, 15    | In: 13, 10, 15, 8    | Post: 13, 15, 10, 8
```

### 5.5 Ejercicio Avanzado (PDF 3.1 - Construcción desde Recorridos)

**Ejercicio**: Dados los recorridos `inorden: C B F E G A D I H` y `postorden: C F G E B I H D A`, construir el árbol original.
*   **Método**: En postorden, el último elemento es la raíz (`A`). En inorden, todo lo que está a la izquierda de `A` (`C B F E G`) es el subárbol izquierdo, y lo que está a la derecha (`D I H`) es el subárbol derecho. Se repite el proceso recursivamente.
*   **Árbol Final**:
```text
                [A]
               /   \
             [B]   [D]
            /   \     \
          [C]   [E]   [H]
              /   \   /
            [F]   [G] [I]
```