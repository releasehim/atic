# Ejercicio 9: ParcialArboles (sumAndDif)

### Respuestas Teóricas y Análisis del Ejemplo

El método `sumAndDif` realiza una transformación en la cual cada nodo del árbol resultante contiene dos valores calculados a partir de la estructura original:

1. **Suma:** La suma acumulada de los valores de los nodos a lo largo del camino desde la raíz hasta el nodo actual.
    $$\text{Suma}(N) = \text{Suma}(\text{Padre}) + N.data$$
2. **Diferencia:** La resta entre el valor del nodo actual y el de su padre (considerando $0$ si es la raíz).
    $$\text{Diferencia}(N) = N.data - \text{Padre}.data$$

---

### Nota sobre una inconsistencia (Typo) en el PDF de la Práctica

En el ejemplo de salida proporcionado en el enunciado del PDF, el nivel inferior muestra los siguientes nodos:
`[36|-9] [110|2] [104|-46]` (y no muestra el correspondiente a la hoja `6` que es hija de `-9`).

Al analizar matemáticamente los valores del ejemplo, se detecta un **error de imprenta (typo)** en el PDF de la cátedra por las siguientes razones:

1. **Hijo de 50 (Suma 100, Valor 50):**
    * Su hijo izquierdo es `4`.
    * **Suma:** $100 + 4 = 104$.
    * **Diferencia:** $4 - 50 = -46$.
    * Esto da correctamente el nodo `[104|-46]` presente en el nivel inferior.
2. **El nodo fantasma `[110|2]`:**
    * Si un nodo tiene Suma = $110$ bajo el padre $50$ (cuya Suma es $100$), su valor original debe ser $10$ ($110 - 100 = 10$).
    * Si su valor original es $10$ y su padre es $50$, su Diferencia debe ser $10 - 50 = -40$, y no $2$.
    * Si la Diferencia es $2$ bajo el padre $50$, su valor original debe ser $52$ ($50 + 2 = 52$). Pero entonces su Suma sería $100 + 52 = 152$, y no $110$.
    * Por lo tanto, la dupla `[110|2]` es matemáticamente inconsistente y representa un error en el gráfico del enunciado.
3. **Hijo de -9 (Suma 41, Valor -9):**
    * Su hijo derecho es `6`.
    * **Suma:** $41 + 6 = 47$.
    * **Diferencia:** $6 - (-9) = 15$.
    * El resultado correcto es `[47|15]`, el cual nuestro código genera correctamente y que falta en el esquema ilustrativo del PDF.

Nuestra implementación en [Ejercicio9.java](file:///workspaces/atic/2/ayed/practica/src/tp2/ejercicio9/Ejercicio9.java) es 100% correcta y consistente con las reglas matemáticas del enunciado.
