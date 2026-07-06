# Ejercicio 4: Red Binaria Llena (Retardo de Reenvío)

### Respuestas Teóricas

#### a. Estrategia a utilizar

Para resolver este problema, la mejor estrategia es realizar un **recorrido en profundidad (DFS)** recursivo.

**Justificación:**

* El retardo de reenvío total acumulado se define por caminos completos desde la raíz hasta las hojas.
* El recorrido en profundidad permite descender por cada rama hasta llegar a las hojas (los casos base), calculando los retardos parciales de cada camino de manera natural a través de la pila de llamadas del compilador.
* Una vez que obtenemos los retardos de los subárboles izquierdo y derecho, podemos subir en la recursión aplicando la fórmula:
    $$\text{retardo}(N) = N.data + \max(\text{retardo}(HijoIzq), \text{retardo}(HijoDer))$$
* Un recorrido por niveles no sería adecuado porque no asocia de forma directa la procedencia de los nodos por caminos individuales de ancestro a descendiente, dificultando la suma acumulativa por ramificación.
