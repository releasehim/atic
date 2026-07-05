# Ejercicio 7: Uso de las estructuras de listas provistas por la API de Java

### Respuestas Teóricas

#### b. Si se hubiera usado `LinkedList` en lugar de `ArrayList` en el inciso 7.a, ¿qué diferencias se encontrarían respecto de la implementación?
Respecto de la **implementación del código escrito**, no habría prácticamente ninguna diferencia visible aparte de la inicialización de la lista (`List<Integer> lista = new LinkedList<>();` en lugar de `new ArrayList<>();`).
*   Esto se debe a que ambas clases implementan la interfaz común `List<E>`, compartiendo firmas de métodos como `add(E e)` e interfaces de recorrido (ya sea por bucle for-each o utilizando un `Iterator`).
*   **A nivel interno y de rendimiento**, la diferencia sería que `ArrayList` almacena los datos de forma contigua en un arreglo interno, mientras que `LinkedList` crea nodos dispersos en memoria enlazados mediante referencias.

---

#### c. ¿Existen otras alternativas para recorrer los elementos de la lista del punto 7.a?
Sí, existen las siguientes alternativas para recorrer una lista:
1.  **Bucle for-each (for extendido):**
    ```java
    for (Integer num : lista) {
        System.out.println(num);
    }
    ```
2.  **Iterador explícito (`Iterator`):**
    ```java
    Iterator<Integer> it = lista.iterator();
    while (it.hasNext()) {
        System.out.println(it.next());
    }
    ```
3.  **Bucle for clásico con índices (solo recomendado para `ArrayList`):**
    ```java
    for (int i = 0; i < lista.size(); i++) {
        System.out.println(lista.get(i));
    }
    ```
    *Nota: Usar el bucle for clásico por índices en una `LinkedList` es ineficiente ya que `get(i)` tiene complejidad O(i), lo que eleva el recorrido completo a O(N²).*

---

#### d.5. Modificación de datos de los estudiantes y conclusiones
Al copiar la lista usando construcciones del estilo `new ArrayList<>(listaOriginal)` y luego modificar un dato del estudiante (ej: cambiar el nombre con un `setter`), el cambio se ve reflejado **tanto en la lista original como en la nueva lista**.

**Conclusiones:**
*   Esto ocurre porque el constructor de copia de las colecciones de Java realiza una **copia superficial (shallow copy)**. 
*   La nueva lista contiene referencias que apuntan a los **mismos objetos `Estudiante`** que la lista original en el heap. No se duplican los objetos estudiantes en sí, sino las referencias a ellos.

---

#### d.6. ¿Cuántas formas de copiar una lista existen? ¿Qué diferencias existen entre ellas?
Existen principalmente dos niveles de copias de listas:

1.  **Copia Superficial (Shallow Copy):**
    *   **Formas de implementarla:** `new ArrayList<>(original)`, `original.addAll(...)`, o `Collections.copy()`.
    *   **Comportamiento:** Se crea una nueva lista (nueva estructura), pero los elementos dentro de ella siguen siendo referencias a los mismos objetos en memoria que la lista original. Si se modifica la lista agregando/eliminando elementos, la otra lista no se ve afectada, pero si se modifica el *estado interno* de los objetos contenidos, el cambio se observa en ambas listas.
2.  **Copia Profunda (Deep Copy):**
    *   **Formas de implementarla:** Iterar sobre la lista original, crear un nuevo objeto del mismo tipo (clon) copiando campo por campo sus atributos (`new Estudiante(...)`), y agregar ese nuevo objeto a la nueva lista.
    *   **Comportamiento:** Se duplican tanto la estructura de la lista como los objetos individuales contenidos en ella. Las modificaciones realizadas a los objetos de la nueva lista no afectan en absoluto a los de la lista original.
