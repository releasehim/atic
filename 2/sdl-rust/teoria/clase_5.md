# Clase 5: Traits, POO y Closures

## Temario

- Traits
- POO (Programación Orientada a Objetos)
- Closures

---

## Traits

### ¿Qué es?

- Es una funcionalidad particular que tiene un tipo y puede compartir con otros tipos.
- Podemos usar traits para definir comportamiento de manera abstracta.
- Se pueden usar traits como límites en tipos de datos genéricos para determinada funcionalidad que el tipo genérico debe cumplir.
- Son similares a las interfaces llamadas en otros lenguajes pero con algunas diferencias.

### Ejemplo I: Definición e implementación simple

```rust
pub trait MulI32 {
    fn mul(&self, other: i32) -> f64; // Método abstracto
    fn hace_algo_concreto(&self) {   // Método por defecto
        println!("hace_algo_concreto");
    }
}

impl MulI32 for f64 {
    fn mul(&self, other: i32) -> f64 {
        self * other as f64
    }
}

fn main() {
    let v1 = 2.8;
    let v2 = 4;
    let r = v1.mul(v2);
    println!("{}", r);
}
```

### Ejemplo II: Trait compartido entre tipos

```rust
struct Perro {}
struct Gato {}

pub trait Animal {
    fn hablar(&self) -> String;
}

impl Animal for Perro {
    fn hablar(&self) -> String {
        "Guau!".to_string()
    }
}

impl Animal for Gato {
    fn hablar(&self) -> String {
        "Miau!".to_string()
    }
}

fn main() {
    let gato = Gato {};
    let perro = Perro {};
    println!("{} {}", gato.hablar(), perro.hablar());
}
```

### Limitando un genérico con Trait (Bounds)

```rust
pub fn imprimir_hablar<T: Animal>(animal: &T) {
    println!("Hablo! {}", animal.hablar());
}

fn main() {
    let gato = Gato {};
    let perro = Perro {};
    imprimir_hablar(&gato);
    imprimir_hablar(&perro);
}
```

### Trait como parámetro de función (`impl Trait`)

```rust
pub fn imprimir_hablar(animal1: &impl Animal, animal2: &impl Animal) {
    println!("Hablando! {} {}", animal1.hablar(), animal2.hablar());
}

fn main() {
    let gato = Gato {};
    let perro = Perro {};
    imprimir_hablar(&gato, &perro);
}
```

### Múltiples Traits y sintaxis `where`

```rust
// Con sintaxis impl Trait
pub fn imprimir_hablar(animal: &(impl Animal + OtroTrait)) {
    println!("Hablo! {}", animal.hablar());
}

// Con sintaxis where (recomendado para múltiples bounds)
pub fn imprimir_hablar<T>(animal: &T) 
where 
    T: Animal + OtroTrait 
{
    println!("Hablo! {}", animal.hablar());
}
```

---

## POO (Programación Orientada a Objetos)

### Elementos fundamentales

- Clases
- Métodos
- Atributos
- Objetos

### Conceptos principales

- **Encapsulamiento**: Permite agrupar comportamiento y datos, y restringirlos a través de su interfaz.
- **Abstracción**: Representar un objeto del mundo real con sus características apropiadas y que pueda comunicarse con otros objetos sin saber cómo están hechas sus implementaciones.
- **Polimorfismo**: Distintos tipos de objetos tienen la misma interfaz de comunicación pero su implementación es distinta.
- **Herencia**: Mecanismo mediante el cual un objeto puede heredar elementos de otro objeto.
- **Modularidad**: Organización del código en unidades lógicas.

### Encapsulamiento en Rust

```rust
// ejemplos.rs
pub struct Ejemplo {
    atr1: i32,
    atr2: i32,
}

impl Ejemplo {
    pub fn new(atr1: i32, atr2: i32) -> Ejemplo {
        Ejemplo { atr1, atr2 }
    }
    pub fn calcular(&self) -> i32 {
        self.atr1 * self.atr2
    }
}
```

```rust
// main.rs
mod ejemplos;

fn main() {
    let mut e = Ejemplo::new(3, 4);
    // Error: field `atr1` of struct `Ejemplo` is private
    // e.atr1 = 5; 
}
```

### Abstracción en Rust

La abstracción se logra exponiendo solo la interfaz pública (`pub`) y ocultando los detalles internos de implementación.

```rust
// main.rs
mod ejemplos;

fn main() {
    let e = Ejemplo::new(3, 4);
    e.calcular(); // Interacción a través de un método público
}
```

### Polimorfismo en Rust

Podemos lograr polimorfismo a través de los **Traits**.

```rust
use std::collections::LinkedList;
use std::collections::VecDeque;

trait PushBack<T> {
    fn push_back(&mut self, elt: T);
}

impl<T> PushBack<T> for LinkedList<T> {
    fn push_back(&mut self, elt: T) {
        self.push_back(elt);
        println!("Acá podría ir otra lógica! Sobre LinkedList!");
    }
}

impl<T> PushBack<T> for VecDeque<T> {
    fn push_back(&mut self, elt: T) {
        self.push_back(elt);
        println!("Acá podría ir otra lógica! Sobre VecDeque!");
    }
}

fn main() {
    let mut list = LinkedList::new();
    let mut vecdeque = VecDeque::new();

    PushBack::push_back(&mut vecdeque, 3);
    PushBack::push_back(&mut list, 3);

    println!("{:?}", list);
    println!("{:?}", vecdeque);
}
```

### Herencia en Rust

**Concepto:** La herencia es un mecanismo mediante el cual un objeto puede heredar elementos de la definición de otro objeto, obteniendo así los datos y el comportamiento del objeto principal sin tener que definirlos nuevamente. Una de las razones principales para usar herencia es la reutilización del código.

**En Rust:** Si un lenguaje debe tener herencia para ser orientado a objetos, entonces Rust no lo es. No hay forma de definir una estructura que herede los campos y métodos de otra estructura principal. Sin embargo, se puede lograr la **reutilización de código** mediante la **composición** (anidando un tipo dentro de otro) y los **Traits**.

#### Diagrama conceptual (Herencia clásica vs Composición con Traits en Rust)

El siguiente diagrama representa el enfoque típico para "heredar" comportamiento en Rust, almacenando los datos comunes en una estructura separada (`DatosVehiculo`) y compartiendo la lógica a través de un `trait`.

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

#### Implementación del diagrama en Rust

```rust
struct DatosVehiculo {
    matricula: String,
    modelo: i32,
    potencia: i32,
}

trait Vehiculo {
    fn get_matricula(&self, datos: &DatosVehiculo) -> String {
        datos.matricula.clone()
    }
    fn get_modelo(&self, datos: &DatosVehiculo) -> i32 {
        datos.modelo
    }
    fn get_potencia(&self, datos: &DatosVehiculo) -> i32 {
        datos.potencia
    }
}

struct Taxi {
    datos_vehiculo: DatosVehiculo,
    numero_licencia: i32,
}

impl Taxi {
    fn new(numero_licencia: i32, matricula: String, modelo: i32, potencia: i32) -> Taxi {
        Taxi {
            datos_vehiculo: DatosVehiculo { matricula, modelo, potencia },
            numero_licencia,
        }
    }
}
impl Vehiculo for Taxi {}

struct Autobus {
    datos_vehiculo: DatosVehiculo,
    numero_plazas: i32,
}

impl Autobus {
    fn new(numero_plazas: i32, matricula: String, modelo: i32, potencia: i32) -> Autobus {
        Autobus {
            datos_vehiculo: DatosVehiculo { matricula, modelo, potencia },
            numero_plazas,
        }
    }
}
impl Vehiculo for Autobus {}

fn main() {
    let t = Taxi::new(1, "U".to_string(), 2002, 145);
    let mat_t = t.get_matricula(&t.datos_vehiculo);

    let a = Autobus::new(20, "U".to_string(), 2002, 145);
    let mat_a = a.get_matricula(&a.datos_vehiculo);
    
    println!("Matrícula Taxi: {}", mat_t);
    println!("Matrícula Autobus: {}", mat_a);
}
```

### Modularidad en Rust

Rust nos brinda esta característica a través de la creación y uso de módulos (`mod`), como así también la importación de librerías externas (`crates`). Hemos estado utilizando esto en las prácticas de la materia.

### Resumen de POO en Rust

- **Abstracción y Encapsulamiento:** Sí, a través de `pub` y `impl`.
- **Polimorfismo:** Sí, a través de los **Traits**.
- **Herencia:** No directa, pero se logra reutilización mediante **Composición** y **Traits**.
- **Modularidad:** Sí, a través del sistema de **módulos**.

---

## Closures

### ¿Qué son?

Los **closures** son funciones anónimas que se pueden guardar en una variable o pasar como argumentos a otras funciones. Se puede crear el closure en un lugar y luego llamarlo en otro para evaluar algo en un contexto diferente. Permiten la reutilización de código.

### Ejemplo en contexto: Inventario y preferencias

```rust
#[derive(Debug, PartialEq, Copy, Clone)]
enum Color {
    Rojo,
    Azul,
}

struct Inventario {
    remeras: Vec<Color>,
}

impl Inventario {
    fn get_color(&self, color_favorito: Option<Color>) -> Color {
        color_favorito.unwrap_or_else(|| self.mas_stockeado())
    }

    fn mas_stockeado(&self) -> Color {
        let mut num_rojo = 0;
        let mut num_azul = 0;
        
        for color in &self.remeras {
            match color {
                Color::Rojo => num_rojo += 1,
                Color::Azul => num_azul += 1,
            }
        }
        
        if num_rojo > num_azul {
            Color::Rojo
        } else {
            Color::Azul
        }
    }
}

fn main() {
    let store = Inventario {
        remeras: vec![Color::Azul, Color::Rojo, Color::Azul],
    };

    let u_pref1 = Some(Color::Rojo);
    let get_color1 = store.get_color(u_pref1);
    println!("El usuario prefiere {:?} y obtiene {:?}", u_pref1, get_color1);

    let u_pref2 = None;
    let get_color2 = store.get_color(u_pref2);
    println!("El usuario prefiere {:?} y obtiene {:?}", u_pref2, get_color2);
}
```

### Inferencia de tipos en Closures

Rust puede inferir los tipos de los argumentos y el retorno de un closure. Incluso se pueden omitir las llaves si es una sola expresión.

```rust
fn main() {
    fn sumar_v1(x: u32, y: u32) -> u32 { x + y }
    let sumar_v2 = |x: u32, y: u32| -> u32 { x + y };
    let sumar_v3 = |x, y| { x + y };
    let sumar_v4 = |x, y| x + y;

    sumar_v1(3, 3);
    sumar_v2(3, 3);
    sumar_v3(3, 3);
    sumar_v4(3, 3);
}
```

### Inferencia de tipos con estructuras personalizadas

```rust
use std::ops::Add;

#[derive(Clone, Copy)]
struct Caja {
    dato: i32,
}

impl Add for Caja {
    type Output = i32;
    fn add(self, otro: Self) -> i32 {
        self.dato + otro.dato
    }
}

fn main() {
    let sumar_v3 = |x, y| { x + y };
    let sumar_v4 = |x, y| x + y;
    
    let c1 = Caja { dato: 2 };
    let c2 = Caja { dato: 5 };
    
    println!("{}", sumar_v3(c1, c2));
    println!("{}", sumar_v4(c1, c2));
}
```

### Closures: Referencias y Ownership

Los closures pueden capturar variables de su entorno de tres maneras:

1. **Prestado inmutable (`&T`)**: El closure toma prestado el valor.
2. **Prestado mutable (`&mut T`)**: El closure toma prestado mutablemente el valor.
3. **Movimiento (`move`)**: El closure toma posesión del valor.

**Ejemplo con préstamo inmutable:**

```rust
fn main() {
    let list = vec!["uno".to_string(), "dos".to_string(), "tres".to_string()];
    println!("Antes de definir el closure: {:?}", list);

    let pide_prestado = || println!("Desde el closure: {:?}", list);
    println!("Antes de llamar al closure: {:?}", list);

    pide_prestado();
    println!("Luego de llamar al closure: {:?}", list);
}
```

**Ejemplo con préstamo mutable:**

```rust
fn main() {
    let mut list = vec![1, 2, 3];
    println!("Antes de definir el closure: {:?}", list);

    let mut prestado_mutable = || list.push(7);
    prestado_mutable();

    println!("Luego de llamar al closure: {:?}", list);
}
```

**Ejemplo con `move` (Error común):**
Usar `move` fuerza al closure a tomar propiedad de las variables que captura. Si luego se intenta usar la variable en el ámbito original, el compilador dará un error.

```rust
fn main() {
    let mut list = vec![1, 2, 3];
    println!("Antes de definir el closure: {:?}", list);

    // Error: la variable list se mueve al closure y ya no es válida en el ámbito principal
    let mut prestado_mutable = move || { list.push(7); println!("{:?}", list); };
    
    prestado_mutable();
    // println!("Luego de llamar al closure: {:?}", list); // <-- ERROR: borrow later used here
}
```

### Closures como parámetros de funciones

Un uso muy común es pasar closures a métodos de iteradores como `sort_by_key`.

```rust
#[derive(Debug)]
struct Rectangulo {
    ancho: u32,
    alto: u32,
}

fn main() {
    let mut arreglo = [
        Rectangulo { ancho: 10, alto: 1 },
        Rectangulo { ancho: 3, alto: 5 },
        Rectangulo { ancho: 7, alto: 12 },
    ];

    arreglo.sort_by_key(|r| r.ancho); // Closure como argumento
    println!("{:?}", arreglo);
}
```
