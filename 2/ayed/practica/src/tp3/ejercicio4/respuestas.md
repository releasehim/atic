# Ejercicio 4: Analizador de Mensajes de la Empresa

### Respuestas Teóricas

#### a. Justificación del recorrido

Para resolver este problema, la mejor estrategia es utilizar un **recorrido por niveles (BFS)**.

**Justificación:**

* El enunciado nos pide encontrar el mayor promedio entre los **promedios de los niveles** del árbol.
* Un recorrido por niveles procesa todos los nodos de una misma profundidad juntos antes de pasar a la siguiente profundidad.
* Utilizando una cola, podemos delimitar claramente cuándo finaliza un nivel y comienza el siguiente (ya sea midiendo el tamaño de la cola al inicio de cada ciclo o usando sentinelas). Esto nos permite acumular la suma y el conteo de nodos por nivel para calcular su promedio de manera aislada y eficiente.
* Un recorrido en profundidad (DFS) requeriría mantener estructuras auxiliares adicionales (como listas o mapas indexados por profundidad) para ir acumulando los valores de los nodos a medida que se descubren, lo que incrementa la complejidad del algoritmo y el consumo de memoria extra.
