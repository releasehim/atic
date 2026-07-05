# Clases 1 y 2 AyED

## Parte 1: Fundamentos de Java

### 1.1 La Plataforma Java y Compilación
Java es un lenguaje que compila a *bytecode* (archivos `.class`), el cual es interpretado por la Máquina Virtual de Java (JVM). Esto permite que el código sea independiente de la plataforma.
*   **Compilación**: `Programa.java` → `javac` (compilador) → `Programa.class` (bytecode).
*   **Ejecución**: `Programa.class` → `java` (intérprete JVM) → Ejecución en el sistema operativo.

### 1.2 Clases, Objetos y el Estado
*   **Clase**: Es un molde que define el estado (variables de instancia) y el comportamiento (métodos) de los objetos.
*   **Objeto**: Una entidad de software creada a partir de una clase (instanciación con `new`). Almacena su estado en la memoria **HEAP**.
*   **Variables de Instancia**: Declaradas dentro de la clase pero fuera de los métodos. Suelen ser `private` para encapsulamiento.

```text
Ejemplo de definición de clase y objeto en memoria:
[ Clase Contacto ]
├── private String nombre;
├── private String estado;
└── public void setNombre(String n) { this.nombre = n; }

[ Memoria STACK ]                  [ Memoria HEAP ]
[ ref contacto1 ] -----------------> [ Objeto Contacto ]
                                     ├── nombre = "Lucia"
                                     ├── estado = "feliz"
                                     └── id = 1
```
*   **Variables Locales**: Declaradas dentro de métodos o bloques. Son temporales y se destruyen al finalizar el método. **Deben ser inicializadas explícitamente** antes de usarse.
*   **Palabra clave `this`**: Referencia al objeto actual dentro de un método de instancia.

### 1.3 Constructores
Los constructores son métodos especiales que se ejecutan al crear un objeto (`new`) para inicializar su estado.
*   Deben tener el mismo nombre que la clase.
*   No tienen tipo de retorno.
*   **Constructor por defecto**: Si no se define ningún constructor, Java lo proporciona automáticamente sin argumentos y con cuerpo vacío.
*   **Sobrecarga de constructores**: Se pueden definir múltiples constructores con diferentes parámetros.

```java
public class Vehiculo {
    private String marca;
    private double precio;

    // Constructor sin argumentos
    public Vehiculo() { }

    // Constructor con argumentos
    public Vehiculo(String marca, double precio) {
        this.marca = marca;
        this.precio = precio;
    }
}
```

### 1.4 Tipos de Datos en Java
Java tiene dos grandes categorías de tipos:

1.  **Tipos Primitivos**: Almacenan valores simples y no son objetos.
    *   `byte`, `short`, `int`, `long` (enteros)
    *   `float`, `double` (coma flotante)
    *   `char` (carácter)
    *   `boolean` (verdadero/falso)
    *   Por defecto se inicializan en `0` o `false`.
2.  **Tipos Referenciales (Objetos)**: Almacenan una dirección de memoria (referencia) que apunta al objeto en el *heap*.
    *   Clases (`String`, `Contacto`, etc.), Interfaces, Arreglos.
    *   Por defecto se inicializan en `null`.

#### Clases Wrapper y Autoboxing/Unboxing
Java trata los tipos primitivos de forma eficiente, pero para ser usados en colecciones (como `List`), que solo aceptan objetos, se necesitan clases *wrapper*: `Integer`, `Double`, `Character`, `Boolean`.
*   **Autoboxing**: Conversión automática de primitivo a wrapper (`int` → `Integer`).
*   **Unboxing**: Conversión automática de wrapper a primitivo (`Integer` → `int`).
*   **Importante**: El uso excesivo de wrappers tiene impacto en el rendimiento (es más lento que usar primitivos), ya que se crean objetos en el heap.

### 1.5 La palabra clave `static`
Los métodos y variables `static` pertenecen a la **clase**, no a las instancias individuales.
*   **Variables de clase**: Son compartidas por todas las instancias (ej. un contador de instancias creadas).
*   **Métodos de clase**: No pueden acceder a variables de instancia `non-static`, solo a variables `static` o locales.
*   Se invocan usando el nombre de la clase (`Clase.metodoEstatico()`).

### 1.6 Arreglos (Arrays)
Un array es un objeto que contiene un número fijo de valores del mismo tipo.
*   **Índice**: Comienza en `0` y termina en `longitud - 1`.
*   **Propiedad**: `.length`.
*   **Declaración e inicialización**: `int[] numeros = new int[5];` o `int[] numeros = {1, 2, 3, 4, 5};`.

```text
Representación en memoria de un array de objetos:
[ Memoria STACK ]          [ Memoria HEAP ]
[ ref cliArray ] --------> [ Array (Longitud 3) ]
                            ├── [0] -> ref -> [ Objeto Cliente 1 ]
                            ├── [1] -> ref -> [ Objeto Cliente 2 ]
                            └── [2] -> ref -> [ Objeto Cliente 3 ]
```
*   **Recorrido**: `for` tradicional o `for-each` (`for (Tipo elemento : array)`).

### 1.7 Pasaje de Parámetros en Java
Concepto fundamental: **Java siempre pasa los parámetros por valor**.
1.  **Tipo Primitivo**: Se pasa una **copia** del valor. Si se modifica el parámetro dentro del método, la variable original no se ve afectada.
2.  **Tipo Referencial (Objetos)**: Se pasa una **copia de la referencia** al objeto. Esto permite modificar el **estado** del objeto referenciado (ya que la copia apunta al mismo objeto en el heap). Sin embargo, no se puede reasignar la variable original para que apunte a un nuevo objeto fuera del método.
3.  **Inmutabilidad**: Clases como `String` y los Wrappers (`Integer`, `Double`) son inmutables. Cualquier "modificación" en su valor crea un **nuevo objeto**, por lo que no se afecta el objeto original pasado al método.

```text
Pasaje por referencia (objeto mutable):
Método main: Contacto c = new Contacto("Lucia");
             cambiarNombre(c);
             System.out.println(c.getNombre()); // Imprime "Pilar"

Método cambiarNombre(Contacto param) {
    param.setNombre("Pilar"); // ¡Se modifica el objeto HEAP!
}

Pasaje por referencia (reasignación errónea):
Método main: Contacto c = new Contacto("Lucia");
             cambiarObjeto(c);
             System.out.println(c.getNombre()); // Sigue imprimiendo "Lucia"

Método cambiarObjeto(Contacto param) {
    param = new Contacto("Juan"); // param ahora apunta a OTRO objeto HEAP, pero no cambia 'c' en main
}
```

---

## Parte 2: Herencia, Polimorfismo y Clases Abstractas

### 2.1 Conceptos de Herencia (`extends`, `super`)
La herencia permite que una clase (Subclase) herede atributos y métodos de otra clase (Superclase), promoviendo la reutilización de código.
*   **Sintaxis**: `public class Camion extends Vehiculo { ... }`
*   **Palabra clave `super`**: Se usa para invocar al constructor o métodos de la superclase.
*   **Sobrescritura (Overriding)**: Una subclase puede redefinir un método heredado para proporcionar una implementación específica. Debe coincidir en nombre, tipo de retorno y parámetros.

```text
Jerarquía de Herencia (Ejemplo Vehículo):

       +-------------------+
       |     Vehiculo      |  <- Superclase
       +-------------------+
       | - marca: String   |
       | - precio: double  |
       +-------------------+
       | + detalles()      |
       | + getMarca()      |
       +-------------------+
             /|\
              |
    +---------+---------+
    |                   |
+-----------+      +------------+
|   Auto    |      |  Camion    |  <- Subclases
+-----------+      +------------+
| - tipo    |      | - cargaMax |
+-----------+      +------------+
| + detalles()      | + detalles() (sobrescrito)
+-----------+      +------------+
```

### 2.2 Upcasting y Downcasting
*   **Upcasting**: Tratar una referencia de subclase como si fuera de la superclase. Es **seguro** y automático, pero se pierde el acceso a métodos específicos de la subclase.
    *   `Vehiculo v = new Camion();`
*   **Downcasting**: Tratar una referencia de superclase como si fuera de la subclase. No es seguro y requiere un *cast* explícito.
    *   `Camion c = (Camion) v;`
    *   En tiempo de ejecución, si `v` no es realmente un `Camion`, lanzará `ClassCastException`.

### 2.3 La Clase `Object` y Sobrescritura de Métodos
Toda clase en Java hereda implícitamente de la clase `Object`. Es común sobrescribir dos métodos clave:
*   **`public boolean equals(Object obj)`**: Por defecto, compara referencias de memoria (`==`). Se sobrescribe para comparar el **contenido lógico** de los objetos.
*   **`public String toString()`**: Por defecto, devuelve `NombreClase@hashCode`. Se sobrescribe para devolver una representación legible del estado del objeto.

```java
public class Fecha {
    private int dia, mes, anio;

    @Override
    public boolean equals(Object o) {
        if (o == null || !(o instanceof Fecha)) return false;
        Fecha f = (Fecha) o;
        return this.dia == f.dia && this.mes == f.mes && this.anio == f.anio;
    }

    @Override
    public String toString() {
        return dia + "-" + mes + "-" + anio;
    }
}
```

### 2.4 Clases y Métodos Abstractos
*   **Clase Abstracta**: Clase que no se puede instanciar (`new FiguraGeometrica()` es ilegal). Se usa como molde base para otras clases.
*   **Método Abstracto**: Declaración de un método sin cuerpo (`public abstract void dibujar();`). Cualquier subclase *concreta* debe implementar este método.

```text
Diagrama de Clase Abstracta:

+----------------------------+
| <<abstract>>               |
|   FiguraGeometrica         |
+----------------------------+
| - color: Color             |
| - x, y: int                |
+----------------------------+
| + mover(int, int)          |  <-- Método concreto (compartido)
| + abstract getArea()       |  <-- Método abstracto
| + abstract dibujar()       |  <-- Método abstracto
+----------------------------+
            /|\
             |
    +--------+--------+
    |                 |
+-----------+  +-----------+
|  Circulo  |  | Rectangulo |  <-- Clases concretas implementan abstractos
+-----------+  +-----------+
```

---

## Parte 3: El Framework de Colecciones de Java (Listas)

### 3.1 El TDA Lista
Una **Lista** es una secuencia lineal de elementos que pueden manipularse libremente. Se puede agregar y eliminar elementos en cualquier posición.
**Operaciones clave**:
*   `add(e)`: Agrega al final.
*   `add(pos, e)`: Agrega en una posición.
*   `get(pos)`: Obtiene el elemento en `pos`.
*   `indexOf(e)`: Devuelve el índice de la primera ocurrencia de `e`.
*   `remove(pos)`: Elimina el elemento en `pos`.
*   `isEmpty()`: Verifica si está vacía.
*   `size()`: Cantidad de elementos.

### 3.2 Implementaciones Concretas en Java (`ArrayList` y `LinkedList`)
La API de Java (`java.util`) provee la interfaz `List<E>` y dos implementaciones principales:

| Característica | `ArrayList` | `LinkedList` |
| :--- | :--- | :--- |
| **Implementación Interna** | Arreglo de tamaño variable. | Lista doblemente enlazada (nodos). |
| **Acceso Aleatorio (`get`)** | **O(1)** (Directo por índice). | **O(n)** (Recorrido secuencial). |
| **Inserción/Eliminación** | **O(n)** en medio o principio (debe mover elementos). | **O(1)** al principio y final. **O(n)** en medio. |
| **Memoria** | Menos uso (datos contiguos). | Más uso (almacena referencias a nodos siguiente/anterior). |
| **Mejor Uso** | Muchas lecturas por índice y pocas inserciones/eliminaciones. | Muchas inserciones/eliminaciones al principio/final. |

**Buenas Prácticas**:
*   Declarar siempre con la interfaz `List<T>` para ganar flexibilidad: `List<Integer> numeros = new ArrayList<>();`.
*   Usar tipos genéricos (`List<String>`) para evitar `casting` y capturar errores en tiempo de compilación.

### 3.3 Genéricos y Seguridad de Tipos
Sin genéricos, las listas almacenan `Object`, forzando a realizar `casting` inseguro:
`List lista = new ArrayList(); lista.add("Hola"); String s = (String) lista.get(0); // OK, pero verboso y peligroso`.

Con **Genéricos**:
`List<String> lista = new ArrayList<>();`
*   **Error de compilación**: El compilador rechaza agregar `Integer` a la lista.
*   **No hace falta castear**: `String s = lista.get(0);`
*   **Restricción**: El tipo genérico **no puede ser un primitivo**. Se debe usar su clase Wrapper (`Integer`, no `int`).

### 3.4 Copia de Colecciones e Iteradores
**Métodos para copiar una lista**:
1.  Por constructor: `new ArrayList<>(listaOriginal)`
2.  `addAll()`: `nuevaLista.addAll(listaOriginal)`
3.  `clone()`: (Requiere castear a `ArrayList`, no muy recomendado).

**Iteradores**:
El patrón Iterator permite recorrer una estructura sin exponer sus detalles internos.
```java
List<Integer> lista = new ArrayList<>();
Iterator<Integer> it = lista.iterator();
while (it.hasNext()) {
    Integer num = it.next();
    System.out.println(num);
}
```
```text
Diagrama de desplazamiento de un Iterador:
[ D ] [ H ] [ R ] [ T ]  <- Elementos
  ^
Cursor inicia antes del primer elemento (iterator())

[ D ] [ H ] [ R ] [ T ]
  ^
 next() devuelve "D" y mueve cursor.

[ D ] [ H ] [ R ] [ T ]
      ^
 next() devuelve "H" y mueve cursor.
```

---

## Parte 4: Pilas y Colas (Estructuras Lineales con Herencia)

### 4.1 El TDA Pila (Stack) - LIFO
Una Pila sigue la política **LIFO** (*Last-In, First-Out*): el último elemento en entrar es el primero en salir.
**Operaciones**:
*   `push(e)`: Inserta en el tope.
*   `pop()`: Elimina y devuelve el tope.
*   `top()`: Devuelve el tope sin eliminarlo.
*   `isEmpty()`, `size()`.

**Implementación** (utilizando `ArrayList` internamente):
```java
public class Stack<T> {
    private List<T> data = new ArrayList<>();
    public void push(T dato) { data.add(data.size(), dato); }
    public T pop() { return data.remove(data.size()-1); }
    public T top() { return data.get(data.size()-1); }
}
```

#### Ejemplo Aplicado: Evaluación de Expresión Postfija (Video 3)
Dada la expresión `3 4 + 2 *`:

```text
Paso a paso en la Pila:
1. Leer '3'. push(3).  [ 3 ]
2. Leer '4'. push(4).  [ 3 | 4 ] (Tope a la derecha)
3. Leer '+'. a = pop() -> 4, b = pop() -> 3. Result = 3 + 4 = 7. push(7). [ 7 ]
4. Leer '2'. push(2).  [ 7 | 2 ]
5. Leer '*'. a = pop() -> 2, b = pop() -> 7. Result = 7 * 2 = 14. push(14). [ 14 ]
6. Resultado Final: pop() -> 14.
```

### 4.2 El TDA Cola (Queue) - FIFO
Una Cola sigue la política **FIFO** (*First-In, First-Out*): el primer elemento en entrar es el primero en salir.
**Operaciones**:
*   `enqueue(e)`: Inserta al final (rabo).
*   `dequeue()`: Elimina y devuelve el frente.
*   `head()`: Devuelve el frente sin eliminarlo.
*   `isEmpty()`, `size()`.

**Implementación** (utilizando `LinkedList`, ya que tiene referencias al principio y final, dando `O(1)` al encolar/desencolar):
```java
public class Queue<T> {
    private List<T> data = new LinkedList<>();
    public void enqueue(T dato) { data.add(dato); }
    public T dequeue() { return data.remove(0); }
}
```

#### Ejemplo Aplicado: Simulación de Caja de Supermercado (Video 3)
*   Cliente `[Nombre, Productos]`. Tiempo de atención = `productos * 2 segundos`.
*   Se encolan los clientes y se procesan en orden.
*   El método `simular(Queue<Cliente> cola)` itera (`while(!cola.isEmpty()) { ... cola.dequeue() ... }`) acumulando el tiempo total.

### 4.3 Relación de Herencia
En el diseño presentado en la teoría (PDF 2.2), se plantea una clase abstracta `Sequence` que define métodos comunes (`size()`, `isEmpty()`) y es heredada por `Queue` y `Stack`.
```text
Herencia de Estructuras Lineales:

       +---------------------+
       | <<abstract>> Sequence|
       +---------------------+
       | + size(): int        |
       | + isEmpty(): boolean |
       +---------------------+
          /|\            /|\
           |              |
+-------------------+  +-------------------+
|    Queue<T>       |  |    Stack<T>       |
+-------------------+  +-------------------+
| - data: List<T>   |  | - data: List<T>   |
+-------------------+  +-------------------+
| + enqueue(T)      |  | + push(T)         |
| + dequeue()       |  | + pop()           |
+-------------------+  +-------------------+
          |
+-------------------+
| DoubleEndedQueue  |
| + enqueueFirst(T) | (Permite encolar al principio)
+-------------------+
```

---

## Parte 5: Recursión y Aplicaciones en Listas

### 5.1 Concepto de Recursión
La recursión es una técnica donde una función se llama a sí misma para resolver un problema dividiéndolo en subproblemas más pequeños.
*   **Caso Base**: Condición que detiene la recursión (por ejemplo, haber llegado al final de una lista).
*   **Caso Recursivo**: La llamada a la función con un estado más pequeño (por ejemplo, el resto de la lista).

### 5.2 Ejemplo Práctico: Contar Pares en una Lista (Video 1)
*   **Problema**: Contar cuántos números pares hay en una `List<Integer>`.
*   **Solución**:
    1. Caso base: Si estamos al final (índice == `lista.size()`), devolvemos 0.
    2. Caso recursivo: Si el número actual es par, sumamos 1 + resultado del resto de la lista. Si es impar, sumamos 0 + resultado del resto de la lista.

```java
public class ContadorPares {
    public static int contarPares(List<Integer> lista) {
        if (lista == null) return 0;
        return contarParesRecursivo(lista, 0);
    }

    private static int contarParesRecursivo(List<Integer> lista, int indice) {
        // Caso base: llegamos al final
        if (indice == lista.size()) {
            return 0;
        }
        // Verificamos el elemento actual
        int elemento = lista.get(indice);
        int esPar = (elemento % 2 == 0) ? 1 : 0;
        // Caso recursivo: acumulamos y llamamos al siguiente índice
        return esPar + contarParesRecursivo(lista, indice + 1);
    }
}
```

```text
Pila de llamadas recursivas para lista = [3, 8, 10] (índice 0, 1, 2):
contarParesRec([3,8,10], 0)
   ├── 3 es impar (0) + contarParesRec(... , 1)
   │    ├── 8 es par (1) + contarParesRec(... , 2)
   │    │    ├── 10 es par (1) + contarParesRec(... , 3)
   │    │    │    └── Caso base: retorna 0
   │    │    └── 1 + 0 = 1
   │    └── 1 + 1 = 2
   └── 0 + 2 = 2  <-- Resultado final: 2
```

### 5.3 Errores Comunes en Recursión con Listas
1.  **No incrementar el índice**: `return esPar + contarParesRecursivo(lista, indice);` (¡Loop infinito!).
2.  **No usar el operador de acumulación**: Si simplemente se hace la llamada recursiva pero no se suma el resultado actual, el valor se pierde en la pila.
3.  **Uso incorrecto del operador**: `indice++` en el parámetro (`contarParesRecursivo(lista, indice++)`). Esto realiza *post-incremento*, pasando el índice actual y luego incrementando *después* de que la llamada al método termine, causando un loop infinito. Usar `indice + 1`.
4.  **No definir el caso base**: Si no existe una condición de parada, se produce un `StackOverflowError`.

---