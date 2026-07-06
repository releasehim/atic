# Ejercicio 11: Modelado de paradas de una línea de colectivos

### Respuesta Teórica

De acuerdo con las estructuras de datos vistas en esta práctica, la estructura ideal para modelar las paradas de una línea de colectivos es una **Cola Circular (`CircularQueue`)**.

#### Justificación del diseño

* **Comportamiento cíclico y repetitivo:** Un colectivo realiza un recorrido fijo compuesto por una secuencia ordenada de paradas (Parada 1 $\rightarrow$ Parada 2 $\rightarrow$ ... $\rightarrow$ Parada N). Una vez que llega a la última parada (Terminal o final del recorrido), el ciclo vuelve a comenzar en la Parada 1.
* **Aprovechamiento de la operación `shift()`:** La clase `CircularQueue` implementa el método `shift()`, el cual desencola el elemento del frente (parada actual) y lo encola al final. Esto representa de forma directa y natural el avance del colectivo:
    1. El colectivo está en la parada que se encuentra al frente de la cola (`head()`).
    2. Cuando el colectivo avanza a la siguiente parada, realizamos un `shift()`. Esto desplaza la parada visitada al final del recorrido y posiciona la nueva parada al frente.
    3. Este ciclo puede continuar indefinidamente sin necesidad de reiniciar o recrear la estructura de datos, manteniendo siempre el orden correcto de las paradas.
* **Eficiencia:** El avance a la siguiente parada (`shift()`) se realiza en tiempo constante **O(1)**, ya que internamente utiliza `LinkedList` para reasignar referencias.
