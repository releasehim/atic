# Ejercicio 6: Transformación de Árbol Binario

### Respuestas Teóricas

#### ¿Su solución recorre una única vez cada subárbol?
**Sí, la solución propuesta recorre exactamente una única vez cada subárbol (complejidad temporal $O(N)$).**

**Justificación del diseño:**
*   Para calcular la transformación de un nodo $N$, necesitamos conocer la suma acumulada de los elementos en sus subárboles izquierdo y derecho.
*   En lugar de implementar un método separado para sumar elementos (lo cual causaría múltiples visitas redundantes sobre los mismos nodos, resultando en una complejidad de $O(N^2)$), combinamos la **construcción del nuevo árbol** y la **suma acumulativa de los valores originales** en una única llamada recursiva de tipo *bottom-up* (postorden).
*   El método auxiliar `transform` recibe la referencia del nodo original y del nodo nuevo. Éste devuelve la suma del subárbol original completo y, al mismo tiempo, crea y enlaza los hijos del nuevo árbol y setea el valor acumulado en el nodo nuevo:
    ```java
    nuevo.setData(sumIzq + sumDer);
    return original.getData() + sumIzq + sumDer;
    ```
*   De esta manera, cada nodo original se visita exactamente una única vez y el nuevo árbol se construye y puebla simultáneamente de forma óptima.
