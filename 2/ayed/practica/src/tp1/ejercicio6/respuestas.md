# Ejercicio 6: Análisis de las estructuras de listas provistas por la API de Java

### Respuestas Teóricas

#### a. ¿En qué casos `ArrayList` ofrece un mejor rendimiento que `LinkedList`?

`ArrayList` ofrece un rendimiento significativamente mejor en el **acceso aleatorio** (recuperación de elementos por índice mediante el método `get(int index)`).

* **Complejidad temporal:** En `ArrayList`, el acceso es **O(1)** (tiempo constante) porque los datos se almacenan en un arreglo contiguo en memoria, lo que permite calcular la dirección de memoria directamente mediante el índice.
* En `LinkedList`, en cambio, es **O(n)** porque debe recorrer los nodos de la lista uno a uno desde el inicio (o el final) hasta llegar a la posición deseada.
* También es más eficiente al agregar elementos al final de la lista (`add(e)`), ya que solo implica asignarlo en la siguiente posición libre del arreglo interno (en promedio **O(1)** amortizado), mientras que `LinkedList` requiere instanciar un nuevo objeto `Node` en el heap.

---

#### b. ¿Cuándo `LinkedList` puede ser más eficiente que `ArrayList`?

`LinkedList` es más eficiente cuando se realizan **inserciones o eliminaciones al principio** o en posiciones intermedias (si ya se tiene una referencia al nodo/posición).

* **Complejidad temporal:** Insertar o eliminar al inicio de una `LinkedList` es **O(1)**, pues solo requiere reasignar los punteros de los nodos.
* En `ArrayList`, estas operaciones toman **O(n)** de tiempo porque requiere desplazar todos los elementos subsecuentes del arreglo interno una posición hacia la derecha (para inserciones) o hacia la izquierda (para eliminaciones).
* También es más eficiente para evitar costosas operaciones de **redimensionamiento** (resize). Cuando un `ArrayList` se llena, debe crear un arreglo nuevo más grande y copiar todos los elementos del arreglo viejo al nuevo. `LinkedList` nunca se redimensiona; simplemente agrega nodos dinámicamente según sea necesario.

---

#### c. ¿Qué diferencia encuentra en el uso de la memoria en `ArrayList` y `LinkedList`?

* **`ArrayList`:** Tiene menor consumo de memoria por elemento. Utiliza un único objeto de tipo arreglo que guarda referencias directas a los elementos. La única sobrecarga de memoria ocurre cuando hay espacio sin usar reservado al final del arreglo debido a su capacidad inicial o redimensionamientos anteriores.
* **`LinkedList`:** Tiene mayor consumo de memoria por elemento. Cada elemento se envuelve en un objeto `Node` que además del dato almacena dos referencias adicionales (`next` y `prev`). Por lo tanto, requiere guardar tres referencias por cada elemento almacenado en el heap, lo que incrementa sustancialmente el consumo total de memoria.

---

#### d. ¿En qué casos sería preferible usar un `ArrayList` o un `LinkedList`?

* **Preferir `ArrayList` si:**
  * La operación predominante es la lectura de datos por índice (`get()`).
  * Los elementos se agregan mayormente al final.
  * La cantidad de elementos es relativamente estable o conocida de antemano (para inicializar el tamaño del arreglo y evitar redimensionamientos).
  * Se desea optimizar el uso de la memoria.
* **Preferir `LinkedList` si:**
  * Se realizan constantes inserciones y eliminaciones al principio o al final (por ejemplo, para implementar colas o pilas).
  * No se utiliza el acceso por índice de forma frecuente.
  * El tamaño de la lista varía constantemente de forma muy impredecible y no se quiere pagar el costo del redimensionamiento del arreglo.
