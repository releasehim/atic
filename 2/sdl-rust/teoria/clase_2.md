# Rust 2025 - Clase 2

## Temario

* Estructuras de control
* Funciones
* Testing
* Borrowing
* Ownership
* Lifetime

---

# Estructuras de control

## `if`, `if-else`

La estructura básica es la siguiente:

```rust
if condicion_booleana {
    // Hace algo porque condicion_booleana es true
}

if condicion_booleana {
    // Hace algo porque condicion_booleana es true
} else {
    // Hace algo porque condicion_booleana es false
}
```

## `if-else if`

```rust
if condicion_booleana {
    // Hace algo porque la condicion booleana es true
} else if otra_condicion {
    // Hace algo porque otra_condicion es true
} else {
    // Hace algo porque otra_condicion y condicion_booleana son false
}
```

## `if` con declaración `let`

El `if` puede utilizarse en el lado derecho de una asignación, siempre que todas las ramas devuelvan el mismo tipo:

```rust
let data = if condicion_booleana { 20 } else { 0 };

fn main() {
    let number: i32 = 10;
    let condicion_booleana: bool = number < 10;

    let data: i32 = if condicion_booleana {
        // Pueden haber más instrucciones
        println!("Entro por aca!");
        number * number
    } else {
        let mut n: i32 = number;
        n *= 2;
        n
    };

    println!("{}", data);
}
```

---

## `match`

La estructura `match` permite comparar un valor contra una serie de patrones.

```rust
match algun_valor {
    patron_que_cumple_algun_valor => // hace algo porque lo cumple,
    otro_patron => // hace algo porque lo cumple,
}
```

Los patrones pueden ser: literales, arrays desestructurados, enums, structs, tuplas, variables, comodines (wildcards) o placeholders.

### `match` con variables

```rust
let number = 10;

match number {
    3 => println!("Es tres o hace algo porque es 3"),
    7 => println!("Es siete o hace algo porque es 7"),
    other => println!("Hace algo porque no es 3 ni 7"),
}
```

### `match` con placeholder

```rust
let number = 10;

match number {
    3 => println!("Es tres o hace algo porque es 3"),
    7 => println!("Es siete o hace algo porque es 7"),
    _ => println!("Hace algo porque no es 3 ni 7"),
}
```

Si solo nos interesa el caso contrario sin usar la variable, usamos el comodín `_`:

```rust
let number = 10;

match number {
    3 => println!("Es tres o hace algo porque es 3"),
    7 => println!("Es siete o hace algo porque es 7"),
    _ => (),
}
```

---

## `loop`

El bucle `loop` ejecuta un bloque de código indefinidamente hasta que se usa `break`.

```rust
fn main() {
    let mut number = 10;

    loop {
        number += 1;
        if number == 30 {
            break;
        }
    }

    println!("{}", number);
}
```

### Retornando valores con `loop`

```rust
fn main() {
    let mut number = 10;

    let termina = loop {
        number += 1;
        if number == 30 {
            break true;
        }
    };

    println!("{} {}", number, termina);
}
```

### `loop` con etiquetas (tags)

Las etiquetas permiten romper bucles anidados específicos.

```rust
let mut count = 0;
'counting_up: loop {
    let mut remaining = 10;
    loop {
        if remaining == 9 {
            break;
        }
        if count == 2 {
            break 'counting_up;
        }
        remaining -= 1;
    }
    count += 1;
}
println!("End count = {count}");
```

---

## `while`

```rust
let mut number = 0;

while number < 10 {
    println!("{number}");
    number += 2;
}
```

---

## `for`

Recorre colecciones.

```rust
let arreglo = [1, 2, 3, 4, 5];
for elemento in arreglo {
    println!("El valor es: {elemento}");
}
```

También se puede usar con rangos:

```rust
let limite = 5;

for i in 1..limite+1 {
    println!("El valor es: {i}");
}

// Iteración inversa
for i in (1..limite+1).rev() {
    println!("El valor es: {i}");
}
```

---

# Funciones

La definición de una función se realiza con la palabra reservada `fn`, seguida del nombre (en snake_case) y luego los argumentos entre paréntesis. Entre llaves se encuentra el código propio del alcance (scope) de la función.

```rust
fn mi_nueva_function(arg1: tipo, arg2: tipo, arg_n: tipo) {
    // código propio del scope de la función
}
```

### Ejemplos con tipos de datos

```rust
fn mi_funcion(data: i32) {
    println!("{data}");
}

fn mi_funcion(data: [i32; 7]) {
    for i in data {
        println!("{i}");
    }
}
```

## Funciones: Retornando valores

La flecha `->` indica el tipo de retorno. Podemos usar `return` explícitamente o dejar la última expresión (sin punto y coma) como valor de retorno.

```rust
fn mi_funcion(data: i32) -> i32 {
    println!("{data}");
    return data;
}

fn mi_funcion(data: i32) -> i32 {
    println!("{data}");
    data // Última expresión, se retorna implícitamente
}
```

---

# Ownership y Borrowing

## Concepto general

Para el manejo de memoria de los programas hay dos enfoques comunes:

1. **Garbage Collector**: Busca periódicamente memoria que no se use para limpiarla.
2. **Asignación y liberación explícita**: El programador debe encargarse de la memoria manualmente.

Rust usa un tercer enfoque: la memoria se administra a través del concepto de **propiedad (ownership)** y un conjunto de reglas.

### Reglas del Ownership

1. Cada valor en Rust tiene un **dueño**.
2. Solo puede haber un **dueño a la vez**.
3. Cuando el dueño queda fuera del alcance (scope), el valor se **eliminará**.

### Stack vs Heap

* **Stack**: Es rápida, se libera al finalizar el scope. Aquí van los datos de tipo conocido en tiempo de compilación (ej. `i32`).
* **Heap**: Es flexible, tiene un costo elevado en asignar y recuperar datos. Se libera cuando ya no tiene dueños. Aquí van los datos de tamaño desconocido en tiempo de compilación (ej. `String`).

## Comportamiento con tipos `Copy`

```rust
fn main() {
    let s1 = 10;
    let s2 = s1;
    println!("{}", s1); // Esto funciona porque los enteros implementan el trait Copy
}
```

### Tipos que implementan el trait Copy

* Todos los enteros (`u32`, `i32`, etc.).
* Booleanos (`bool`).
* Punto flotante (`f32`, `f64`).
* Caracteres (`char`).
* Tuplas que solo contengan tipos que implementen `Copy`.

## Comportamiento con `String` (dueño movido)

Cuando el tipo no implementa `Copy`, el valor se mueve y el dueño original ya no puede usarlo.

**Representación en memoria (después de asignar `s1` a `s2`)**:

```text
   s1                      Heap (Datos)
+--------+              +-----+-----+
| ptr    |------------->| 0   | 'h' |
| len 5  |              | 1   | 'e' |
| cap 5  |              | 2   | 'l' |
+--------+              | 3   | 'l' |
                        | 4   | 'o' |
   s2                   +-----+-----+
+--------+
| ptr    |--------------+
| len 5  |              |
| cap 5  |              |
+--------+              |
                        (Tanto s1 como s2 apuntan al mismo bloque en el heap)
```

*(Nota: En realidad, en el caso de una `String`, hacer `let s2 = s1` mueve el puntero a `s2` e invalida `s1` para evitar el doble `free`; el gráfico representa el estado conceptual del puntero compartido).*

## Funciones y Ownership

### Paso por valor (move)

El argumento toma ownership del valor.

```rust
fn main() {
    let dato1 = 10;
    mi_function(dato1);
    println!("{}", dato1); // Funciona porque i32 es Copy
}

fn mi_function(mut data: i32) {
    data += 1;
    println!("Muestro data en la funcion: {}", data);
}
```

### Préstamo (Borrowing) mutable (`&mut`)

```rust
fn main() {
    let mut dato1 = 10;
    mi_funcion(&mut dato1);
    println!("{}", dato1);
}

fn mi_funcion(data: &mut i32) {
    *data += 1; // Es necesario desreferenciar (*) para modificar el valor apuntado
    println!("Muestro data en la funcion: {}", data);
}
```

### Error: Movimiento a función

```rust
fn main() {
    let dato1 = String::from(" Seminario de: ");
    mi_function(dato1);
    println!("{}", dato1); // ERROR: `dato1` fue movido y ya no es válido aquí
}

fn mi_function(data: String) {
    println!("Muestro data en la funcion: {}", data);
}
```

### Préstamo (Borrowing) inmutable (`&`)

Para solucionar el error anterior, podemos pasar una referencia:

```rust
fn main() {
    let dato1 = String::from(" Seminario de: ");
    mi_function(&dato1);
    println!("{}", dato1); // Funciona, solo se prestó el valor
}

fn mi_function(data: &String) {
    println!("Muestro data en la funcion: {}", data);
}
```

### Devolviendo la propiedad (Moving back)

Para recuperar la propiedad, la función debe devolver el valor.

```rust
fn main() {
    let dato1 = String::from(" Seminario de: ");
    let dato1 = mi_funcion(dato1); // Reasignamos el ownership
    println!("{}", dato1);
}

fn mi_funcion(data: String) -> String {
    println!("Muestro data en la funcion: {}", data);
    data // Retorna la propiedad al ámbito llamante
}
```

---

# Lifetime

## Concepto

Cada referencia en Rust tiene una **vida útil (lifetime)**, que es el alcance para el cual esa referencia es válida. La mayoría de las veces, el tiempo de vida se infiere, al igual que los tipos.

El lifetime es la manera que tiene el compilador de Rust de asegurar que una referencia apunta a una ubicación de memoria válida.

## Ejemplo de error de Lifetime

```rust
fn main() {
    let dato1: &i32;
    {
        let otro_scope = 2;
        dato1 = &otro_scope;
    }
    println!("{}", dato1);
}

// Error de compilación:
// error[E0597]: `otro_scope` does not live long enough
//  --> src/main.rs:15:17
//   |
// 15 |         dato1 = &otro_scope;
//    |                 ^^^^^^^^^^^ borrowed value does not live long enough
// ...
//    |         `otro_scope` dropped here while still borrowed
// 18 |     println!("{}", dato1);
//    |                  -------- borrow later used here
```

### Anotaciones de Lifetime en funciones

Cuando una función devuelve una referencia prestada, el compilador necesita saber si esa referencia viene del argumento `data1` o `data2`.

```rust
// Código con error (falta el especificador de Lifetime)
fn crear(data1: &str, data2: &str) -> &String {
    let resultado:String = data1.to_string().add(data2);
    &resultado
}

// Error:
// error[E0106]: missing lifetime specifier
// help: this function's return type contains a borrowed value...
```

**Solución propuesta por el compilador**:

```rust
fn crear<'a>(data1: &'a str, data2: &'a str) -> &'a String {
    // ...
}
```

### Error por retornar referencia a variable local

```rust
use std::ops::Add;

fn main() {
    let d1 = "str1";
    let d2 = "str2";
    let r = crear(d1, d2);
    println!("{}", r);
}

// Error
fn crear<'a>(data1: &'a str, data2: &'a str) -> &'a str {
    let d1 = data1.to_string();       // Variable local
    let resultado: &str = d1.add(data2).as_str(); // Se crea una referencia temporal
    &resultado                        // Se retorna una referencia a una variable local
}

// error[E0515]: cannot return value referencing temporary value
// returns a value referencing data owned by the current function
```

### Ejemplo correcto de Lifetime con comparación

```rust
fn main() {
    let string1 = String::from("Seminario de:");
    let string2 = "Rust!!!";
    let result = mas_largo(string1.as_str(), string2);
    println!("El mas largo es: {}", result);
}

fn mas_largo<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

### Relación de vidas (comprobación en tiempo de compilación)

```rust
fn mayor<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y { x } else { y }
}

fn main() {
    let a = 10;
    let r;
    {
        let b = 20;
        r = mayor(&a, &b); //  aquí 'a = lifetime de b (el menor de los dos)
    }
    println!("Mayor: {}", r); //  Error: b ya no está vivo
}
```

**Explicación**:

* `&a` vive todo `main`.
* `&b` vive solo dentro del bloque.
* `mayor` podría devolver `&b`.
* Entonces `'a = lifetime de b` (el más corto).
* `r` no puede usarse después del bloque porque podría estar apuntando a `b`, por lo tanto, **Rust te impide compilar**.

## Sintaxis de Lifetime

```rust
&i32                  // una referencia (lifetime elidido)
&'a i32               // una referencia con lifetime explícito
&'a mut i32           // una referencia mutable con lifetime explícito
```

---

# Tests (Unit Testing)

## Concepto

En desarrollo de software, es la práctica en la cual se crean pruebas automatizadas para verificar el correcto funcionamiento individual de las unidades de código más pequeñas, como funciones, métodos o clases. Estas pruebas se enfocan en aislar y probar una unidad de código de forma independiente, sin depender de otras partes del sistema.

## Algunas ventajas

* **Detección temprana de errores**: Permite identificar y corregir errores en las primeras etapas del desarrollo.
* **Mejora de la calidad del código**: Promueve la escritura de código más limpio, modular y de alta calidad.
* **Facilita la refactorización**: Si las pruebas pasan después de realizar cambios, da confianza de que la funcionalidad sigue intacta.
* **Documentación viva**: Al leer las pruebas, se obtiene una comprensión clara de cómo se espera que funcione cada unidad de código.

> **Importante**: Unit testing no asegura que nuestro código no tenga errores, sino que es una **buena práctica para reducirlos**.

## Unit testing en Rust

### Macros de aserción

```rust
#[test]         // Atributo para marcar una función como test
assert!(expression); // Verifica que la expresión sea true
assert_eq!(v1, v2);  // Verifica que v1 y v2 sean iguales
assert_ne!(v1, v2);  // Verifica que v1 y v2 sean diferentes
```

### Comandos para ejecutar tests

```bash
cargo test                 # Ejecuta todos los tests
cargo test nombre_del_test # Ejecuta un test específico por nombre
cargo test prefijo         # Ejecuta todos los tests que empiecen con un prefijo
```

### Atributos útiles

```rust
#[ignore]       # Ignora este test al ejecutar `cargo test`
#[should_panic] # Espera que el test entre en pánico (para probar errores)
#[should_panic(expected = "mensaje del panic")] # Espera un pánico con un mensaje específico
```

### Ejemplo de prueba

```rust
// Definir una función llamada contar_letras que reciba un &str y un char,
// y retorne cuántas veces aparece el carácter en el string.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contar_letras() {
        let texto = "hola mundo";
        assert_eq!(contar_letras(texto, 'o'), 2);
        assert_eq!(contar_letras(texto, 'z'), 0);
    }
}
```
