# Análisis de Algoritmos y Tiempo de Ejecución

## Introducción: ¿Por qué analizar algoritmos?

El análisis de algoritmos nos permite comparar la eficiencia de diferentes soluciones para un mismo problema de forma independiente de la plataforma (hardware, lenguaje). El objetivo es medir cómo crece el **tiempo de ejecución ($T(n)$)** a medida que aumenta el **tamaño de la entrada ($n$)**. Este análisis se realiza típicamente para el **peor caso**.

---

## Parte 1: Herramientas Matemáticas Fundamentales

Para calcular $T(n)$ a partir del código, necesitamos traducir las estructuras de control (bucles, recursión) a expresiones matemáticas. La herramienta principal es la **Notación Sigma ($\Sigma$)**.

### 1.1 La Notación Sigma ($\Sigma$)

La sumatoria es la forma abreviada de representar la suma de una serie de números que siguen un patrón.

* **Sintaxis:** $\sum_{i = inicio}^{fin} f(i)$
* **Componentes:**
  * $i$: **Índice de suma** (variable que se incrementa).
  * $inicio$: **Índice inicial** (límite inferior).
  * $fin$: **Índice final** (límite superior).
  * $f(i)$: **Función que depende del índice** (cuerpo de la sumatoria).

```text
Diagrama de una Sumatoria (Notación Sigma):
        Índice final            Función que depende del índice
              |                           |
              v                           v
              n
             ____
             \        f(i)
             /____
              i = 1
                ^   ^
           Índice de suma   Índice inicial
```

### 1.2 Propiedades Básicas de las Sumatorias

Cuando el cuerpo de la sumatoria es **independiente del índice** (es una constante $C$), la sumatoria es simplemente la constante multiplicada por la cantidad de términos.

* **Suma de una constante (Comenzando en 1):** $\sum_{i=1}^{n} C = C + C + \dots + C = n \times C$
* **Suma de una constante (Comenzando en $j$):** $\sum_{i=j}^{n} C$. La cantidad de términos es $n - j + 1$, por lo tanto, $\sum_{i=j}^{n} C = (n - j + 1) \times C$.
* **Sacar factor común:** $\sum_{i=1}^{n} (C \times f(i)) = C \times \sum_{i=1}^{n} f(i)$.
* **Separar sumas:** $\sum_{i=1}^{n} (f(i) + g(i)) = \sum_{i=1}^{n} f(i) + \sum_{i=1}^{n} g(i)$.

**El truco para sumatorias que no empiezan en 1:**
Si tenemos $\sum_{i=j}^{n} f(i)$, podemos "cortar" la sumatoria extendiéndola desde 1 y restando lo que sobra.
$$ \sum_{i=j}^{n} f(i) = \sum_{i=1}^{n} f(i) - \sum_{i=1}^{j-1} f(i) $$
Esto es útil porque las fórmulas de sumatorias se definen fácilmente para empezar en 1.

### 1.3 Identidades Útiles para el Análisis de Algoritmos

Las siguientes fórmulas son fundamentales para resolver las sumatorias que aparecen en el análisis:

| Nombre | Fórmula |
| :--- | :--- |
| **Suma de los primeros $n$ naturales** | $\sum_{i=1}^{n} i = \frac{n(n+1)}{2}$ |
| **Suma de los primeros $n$ cuadrados** | $\sum_{i=1}^{n} i^2 = \frac{n(n+1)(2n+1)}{6}$ |
| **Serie Geométrica (Suma de potencias de $k$)** | $\sum_{i=0}^{n} k^i = \frac{k^{n+1} - 1}{k - 1}$ |

---

## Parte 2: Análisis de Algoritmos Iterativos ($T(n)$)

Al analizar algoritmos iterativos, debemos identificar los bucles y traducir su flujo a sumatorias. Las operaciones simples dentro de los bucles se tratan como constantes ($c$).

### 2.1 Bucles Simples (`for` y `while`)

* **Bucle `for` básico:** `for (i = 1; i <= n; i++) { C }`
  * $T(n) = \sum_{i=1}^{n} C = n \times C \Rightarrow O(n)$
* **Bucle `while` básico:** `int x = 1; while (x < n) { x = 2 * x; }`
  * La variable crece exponencialmente ($1, 2, 4, 8, \dots$). La cantidad de iteraciones es logarítmica. $T(n) = \log_2(n) \times C \Rightarrow O(\log n)$.

### 2.2 Bucles Anidados y Sumatorias Anidadas

Cuando tenemos un bucle dentro de otro, el tiempo total es la suma del tiempo del bucle interior por cada iteración del exterior. Se traduce a una sumatoria doble.

**Ejemplo:**

```java
for (int i = 1; i <= n; i++) {
    for (int j = 1; j <= n; j++) {
        // operación constante C
    }
}
```

$$ T(n) = \sum_{i=1}^{n} \left( \sum_{j=1}^{n} C \right) = \sum_{i=1}^{n} (n \times C) = n \times n \times C = n^2 C \Rightarrow O(n^2) $$

### 2.3 Ejemplo Avanzado: Análisis Detallado Paso a Paso

Consideremos el siguiente código:

```java
public static void ejercicio2(int n) {
    int i, j;
    for (i = 1; i <= n; i++) {
        j = i;
        while (j <= Math.pow(n, 3)) {
            // operación constante c3
            j++;
        }
    }
}
```

**Paso 1: Plantear el $T(n)$**
El bucle `for` externo se ejecuta $n$ veces. Dentro de él, el `while` se ejecuta desde $j=i$ hasta $n^3$. El costo total es:
$$ T(n) = c_1 + \sum_{i=1}^{n} \left[ c_2 + \sum_{j=i}^{n^3} c_3 \right] $$
(Donde $c_1$ es la inicialización, $c_2$ la operación del for y el `println`, y $c_3$ el incremento del while).

**Paso 2: Resolver la sumatoria interna**
La sumatoria interna $\sum_{j=i}^{n^3} c_3$ tiene como límite superior $n^3$. Aplicando el truco de cortar la sumatoria:
$$ \sum_{j=i}^{n^3} c_3 = c_3 \cdot (n^3 - i + 1) $$

**Paso 3: Sustituir y reescribir la sumatoria externa**
$$ T(n) = c_1 + \sum_{i=1}^{n} \left[ c_2 + c_3 \cdot (n^3 - i + 1) \right] $$
$$ T(n) = c_1 + \sum_{i=1}^{n} \left[ c_2 + c_3 n^3 - c_3 i + c_3 \right] $$

**Paso 4: Aplicar distributiva y resolver**
$$ T(n) = c_1 + \sum_{i=1}^{n} c_2 + \sum_{i=1}^{n} c_3 n^3 - \sum_{i=1}^{n} c_3 i + \sum_{i=1}^{n} c_3 $$
Aplicamos la fórmula de la suma de los primeros $n$ naturales ($ \sum i = \frac{n(n+1)}{2} $):
$$ T(n) = c_1 + n c_2 + n c_3 n^3 - c_3 \frac{n(n+1)}{2} + n c_3 $$
$$ T(n) = c_1 + c_2 n + c_3 n^4 - c_3 \frac{n^2}{2} - c_3 \frac{n}{2} + c_3 n $$
**¡Regla de oro!** Al finalizar, verifica que la única variable sea $n$. **No deben quedar variables de los iteradores (como $i$ o $j$)**. El término dominante es $c_3 n^4$, por lo que $T(n) = O(n^4)$.

---

## Parte 3: Análisis de Algoritmos Recursivos ($T(n)$)

Para los algoritmos recursivos, no podemos escribir sumatorias directamente. En su lugar, definimos una **Función de Recurrencia**. Esta función expresa el tiempo $T(n)$ en términos del tiempo de una entrada más pequeña $T(n-1)$ o $T(n/2)$.

### 3.1 Ejemplos Básicos

* **Factorial (Recursivo):**

    ```java
    public static int factorial(int n) {
        if (n == 1) return 1;
        else return n * factorial(n - 1);
    }
    ```

  * **Recurrencia:** $T(n) = \begin{cases} cte_1 & \text{si } n = 1 \\ cte_2 + T(n-1) & \text{si } n > 1 \end{cases}$
  * **Solución:** $T(n) = T(n-1) + cte_2 = (T(n-2) + cte_2) + cte_2 = \dots = cte_1 + (n-1)cte_2 \Rightarrow O(n)$.

* **Fibonacci (Recursivo):**

    ```java
    public static int fib(int n) {
        if (n <= 1) return 1;
        else return fib(n-1) + fib(n-2);
    }
    ```

  * **Observación:** Este algoritmo es terriblemente ineficiente ($O(2^n)$) porque recalcula los mismos valores múltiples veces. La versión iterativa es $O(n)$.

### 3.2 Resolución de Recurrencias: Método de Desarrollo (Unrolling)

El método más común para resolver recurrencias es "desarrollar" la ecuación hasta encontrar un patrón y llegar al caso base.

**Ejemplo (Del material de TP04):**
$$ T(n) = \begin{cases} 3 & \text{si } n = 1 \\ 27 T(n/3) + n^3 & \text{si } n \ge 2 \end{cases} $$

**Paso 1: Desarrollo iterativo (Unrolling)**

* **Paso 1:** $T(n) = 27 T\left(\frac{n}{3}\right) + n^3$
* **Paso 2:** Reemplazamos $T(n/3)$:
    $T(n) = 27 \left[ 27 T\left(\frac{n}{9}\right) + \left(\frac{n}{3}\right)^3 \right] + n^3$
    $T(n) = 27^2 T\left(\frac{n}{3^2}\right) + 27 \cdot \frac{n^3}{27} + n^3$
    **Simplificación:** El término $27 \cdot \frac{n^3}{27}$ se cancela (porque $3^3 = 27$), quedando:
    $T(n) = 27^2 T\left(\frac{n}{3^2}\right) + 2 n^3$
* **Paso 3:**
    $T(n) = 27^3 T\left(\frac{n}{3^3}\right) + 3 n^3$

**Paso 2: Paso General (Paso $i$)**
Observando el patrón, para cualquier paso $i$:
$$ T(n) = 27^i T\left(\frac{n}{3^i}\right) + i \cdot n^3 $$

**Paso 3: Caso Base**
El desarrollo termina cuando el argumento de $T$ llega al caso base, es decir, cuando:
$$ \frac{n}{3^i} = 1 $$
Despejando $i$:
$$ n = 3^i \Rightarrow i = \log_3(n) $$

**Paso 4: Sustitución Final**
Reemplazamos $i$ en el paso general:
$$ T(n) = 27^{\log_3(n)} T(1) + \log_3(n) \cdot n^3 $$
Sabemos que $T(1) = 3$. Ahora, simplificamos $27^{\log_3(n)}$. Usando propiedades de logaritmos y potencias:

* Forma 1: $27 = 3^3$, entonces $27^{\log_3(n)} = (3^3)^{\log_3(n)} = 3^{3 \log_3(n)} = (3^{\log_3(n)})^3 = n^3$.
* Forma 2: $27^{\log_3(n)} = n^3$ (directamente por propiedad $a^{\log_b(c)} = c^{\log_b(a)}$).

**Paso 5: Resultado Final del Tiempo de Ejecución**
$$ T(n) = n^3 \cdot 3 + \log_3(n) \cdot n^3 $$
$$ T(n) = 3 \cdot n^3 + n^3 \log_3(n) $$
Por lo tanto, el orden de ejecución es:
$$ T(n) = O(n^3 \log(n)) $$

---

## Parte 4: Notación Asintótica (Big-Oh / $O$)

La notación Big-Oh describe la cota superior del tiempo de ejecución. No nos interesa el tiempo exacto, sino cómo crece la función a medida que $n$ se hace muy grande.

### 4.1 Definición Formal

Decimos que $T(n) = O(f(n))$ si existen constantes $c > 0$ y $n_0 > 0$ tales que:
$$ T(n) \le c \cdot f(n) \quad \text{para todo } n \ge n_0 $$

```text
Interpretación Gráfica de la Definición:
        ^ T(n), c*f(n)
        |
        |              c*f(n) (Cota superior)
        |             /
        |            /  T(n) (Función real)
        |           /  /
        |          /  /
        |_________/__/_________________> n
                  ^
                  |
                 n0
A partir del punto n0, la función c*f(n) siempre está POR ENCIMA de T(n).
```

### 4.2 Cómo Demostrar el Orden (Guía Paso a Paso)

Para demostrar que $T(n) = O(f(n))$, se sigue la estrategia de acotar cada término por separado y luego sumar las cotas.

**Ejemplo:** Demostrar que $T(n) = 5n + 3n^2 + 2n^2 \log_2(n)$ es $O(n^2 \log_2(n))$.

**Paso 1: Acotar el primer término ($5n$)**
Necesitamos $5n \le c_1 n^2 \log_2(n)$.

* Como $n \le n^2 \log_2(n)$ para $n \ge 2$ (porque $\log_2(2) = 1$, y $2 \le 4\cdot1$), multiplicamos por 5:
* $5n \le 5 n^2 \log_2(n)$. Entonces $c_1 = 5$ y $n_0 = 2$.

**Paso 2: Acotar el segundo término ($3n^2$)**
Necesitamos $3n^2 \le c_2 n^2 \log_2(n)$.

* Como $1 \le \log_2(n)$ para $n \ge 2$ (porque $\log_2(2) = 1$):
* $3n^2 \le 3 n^2 \log_2(n)$. Entonces $c_2 = 3$ y $n_0 = 2$.

**Paso 3: Acotar el tercer término ($2n^2 \log_2(n)$)**
Necesitamos $2n^2 \log_2(n) \le c_3 n^2 \log_2(n)$.

* Directamente podemos elegir $c_3 = 2$ y $n_0 = 1$ (o $n_0 = 2$).

**Paso 4: Sumar las cotas y encontrar la constante global $c$**
Sumamos las desigualdades:
$$ 5n + 3n^2 + 2n^2 \log_2(n) \le 5 n^2 \log_2(n) + 3 n^2 \log_2(n) + 2 n^2 \log_2(n) $$
$$ T(n) \le (5 + 3 + 2) n^2 \log_2(n) $$
$$ T(n) \le 10 n^2 \log_2(n) $$

**Paso 5: Elegir el $n_0$ más restrictivo**
El más restrictivo es $n_0 = 2$.
**Conclusión:** $T(n) = O(n^2 \log_2(n))$ con $c = 10$ y $n_0 = 2$.

### 4.3 Orden de Crecimiento de Funciones (Tabla)

Es crucial identificar qué función crece más rápido para elegir $f(n)$ correctamente.

| Orden | Nombre | Ejemplo |
| :--- | :--- | :--- |
| $O(1)$ | Constante | Acceso a un array por índice. |
| $O(\log n)$ | Logarítmico | Búsqueda binaria (dividir y conquistar). |
| $O(n)$ | Lineal | Recorrer un array una vez. |
| $O(n \log n)$ | N-Log-N | Ordenamiento eficiente (Merge Sort, Heap Sort). |
| $O(n^2)$ | Cuadrático | Recorrer una matriz cuadrada (bucles anidados). |
| $O(n^3)$ | Cúbico | Recorrer una matriz tridimensional. |
| $O(2^n)$ | Exponencial | Recursión de Fibonacci. |

---

## Parte 5: Optimización de Algoritmos (Ejemplo del Video de Permutaciones)

El video muestra un ejemplo práctico de cómo el análisis de tiempo de ejecución puede ayudar a optimizar un algoritmo que genera permutaciones aleatorias.

**Problema:** Generar un arreglo con los números $\{0, 1, \dots, n-1\}$ en orden aleatorio (una permutación).

* **Algoritmo 1 ($O(n^3)$):**
  * Usa un bucle externo $n$ veces.
  * Genera un número aleatorio $x$.
  * **Bucle interno:** Recorre todo el arreglo para verificar si $x$ ya fue usado.
  * Si está usado, repite el proceso (bucle `while`).
  * *Análisis:* El peor caso es que la función `random()` siempre devuelva números repetidos, causando un bucle infinito (¡mal!). El caso esperado es $O(n^3)$ por los tres niveles anidados ($for \times while \times for$).

* **Algoritmo 2 ($O(n^2)$):**
  * Usa un bucle externo $n$ veces.
  * Genera un número aleatorio $x$.
  * Verifica si $x$ está usado consultando un **array booleano auxiliar** ($O(1)$).
  * Si está usado, repite el proceso (bucle `while`).
  * *Análisis:* Se elimina el bucle de verificación interna. Sigue teniendo el riesgo teórico de bucle infinito, pero su complejidad esperada baja a $O(n^2)$.

* **Algoritmo 3 ($O(n)$) - Solución Óptima:**
  * **Paso 1:** Crea el arreglo ordenado: `{0, 1, 2, ..., n-1}` (esto es una permutación válida).
  * **Paso 2:** Recorre el arreglo una vez (bucle $n$ veces).
  * En la posición `i`, genera un índice aleatorio `r` entre `0` y `n-1`.
  * **Intercambia** el valor en `a[i]` con el valor en `a[r]`.
  * *Análisis:* No depende de la suerte de `random()`. El bucle siempre se ejecuta $n$ veces. **Complejidad óptima: $O(n)$**.

---

## Conclusión y Resumen

Calcular el tiempo de ejecución de un algoritmo es un proceso metódico:

1. **Escribir la sumatoria (iterativo)** o **la recurrencia (recursivo)** a partir del código.
2. **Resolver las sumatorias** usando las propiedades matemáticas (suma de naturales, cortar sumatorias, sacar factor común).
3. **Verificar** que no queden variables de los iteradores en la función final $T(n)$.
4. **Identificar el término dominante** de $T(n)$ para proponer la cota $f(n)$.
5. **Demostrar** que $T(n) = O(f(n))$ encontrando las constantes $c$ y $n_0$.
6. **Optimizar:** Elegir el algoritmo con el menor Big-Oh posible siempre que sea práctico y no caiga en riesgos de bucles infinitos.
