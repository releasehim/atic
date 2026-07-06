# Práctica 2: Árboles Binarios

Implemente cada ejercicio en un paquete que contenga los números del TP y del ejercicio. Ejemplo `tp2.ejercicio3` (dentro del proyecto llamado “AYED”).

---

## Diagrama de Clases (UML) - `BinaryTree<T>`

```text
+------------------------------------+
|          BinaryTree<T>              |
+------------------------------------+
| - data: T                           |
| - leftChild: BinaryTree<T>          |
| - rightChild: BinaryTree<T>         |
+------------------------------------+
| + BinaryTree()                      |
| + BinaryTree(T)                     |
| + getData(): T                      |
| + setData(T): void                  |
| + getLeftChild(): BinaryTree<T>     |
| + getRightChild(): BinaryTree<T>    |
| + addLeftChild(BinaryTree<T>): void |
| + addRightChild(BinaryTree<T>): void|
| + removeLeftChild(): void           |
| + removeRightChild(): void          |
| + isEmpty(): boolean                |
| + isLeaf(): boolean                 |
| + hasLeftChild(): boolean           |
| + hasRightChild(): boolean          |
| + toString(): String                |
| + contarHojas(): int                |
| + espejo(): BinaryTree<T>           |
| + entreNiveles(int, int): void      |
+------------------------------------+
```

---

## Ejercicio 1

Considere la siguiente especificación de la clase Java `BinaryTree` (con la representación hijo izquierdo e hijo derecho).

* El constructor `BinaryTree(T data)` inicializa un árbol con el dato pasado como parámetro y ambos hijos nulos.
* Los métodos `getLeftChild():BinaryTree<T>` y `getRightChild():BinaryTree<T>`, retornan los hijos izquierdo y derecho respectivamente del árbol. Si no tiene el hijo tira error.
* El método `addLeftChild(BinaryTree<T> child)` y `addRightChild(BinaryTree<T> child)` agrega un hijo como hijo izquierdo o derecho del árbol.
* El método `removeLeftChild()` y `removeRightChild()`, eliminan el hijo correspondiente.
* El método `isEmpty()` indica si el árbol está vacío y el método `isLeaf()` indica si no tiene hijos.
* El método `hasLeftChild()` y `hasRightChild()` devuelve un booleano indicando si tiene dicho hijo el árbol receptor del mensaje.

**a)** Analice la implementación en JAVA de la clase `BinaryTree` brindada por la cátedra.

---

## Ejercicio 4

Los nodos que conforman una red binaria llena tiene la particularidad de que todos ellos conocen cuál es su retardo de reenvío. El retardo de reenvío se define como el período comprendido entre que un nodo recibe un mensaje y lo reenvía a sus dos hijos.

Su tarea es calcular el mayor retardo posible, en el camino que realiza un mensaje desde la raíz hasta llegar a las hojas en una red binaria llena. En el ejemplo, debería retornar $10 + 3 + 9 + 12 = 34$ (Si hay más de un máximo retorno el último valor hallado).

**Nota**: asuma que cada nodo tiene el dato de retardo de reenvío expresado en cantidad de segundos.

**a)** Indique qué estrategia (recorrido en profundidad o por niveles) utilizará para resolver el problema.
**b)** Cree una clase Java llamada `RedBinariaLlena` donde implementará lo solicitado en el método `retardoReenvío():int`.

---

## Ejercicio 5

Implemente una clase Java llamada `ProfundidadDeArbolBinario` que tiene como variable de instancia un árbol binario de números enteros y un método de instancia `sumaElementosProfundidad (int p):int` el cual devuelve la suma de todos los nodos del árbol que se encuentren a la profundidad pasada como argumento.

---

## Ejercicio 6

Cree una clase Java llamada `Transformation` que tenga como variable de instancia un árbol binario de números enteros y un método de instancia `suma(): BinaryTree<Integer>` el cual devuelve el árbol en el que se reemplazó el valor de cada nodo por la suma de todos los elementos presentes en su subárbol izquierdo y derecho. Asuma que los valores de los subárboles vacíos son ceros.

**Por ejemplo**:

```text
Árbol Original:
          [1]
         /   \
       [2]   [3]
      /     /   \
    [4]   [5]   [6]
         /   \
       [7]   [8]

Árbol Transformado (Resultado):
          [35]  <-- (2+4) + (3+5+6+7+8)
         /    \
       [4]   [26] <-- (5+7+8) + (6)
      /     /    \
    [0]   [15]  [0]
         /   \
       [0]   [0]
```

**¿Su solución recorre una única vez cada subárbol? En el caso que no, ¿Puede mejorarla para que sí lo haga?**

---

## Ejercicio 7

**Restricciones para el desarrollo de ejercicios 7, 8 y 9**:

1. No puede agregar más variables de instancia ni de clase a la clase `ParcialArboles`.
2. Debe respetar la clase y la firma del método indicado.
3. Puede definir todos los métodos y variables locales que considere necesarios.
4. Todo método que no esté definido en la sinopsis de clases debe ser implementado.
5. Debe recorrer la estructura solo 1 vez para resolverlo.
6. Si corresponde, complete en la firma del método el tipo de datos indicado con signo de “?”.

### Enunciado

Escribir en una clase `ParcialArboles` que contenga **UNA ÚNICA** variable de instancia de tipo `BinaryTree` de valores enteros **NO repetidos** y el método público con la siguiente firma:

`public boolean isLeftTree (int num)`

El método devuelve `true` si el subárbol cuya raíz es “num”, tiene en su subárbol izquierdo una cantidad mayor estricta de árboles con un único hijo que en su subárbol derecho. Y `false` en caso contrario.

**Consideraciones**:

* Si “num” no se encuentra en el árbol, devuelve `false`.
* Si el árbol con raíz “num” no cuenta con una de sus ramas, considere que en esa rama hay -1 árboles con único hijo.

**Por ejemplo**, con un árbol como se muestra en la siguiente imagen:

```text
Árbol de ejemplo:
                   [2]
                  /   \
                [7]   [-5]
               /   \       \
            [23]   [6]     [19]
            /      / \       \
          [-3]  [55] [11]    [4]
                              \
                              [18]
```

* **Si `num = 7`** devuelve `true` ya que en su rama izquierda hay 1 árbol con un único hijo (el árbol con raíz 23) y en la rama derecha hay 0. (`1 > 0`) → `true`.
* **Si `num = 2`** devuelve `false`, ya que en su rama izquierda hay 1 árbol con único hijo (árbol con raíz 23) y en la rama derecha hay 3 (árboles con raíces -5, 19 y 4). (`1 > 3`) → `false`.
* **Si `num = -5`** devuelve `true`, ya que en su rama izquierda hay 2 árboles con único hijo (árboles con raíces 19 y 4) y al no tener rama derecha, tiene -1 árboles con un único hijo. (`2 > -1`) → `true`.
* **Si `num = 19`** debería devolver `false`, ya que al no tener rama izquierda tiene -1 árboles con un único hijo y en su rama derecha hay 1 árbol con único hijo. (`-1 > 1`) → `false`.
* **Si `num = -3`** debería devolver `false`, ya que al no tener rama izquierda tiene -1 árboles con un único hijo y lo mismo sucede con su rama derecha. (`-1 > -1`) → `false`.

---

## Ejercicio 8

Escribir en una clase `ParcialArboles` el método público con la siguiente firma:

`public boolean esPrefijo(BinaryTree<Integer> arbol1, BinaryTree<Integer> arbol2)`

El método devuelve `true` si `arbol1` es prefijo de `arbol2`, `false` en caso contrario.

Se dice que un árbol binario `arbol1` es prefijo de otro árbol binario `arbol2`, cuando `arbol1` coincide con la parte inicial del árbol `arbol2` tanto en el contenido de los elementos como en su estructura.

**Ejemplo 1 (Es prefijo)**:

```text
arbol1:
          [65]
         /   \
       [37] [81]
           \    \
          [47] [93]

arbol2:
          [65]
         /   \
       [37] [81]
      /   \  /   \
    [22] [47][76][93]
    /   \       /   \
  [11] [29]   [85] [94]
```

**Ejemplo 2 (NO es prefijo)**:
*El subárbol con raíz 93 no está en el árbol2*

```text
arbol1:
          [65]
         /   \
       [37] [81]
           \    \
          [47] [93]

arbol2:
          [65]
         /   \
       [37] [81]
      /   \    \
    [22] [47] [76]
    /   \
  [11] [29]
```

**Ejemplo 3 (NO es prefijo)**:
*No coincide el contenido. El subárbol con raíz 37 figura con raíz 62.*

```text
arbol1:
          [65]
         /   \
       [37] [81]
           \    \
          [47] [93]

arbol2:
          [65]
         /   \
       [62] [81]
      /   \  /   \
    [22] [47][76][93]
    /   \       /   \
  [11] [29]   [85] [94]
```

---

## Ejercicio 9

Escribir en una clase `ParcialArboles` el método público con la siguiente firma:

`public BinaryTree<?> sumAndDif(BinaryTree<Integer> arbol)`

El método recibe un árbol binario de enteros y devuelve un nuevo árbol que contenga en cada nodo dos tipos de información:

* La suma de los números a lo largo del camino desde la raíz hasta el nodo actual.
* La diferencia entre el número almacenado en el nodo original y el número almacenado en el nodo padre.

**Nota**: En el nodo raíz considere que el valor del nodo padre es 0.

**Ejemplo**:

```text
Árbol Original:
                   [20]
                  /    \
                [5]    [30]
               /  \    /   \
             [-5] [10][50] [-9]
                  /    /     \
                [1]  [4]     [6]

Nuevo árbol (Suma | Diferencia):
                 [20|20]
                /       \
             [25|-15] [50|10]
            /    \    /    \
        [20|-10][35|5][100|20][41|-39]
            /      /    \       \
        [36|-9] [110|2] [104|-46]
```
