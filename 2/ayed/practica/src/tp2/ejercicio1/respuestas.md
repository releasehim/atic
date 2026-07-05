# Ejercicio 1: Análisis de la clase `BinaryTree<T>`

### Estructura de `BinaryTree<T>`
La clase [BinaryTree](file:///workspaces/atic/2/ayed/practica/src/tp2/BinaryTree.java) modela un árbol binario utilizando una representación enlazada de nodos donde cada nodo contiene:
*   Un dato genérico `data` de tipo `T`.
*   Una referencia `leftChild` al subárbol izquierdo (de tipo `BinaryTree<T>`).
*   Una referencia `rightChild` al subárbol derecho (de tipo `BinaryTree<T>`).

### Métodos del TDA
1.  **Constructores**:
    *   `BinaryTree()`: Crea un nodo nulo/vacío.
    *   `BinaryTree(T data)`: Crea un nodo con el dato especificado y sin hijos.
2.  **Modificadores**:
    *   `addLeftChild(BinaryTree<T>)` y `addRightChild(BinaryTree<T>)`: Conectan subárboles a las ramas izquierda o derecha del nodo actual.
    *   `removeLeftChild()` y `removeRightChild()`: Desconectan el respectivo hijo.
3.  **Consultas**:
    *   `getData()` / `setData(T)`: Acceso y mutación del valor del nodo.
    *   `getLeftChild()` / `getRightChild()`: Retornan el subárbol izquierdo o derecho. Lanzan una excepción en tiempo de ejecución si se intenta acceder a un hijo inexistente.
    *   `hasLeftChild()` / `hasRightChild()`: Indican si el nodo posee ramas hijas. Son fundamentales para prevenir excepciones de tipo `NullPointerException`.
    *   `isEmpty()`: Indica si el árbol carece de dato e hijos.
    *   `isLeaf()`: Retorna `true` si el nodo tiene dato pero carece de hijos.
4.  **Operaciones Recursivas e Iterativas**:
    *   `contarHojas()`: Cuenta de manera recursiva la cantidad de hojas en el subárbol.
    *   `espejo()`: Genera recursivamente una copia simétrica de la estructura del árbol original.
    *   `entreNiveles(n, m)`: Imprime por consola los elementos de los niveles comprendidos entre `n` y `m` inclusive, utilizando un recorrido por niveles con una cola.
