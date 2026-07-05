# Ejercicio 4: Pasaje de parámetros en Java

### Respuestas Teóricas

#### a. ¿Qué imprime el código analizado sin ejecutarlo?
El código imprimirá:
```text
a=1 b=2
c=3 d=4
```

**Justificación:**
1. **Pasaje por valor de tipos primitivos (`swap1`):** 
   En Java, el pasaje de parámetros es siempre **por valor**. Al invocar `swap1(a, b)`, se pasan copias de los valores de `a` (1) y `b` (2) a los parámetros locales `x` e `y`. Los cambios realizados en `x` e `y` dentro de `swap1` solo afectan a esas variables locales del método, por lo que `a` y `b` conservan sus valores originales en el `main`.
   
2. **Pasaje por valor de referencias e inmutabilidad (`swap2`):**
   Al invocar `swap2(c, d)` con objetos `Integer`, lo que se pasa es una copia de las referencias que apuntan a los objetos `Integer(3)` e `Integer(4)` en el heap. 
   Dentro del método:
   - `x = y;` hace que la referencia local `x` ahora apunte al mismo objeto que `y` (`Integer(4)`).
   - `y = tmp;` hace que la referencia local `y` apunte a un nuevo objeto `Integer(3)` (creado por autoboxing a partir del `int tmp`).
   Como solo se modificaron las copias locales de las referencias (`x` e `y`), y no los objetos a los que apuntaban ni las variables originales `c` y `d` del `main`, el estado en `main` no se ve afectado. Además, los objetos de tipo `Integer` son **inmutables**, por lo que no es posible modificar su valor interno una vez creados.

---

#### b. Comparación de resultados
Al ejecutar el programa en la computadora (implementado en [Ejercicio4.java](file:///workspaces/atic/2/ayed/practica/ayed/src/tp1/ejercicio4/Ejercicio4.java)), se obtiene exactamente la salida prevista:
```text
a=1 b=2
c=3 d=4
```

---

#### c. Análisis en modo Debug (breakpoints en `y = tmp`)
*   **En `swap1`:** Al detener la ejecución justo en la línea `y = tmp` (o inmediatamente después), las variables locales `x` e `y` del método tienen los valores `x = 2` e `y = 1`. Estos valores están intercambiados. Sin embargo, no coinciden con lo que se imprime finalmente en la consola para `a` y `b` en el método `main`, porque las variables del `main` nunca fueron modificadas.
*   **En `swap2`:** Al detener la ejecución tras asignarse `y = tmp`, la variable local `x` apunta al objeto `Integer(4)` y `y` apunta al objeto `Integer(3)`. Los valores referenciados por las variables locales fueron intercambiados en el ámbito de `swap2`, pero en el `main` las referencias `c` y `d` siguen apuntando a sus objetos originales (`Integer(3)` e `Integer(4)` respectivamente).
