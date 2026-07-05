# Ejercicio 10: Modelado de cola de atención bancaria con prioridad

### Respuesta Teórica

De acuerdo con las estructuras de datos vistas en esta práctica (`ArrayList`, `LinkedList`, `Queue`, `CircularQueue`, `DoubleEndedQueue` y `Stack`), la mejor propuesta para modelar la cola de atención en un banco con prioridades según la Ley 14564 es utilizar **dos colas FIFO estándar (`Queue` implementadas sobre `LinkedList`)**:

1.  **`colaPrioritaria`**: Para mujeres embarazadas, personas con movilidad reducida y mayores de 70 años.
2.  **`colaRegular`**: Para el resto de los clientes.

#### Justificación del diseño:
*   **Mantenimiento del orden de llegada (FIFO):** Es fundamental respetar el orden de llegada dentro de cada grupo. Si usáramos una única `DoubleEndedQueue` insertando a las personas con prioridad al inicio (`enqueueFirst`), estaríamos aplicando una política LIFO (*Last-In, First-Out*) para el grupo prioritario (el último prioritario en llegar sería atendido antes que los prioritarios que llegaron antes), lo cual es injusto e incorrecto.
*   **Algoritmo de atención simple:** Cuando un puesto de atención (cajero) se libera, el sistema consulta primero si `colaPrioritaria.isEmpty()` es falso. En caso afirmativo, desencola de la cola prioritaria. Si está vacía, desencola de `colaRegular`.
*   **Flexibilidad en políticas de atención:** Permite implementar fácilmente esquemas de atención mixtos (por ejemplo, atender a 2 personas de la cola prioritaria por cada 1 de la cola regular para evitar que la cola regular sufra de inanición/starvation extrema si llegan muchos clientes prioritarios seguidos).
*   **Eficiencia:** La inserción al final (`enqueue`) y eliminación al inicio (`dequeue`) sobre una cola implementada con `LinkedList` son operaciones de costo **O(1)**.
