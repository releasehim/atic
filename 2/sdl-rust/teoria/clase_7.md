# Clase 7: Archivos con Serde, Tests y Smart Pointers

## Temario

- Archivos con Serde
- Más sobre Testing
- Smart Pointers

---

## Archivos con Serde

**Serde** es un *framework* para serializar y deserializar structs de Rust de manera eficiente y genérica.
El ecosistema de Serde consiste en estructuras de datos que saben cómo serializarse y deserializarse a sí mismos, junto con formatos de datos que saben cómo serializar y deserializar otras cosas. Serde proporciona la capa por la cual estos dos grupos interactúan.

Algunos formatos compatibles: JSON, YAML, TOML, Pickle, BSON, etc.

### Ejemplo básico: Serialización y Deserialización a JSON

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Punto {
    x: i32,
    y: i32,
}

fn main() {
    let p = Punto { x: 1, y: 2 };

    // Convierte el punto a un String con formato JSON.
    let punto_serializado = serde_json::to_string(&p).unwrap();
    println!("serialized = {}", punto_serializado);

    // Convierte el String con formato JSON de vuelta a un Punto.
    let p_s: Punto = serde_json::from_str(&punto_serializado).unwrap();
    println!("deserialized = {:?}", p_s);
}
```

### Ejemplo: Lectura y escritura de archivos con Serde

```rust
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct Punto {
    x: i32,
    y: i32,
}

fn main() {
    let p = Punto { x: 1, y: 2 };

    // Serializar y escribir en un archivo
    let punto_serializado = serde_json::to_string(&p).unwrap();
    let mut f = File::create("src/archivo_puntos.json").unwrap();
    f.write_all(&punto_serializado.as_bytes()).unwrap();

    // Leer y deserializar desde el archivo
    let mut f_o = File::open("src/archivo_puntos.json").unwrap();
    let mut buf = String::new();
    f_o.read_to_string(&mut buf).unwrap();
    let pl: Punto = serde_json::from_str(&buf).unwrap();
    println!("{:?}", pl);
}
```

### Ejemplo: Serializar un Vector de Structs

```rust
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct Punto {
    x: i32,
    y: i32,
}

fn main() {
    let mut w = Vec::new();
    let p = Punto { x: 1, y: 2 };
    w.push(&p);
    let p = Punto { x: 5, y: 12 };
    w.push(&p);

    // Serializar el vector
    let v_s = serde_json::to_string(&w).unwrap();
    let mut f = File::create("src/archivo_puntos.json").unwrap();
    f.write_all(&v_s.as_bytes()).unwrap();

    // Deserializar el vector desde el archivo
    let mut f = File::open("src/archivo_puntos.json").unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    let w1: Vec<Punto> = serde_json::from_str(&buf).unwrap();
    println!("{:?}", w1);
}
```

---

## Tests (Unit Testing)

En desarrollo de software es la práctica en la cual se crean pruebas automatizadas para verificar el correcto funcionamiento individual de las unidades de código más pequeñas (funciones, métodos). Estas pruebas se enfocan en aislar y probar una unidad de código de forma independiente.

### Ventajas de Unit Testing

- **Detección temprana de errores:** Permite identificar errores en etapas tempranas.
- **Mejora de la calidad del código:** Promueve código más limpio y modular.
- **Facilita la refactorización:** Da seguridad al modificar el código.
- **Documentación viva:** Las pruebas muestran cómo se espera que funcione el código.

> **Importante:** Unit testing no asegura que nuestro código no tenga errores, sino que es una **buena práctica para reducirlos**.

### Unit Testing en Rust

#### Macros y Atributos

```rust
#[test]                     // Marca una función como test
assert!(expression);        // Verifica que la expresión sea true
assert_eq!(v1, v2);         // Verifica que v1 y v2 sean iguales
assert_ne!(v1, v2);         // Verifica que v1 y v2 sean diferentes

#[ignore]                   // Ignora el test al ejecutar `cargo test`
#[should_panic]             // Espera que el test entre en pánico
#[should_panic(expected = "mensaje del panic")] // Espera un pánico con un mensaje específico
```

#### Comandos

```bash
cargo test                    # Ejecuta todos los tests
cargo test nombre_del_test    # Ejecuta un test específico
cargo test prefijo            # Ejecuta los tests que empiecen con un prefijo
```

### Cobertura de Código (Coverage)

Es una métrica que indica el porcentaje de código fuente que ha sido ejecutado durante la ejecución de las pruebas. Una alta cobertura no garantiza la ausencia de errores, pero proporciona una mayor confianza en la calidad del software.

**Ventajas:**

- Identificación de código no probado.
- Guía para la creación de nuevas pruebas.
- Medición de la calidad de las pruebas.

#### Coverage en Rust con `tarpaulin`

```bash
cargo tarpaulin --target-dir src/coverage --skip-clean
cargo tarpaulin --target-dir src/coverage --skip-clean --out html
```

---

## Smart Pointers

- Como vimos, `&` y `&mut` son referencias (punteros) que toman prestado datos sin costo adicional.
- Los **Smart Pointers** son estructuras de datos que actúan como referencias, pero también contienen metadatos y características especiales.
- A diferencia de las referencias, los Smart Pointers **poseen** los datos a los que apuntan.
- Implementan los traits `Drop` y `Deref`.
- **Nota:** `String` y `Vec` son considerados Smart Pointers, ya que poseen memoria y permiten manipularla.

### `Box<T>` (Puntero a Heap)

`Box<T>` permite almacenar datos en el *heap* en lugar de la *pila*. Se usa principalmente para tipos de tamaño desconocido en tiempo de compilación o recursivos.

#### Error de tipo común con `Box`

**Imagen reemplazada (Error de compilación):**

```text
fn main() {
    let caja = Box::new(5);
    if caja == 5 {   // <--- ERROR AQUÍ
        println!("es cinco!");
    }
}

// Error en terminal:
error[E0308]: mismatched types
 --> src/main.rs:3:8
  |
3 |     if caja == 5 {
  |        ^^^^   - expected `Box<{integer}>`, found integer
  |        |
  |        expected struct `Box<{integer}>`, found type `{integer}`
  |
  = note: expected struct `Box<{integer}>`
             found type `{integer}`
```

#### Solución: Desreferenciar (`*`)

```rust
fn main() {
    let caja = Box::new(5);
    if *caja == 5 { // Desreferenciamos para acceder al valor interno
        println!("es cinco!");
    }
}
```

#### Uso de `Box<T>` para estructuras recursivas

**Problema (Tamaño infinito):**

```rust
enum MiLista {
    Nodo(i32, MiLista), // <--- ERROR: Tamaño infinito
    Nada
}
```

**Solución con `Box`:**

```rust
enum MiLista {
    Nodo(i32, Box<MiLista>),
    Nada
}

fn main() {
    use MiLista::*;
    let n4 = Nada;
    let n3 = Nodo(3, Box::new(n4));
    let n2 = Nodo(2, Box::new(n3));
    let n1 = Nodo(1, Box::new(n2));
}
```

### Trait `Deref` (Desreferencia)

El trait `Deref` permite que un Smart Pointer se comporte como una referencia normal mediante el operador `*`.

```rust
use std::ops::Deref;

struct Caja<T> {
    d: T,
}

impl<T> Caja<T> {
    fn new(d: T) -> Caja<T> {
        Caja { d }
    }
}

impl<T> Deref for Caja<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.d
    }
}

fn main() {
    let caja = Caja::new(5);
    if *caja == 5 { // Funciona gracias a la implementación de Deref
        println!("es cinco!");
    }
}
```

### Trait `Drop` (Destructor)

El trait `Drop` permite ejecutar código cuando un valor sale del ámbito (termina su vida útil).

```rust
impl<T> Drop for Caja<T> {
    fn drop(&mut self) {
        println!("Adios!!!");
    }
}

fn main() {
    let caja = Caja::new(5);
    if *caja == 5 {
        println!("es cinco!");
    }
    // Al final del bloque, `drop` se ejecuta automáticamente
    println!("Terminando el main");
}

// Salida:
// es cinco!
// Adios!!!
// Terminando el main
```

**Error al intentar llamar a `drop` explícitamente:**

```rust
caja.drop(); // error[E0040]: explicit use of destructor method not allowed
```

**Solución correcta:** Usar la función `std::mem::drop`.

```rust
drop(caja);
```

### Extra: Diagrama de Herencia (Rust)

**Imagen reemplazada (Diagrama de Vehículo reutilizado):**

```text
          +---------------------------------------------------+
          |                     Vehiculo (Trait)               |
          |   Define comportamiento común para vehículos       |
          +---------------------------------------------------+
          |  + get_matricula(&self, datos: &DatosVehiculo)     |
          |  + get_modelo(&self, datos: &DatosVehiculo)        |
          |  + get_potenciaCV(&self, datos: &DatosVehiculo)    |
          +---------------------------------------------------+
                             |
            (Implementa el trait) / \ (Implementa el trait)
                             |   |
                             v   v
+------------------------+   +---------------------------+
|         Taxi           |   |         Autobus           |
+------------------------+   +---------------------------+
| datos_vehiculo (Struct)|   | datos_vehiculo (Struct)   |
| numero_licencia: i32   |   | numero_plazas: i32        |
+------------------------+   +---------------------------+
| new(...)               |   | new(...)                  |
| set_numero_licencia()  |   | set_numero_plazas()      |
| get_numero_licencia()  |   | get_numero_plazas()      |
+------------------------+   +---------------------------+
```

### `Rc<T>` (Reference Counted)

Permite la **propiedad múltiple** (Multiple Ownership). Es útil en estructuras como grafos, donde un nodo puede tener varios padres.

**Imagen reemplazada (Diagrama de Grafo para propiedad múltiple):**

```text
      (a)
     / | \
    /  |  \
   v   v   v
  (b) (d) (e)
   |   |   |
   |   |   v
   v   v  (f)
  (c) (c)  ^
           |
          (c)
```

*Un nodo `c` puede ser apuntado por múltiples nodos (`b`, `d`, `f`), requiriendo propiedad múltiple.*

#### Error de Propiedad Múltiple con `Box`

```rust
enum MiLista {
    Nodo(i32, Box<MiLista>),
    Nada,
}

fn main() {
    use MiLista::*;
    let n4 = Nada;
    let n3 = Nodo(3, Box::new(n4));
    let n2 = Nodo(2, Box::new(n3));
    // Error: `n2` se mueve, pero queremos que n1 y n5 compartan n2
    let n1 = Nodo(1, Box::new(n2));
    let n5 = Nodo(5, Box::new(n2)); // ERROR: use of moved value: `n2`
}
```

#### Solución con `Rc<T>`

```rust
use std::rc::Rc;

enum MiLista {
    Nodo(i32, Rc<MiLista>),
    Nada,
}

fn main() {
    use MiLista::*;
    let n4 = Nada;
    let n3 = Nodo(3, Rc::new(n4));
    let n2 = Nodo(2, Rc::new(n3));

    let n2_rc = Rc::new(n2); // Rc con conteo de referencias (1)
    let n1 = Nodo(1, Rc::clone(&n2_rc)); // Conteo de referencias (2)
    let n5 = Nodo(5, Rc::clone(&n2_rc)); // Conteo de referencias (3)

    println!("cantidad de refs: {}", Rc::strong_count(&n2_rc)); // salida: 3
}
```

#### Recorriendo la lista con `Rc` (DFS simulado)

```rust
impl MiLista {
    fn get_siguiente(&self) -> Option<&Rc<MiLista>> {
        match self {
            MiLista::Nodo(_, s) => Some(s),
            MiLista::Nada => None,
        }
    }
    fn get_dato(&self) -> Option<&i32> {
        match self {
            MiLista::Nodo(v, _) => Some(v),
            MiLista::Nada => None,
        }
    }
}

fn psudo_dfs(a: &Rc<MiLista>) {
    if !a.get_dato().is_none() {
        if a.get_dato().unwrap() == &1 {
            // Lógica con 1
        }
        println!("{}", a.get_dato().unwrap());
    }
    if let Some(sig) = a.get_siguiente() {
        psudo_dfs(sig);
    }
}
```

### `RefCell<T>` y Mutabilidad Interior

- Permite mutar datos incluso cuando hay referencias inmutables a esos datos (lo cual normalmente está prohibido por las reglas de *borrowing*).
- Usa código inseguro internamente para modificar las reglas de Rust, pero la API sigue siendo segura.
- Las reglas de *borrowing* se verifican en **tiempo de ejecución** en lugar de en tiempo de compilación.

#### Ejemplo de `RefCell` con `Rc` (Lista mutable)

```rust
use std::rc::Rc;
use std::cell::RefCell;

enum MiLista {
    Nodo(RefCell<i32>, Rc<MiLista>),
    Nada,
}

impl MiLista {
    fn get_siguiente(&self) -> Option<&Rc<MiLista>> {
        match self {
            MiLista::Nodo(_, s) => Some(s),
            MiLista::Nada => None,
        }
    }
    fn get_dato(&self) -> Option<&RefCell<i32>> {
        match self {
            MiLista::Nodo(v, _) => Some(v),
            MiLista::Nada => None,
        }
    }
}

fn main() {
    use MiLista::*;

    let n4 = Nada;
    let n4_rc = Rc::new(n4);
    let n3 = Nodo(RefCell::new(3), n4_rc.clone());
    let n3_rc = Rc::new(n3);
    let n2 = Nodo(RefCell::new(2), n3_rc.clone());
    let n2_rc = Rc::new(n2);
    let n1 = Nodo(RefCell::new(1), n2_rc.clone());
    let n5 = Nodo(RefCell::new(5), n2_rc.clone());

    let n5_rc = Rc::new(n5);
    let n1_rc = Rc::new(n1);

    let v = [n1_rc.clone(), n2_rc.clone(), n3_rc.clone(), n4_rc.clone(), n5_rc.clone()];

    // Recorremos y modificamos utilizando RefCell
    for i in v {
        if !i.get_dato().is_none() {
            let d = i.get_dato().unwrap();
            psudo_dfs_mutable(&i);
        }
    }
}

fn psudo_dfs_mutable(a: &Rc<MiLista>) {
    if !a.get_dato().is_none() {
        let mut d = a.get_dato().unwrap().borrow_mut(); // Mutamos el valor en tiempo de ejecución
        if *d == 1 {
            *d = 10; // Modificamos el valor dentro de RefCell
        }
        println!("{}", *d);
    }
    if let Some(sig) = a.get_siguiente() {
        psudo_dfs_mutable(sig);
    }
}
```

#### Repaso de Smart Pointers

| Smart Pointer | Propiedad | Préstamos (Borrowing) | Verificación |
| :--- | :--- | :--- | :--- |
| **Box<T>** | Única | Inmutable o Mutable | Tiempo de compilación |
| **Rc<T>** | Múltiple | Solo Inmutable | Tiempo de compilación |
| **RefCell<T>** | Única | Inmutable o Mutable | Tiempo de ejecución |
| **String** | Única | - | - |
| **Vec<T>** | Única | - | - |

> **Nota clave:** `RefCell<T>` permite la mutación interior, lo que significa que podemos mutar el valor dentro de `RefCell<T>` incluso cuando la variable que contiene el `RefCell` es inmutable, gracias a la verificación en tiempo de ejecución.
