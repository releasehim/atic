# Ejercicio 9: Balanceo de caracteres

### Respuestas Teóricas

#### a. ¿Qué estructura de datos utilizará para resolver este problema y cómo la utilizará?
Para resolver el problema del balanceo de caracteres, utilizaremos una **Pila (Stack)**. La política de acceso LIFO (*Last-In, First-Out*) de la pila es la ideal para este propósito, ya que el último carácter de apertura en ser leído debe ser el primero en cerrarse.

**Algoritmo de utilización:**
1.  Creamos una pila vacía para almacenar caracteres (`Character`).
2.  Iteramos a través de cada carácter del String de izquierda a derecha:
    *   Si el carácter es de **apertura** (`(`, `[`, `{`), lo apilamos (`push`).
    *   Si el carácter es de **cierre** (`)`, `]`, `}`):
        *   Si la pila está vacía, significa que hay un carácter de cierre sin un carácter de apertura correspondiente. Por lo tanto, el string **no está balanceado**.
        *   Si la pila no está vacía, desapilamos (`pop`) el elemento del tope de la pila.
        *   Comparamos el elemento desapilado con el carácter de cierre actual. Si no son del mismo tipo (por ejemplo, se lee `]` pero se desapiló `{`), el string **no está balanceado**.
3.  Al finalizar el recorrido de todo el String:
    *   Si la pila está vacía, significa que todos los caracteres de apertura tuvieron su correspondiente cierre en el orden correcto. El string **está balanceado**.
    *   Si la pila NO está vacía, significa que quedaron caracteres de apertura sin cerrar. El string **no está balanceado**.
