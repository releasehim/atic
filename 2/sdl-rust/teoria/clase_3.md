# Clase 3: Structs, Enums, Option y Collections (Parte 1)

## Temario
- Structs
- Enums
- Option
- Collections (Parte 1)

---

## Structs

### ¿Qué son?
Es un tipo de dato personalizado que permite empaquetar y nombrar valores relacionados que forman un conjunto de datos. Son similares, en la programación orientada a objetos, al conjunto de atributos que tiene una clase.

### ¿Cómo se definen?
Se definen con la palabra clave `struct` de la siguiente manera:

```rust
struct NombreDelStruct {
    nombre_atributo_1: tipo1,
    nombre_atributo_2: tipo2,
    nombre_atributo_n: tipo_n,
}
```

Su definición no necesariamente tiene que ser dentro de la función `main`.

```rust
struct Persona {
    nombre: String,
    apellido: String,
    dni: i32,
}

fn main() {
    let persona1 = Persona {
        nombre: "Lionel".to_string(),
        apellido: "Messi".to_string(),
        dni: 1,
    };
    println!("nombre: {} apellido: {} dni: {}", persona1.nombre, persona1.apellido, persona1.dni);
}
```

### Init Shorthand
Cuando el nombre del campo y la variable que se asigna coinciden, se puede usar la sintaxis abreviada.

```rust
fn main() {
    let persona1 = nueva_persona(
        "Lionel".to_string(),
        "Messi".to_string(),
        1
    );
    println!("nombre: {} apellido: {} dni: {}", persona1.nombre, persona1.apellido, persona1.dni);
}

fn nueva_persona(nombre: String, apellido: String, dni: i32) -> Persona {
    Persona {
        apellido,
        dni,
        nombre,
    }
}
```

### Modificaciones
Para modificar un campo, la instancia debe ser declarada como `mut`.

```rust
fn main() {
    let mut persona1 = nueva_persona(
        "Lionel".to_string(),
        "Messi".to_string(),
        1
    );
    println!("nombre: {} apellido: {} dni: {}", persona1.nombre, persona1.apellido, persona1.dni);
    
    persona1.dni = 99;
    println!("nombre: {} apellido: {} dni: {}", persona1.nombre, persona1.apellido, persona1.dni);
}
```

### Creando instancias desde otra instancia (Struct Update Syntax)
Se puede crear una nueva instancia usando los valores de otra, sobrescribiendo los campos que se deseen.

```rust
fn main() {
    let persona1 = nueva_persona(
        "Lionel".to_string(),
        "Messi".to_string(),
        1
    );
    let persona2 = Persona {
        nombre: "Thiago".to_string(),
        ..persona1
    };
    println!("nombre: {} apellido: {} dni: {}", persona2.nombre, persona2.apellido, persona2.dni);
}
```

### Tuple Structs
Permiten crear estructuras sin nombres de campos, accediendo a ellos mediante índices (`.0`, `.1`, etc.).

```rust
struct Coordenada(f64, f64);

fn main() {
    let la_plata = Coordenada(-34.9213094, -57.9555699);
    println!("latitud: {} longitud: {}", la_plata.0, la_plata.1);
}
```

### Implementando Métodos (Funciones Asociadas)
Se utiliza el bloque `impl` para definir funciones asociadas a la estructura.

```rust
struct Coordenada(f64, f64);

impl Coordenada {
    fn es_la_plata(&self) -> bool {
        let (latitud, longitud) = (-34.9213094, -57.9555699);
        if self.0 == latitud && self.1 == longitud {
            return true;
        }
        false
    }
}

fn main() {
    let la_plata = Coordenada(-34.9213094, -57.9555699);
    println!("es la plata? {}", la_plata.es_la_plata());
}
```

### Otro ejemplo con `derive(Debug)`
Si se quiere imprimir la estructura, se debe implementar el trait `Debug` o derivarlo.

```rust
#[derive(Debug)]
struct Rectangulo {
    ancho: u32,
    altura: u32,
}

impl Rectangulo {
    fn area(&self) -> u32 {
        self.ancho * self.altura
    }
}

fn main() {
    let rec1 = Rectangulo { ancho: 3, altura: 7 };
    println!("rectangulo es: {:?}", rec1);
    println!("el area del rectangulo es: {}", rec1.area());
}
```

### Funciones Asociadas (Constructores)
Todas las funciones definidas dentro de un bloque `impl` se denominan funciones asociadas. Las que no reciben `self` como primer parámetro no son métodos y se suelen usar como constructores. `new` es una convención común, no una palabra reservada.

```rust
impl Rectangulo {
    fn area(&self) -> u32 {
        self.ancho * self.altura
    }

    fn new(ancho: u32, altura: u32) -> Rectangulo {
        Rectangulo { ancho, altura }
    }
}

fn main() {
    let rec1 = Rectangulo::new(3, 7);
    println!("rectangulo es: {:?}", rec1);
    println!("el area del rectangulo es: {}", rec1.area());
}
```

---

## Enums

### Definición y uso básico
`enum` es un tipo de dato que permite definir distintas variaciones.

```rust
enum Rol {
    PADRE,
    HIJO,
}

struct Persona {
    nombre: String,
    apellido: String,
    dni: i32,
    rol: Rol,
}

fn main() {
    let persona = Persona {
        nombre: "Lionel".to_string(),
        apellido: "Messi".to_string(),
        dni: 1,
        rol: Rol::PADRE,
    };
    println!("el rol de: {} es: {:?}", persona.nombre, persona.rol);
}
```

### Enums con valores asociados
Se pueden asociar datos directamente a las variantes del enum.

```rust
enum Rol {
    PADRE(i32),
    HIJO(i32),
}

// ... (Struct Persona igual que antes)

fn main() {
    let persona = Persona {
        nombre: "Lionel".to_string(),
        apellido: "Messi".to_string(),
        dni: 1,
        rol: Rol::PADRE(5),
    };

    match persona.rol {
        Rol::PADRE(valor) => println!("Padre con valor: {}", valor),
        _ => (),
    };
}
```

### Enums con Structs internos
Las variantes pueden contener estructuras completas.

```rust
struct StructPadre {}
struct StructHijo {}

impl StructPadre {
    fn hace_algo(self) {
        println!("Soy un padre que hace algo");
    }
}

impl StructHijo {
    fn hace_algo(self) {
        println!("Soy un hijo que hace algo");
    }
}

enum Rol {
    PADRE(StructPadre),
    HIJO(StructHijo),
}

impl Rol {
    fn hace_algo(self) {
        match self {
            Rol::PADRE(instancia) => instancia.hace_algo(),
            Rol::HIJO(instancia) => instancia.hace_algo(),
        }
    }
}

fn main() {
    let persona = Persona {
        nombre: "Lionel".to_string(),
        apellido: "Messi".to_string(),
        dni: 1,
        rol: Rol::PADRE(StructPadre {}),
    };
    persona.rol.hace_algo();
}
```

---

## Option

### ¿Qué es?
`Option` es un enum disponible en la librería estándar. Tiene dos variantes: `Some(T)` y `None`.
Rust obliga a manejar explícitamente la ausencia de valor para evitar errores de tipo `NullPointerException`.

```rust
enum Option<T> {
    Some(T),
    None,
}
```

### Ejemplo I: Uso básico con `Option<i32>`

```rust
struct Persona {
    nombre: String,
    apellido: String,
    dni: Option<i32>,
    rol: Rol,
}

impl Persona {
    fn new(nombre: String, apellido: String, rol: Rol, dni: Option<i32>) -> Persona {
        Persona {
            nombre,
            apellido,
            dni,
            rol,
        }
    }
}

fn main() {
    let nombre = "Lionel".to_string();
    let apellido = "Messi".to_string();
    let rol = Rol::PADRE(StructPadre {});
    
    // Caso 1: Alguno no tiene DNI
    let dni = None;
    let persona = Persona::new(nombre, apellido, rol, dni);
    println!("La persona: {} tiene el dni: {:?}", persona.apellido, persona.dni);
    
    // Caso 2: Con DNI
    let dni = Some(1);
    let persona = Persona::new("Lionel".to_string(), "Messi".to_string(), rol, dni);
    match persona.dni {
        Some(valor) => println!("El dni de {} es: {}", persona.apellido, valor),
        None => println!("{} no tiene nro de dni registrado", persona.apellido),
    }
}
```

### Ejemplo II: `Option` con otro Struct

```rust
struct DNI {
    tipo: char,
    nro: u32,
}

struct Persona {
    nombre: String,
    apellido: String,
    dni: Option<DNI>,
    rol: Rol,
}

impl Persona {
    fn new(nombre: String, apellido: String, rol: Rol, dni: Option<DNI>) -> Persona {
        Persona { nombre, apellido, dni, rol }
    }
}

fn main() {
    let nombre = "Lionel".to_string();
    let apellido = "Messi".to_string();
    let rol = Rol::PADRE(StructPadre {});
    let dni = Some(DNI { tipo: 'A', nro: 1 });
    let persona = Persona::new(nombre, apellido, rol, dni);

    // Con match
    match persona.dni {
        Some(valor) => println!("El dni de {} es: {}", persona.apellido, valor.nro),
        None => println!("{} no tiene nro de dni registrado", persona.apellido),
    }

    // Con is_none() y unwrap()
    if persona.dni.is_none() {
        println!("{} no tiene nro de dni registrado", persona.apellido);
    } else {
        println!("El dni de {} es: {:?}", persona.apellido, persona.dni.unwrap().nro);
    }
}
```

### Sintaxis `if let`
Permite ejecutar código solo si el patrón coincide.

```rust
if let Some(data) = persona.dni {
    println!("El dni de {} es: {}", persona.apellido, data.nro);
} else {
    println!("{} no tiene nro de dni registrado", persona.apellido);
}
```

### Sintaxis `let else`
Permite extraer el valor o salir del ámbito actual si es `None`.

```rust
let Some(data) = persona.dni else {
    panic!("{} no tiene dni", persona.apellido);
};
```

### Sintaxis `while let`
Permite ejecutar un bucle mientras el patrón coincida.

```rust
fn main() {
    let mut cantidad = Some(5);
    loop {
        match cantidad {
            Some(valor) => {
                if valor > 0 {
                    println!("{}", valor);
                    cantidad = Some(valor - 1);
                } else {
                    cantidad = None;
                }
            },
            None => { break; }
        }
    }
}
```

---

## Collections (Primera Parte)

### Sequences
- **Vec**: Vector dinámico.
- **VecDeque**: Cola de doble extremo.
- **LinkedList**: Lista enlazada.

### Maps
- **HashMap**: Mapa basado en hash.
- **BTreeMap**: Mapa basado en árbol B.

---

### Sequences: Vec

#### Creación y agregado de elementos

```rust
fn main() {
    // Creación
    let mut vector = Vec::new();

    // Agregando elementos
    for i in 1..7 {
        vector.push(i);
    }

    // Recorriéndolo
    for j in vector {
        println!("{}", j);
    }
}
```

#### Accediendo a elementos

```rust
fn main() {
    let mut vector = Vec::new();
    for i in 10..18 {
        vector.push(i);
    }

    // Para acceder al primer elemento
    let primero = vector.first();
    if let Some(elemento) = primero {
        println!("El primer elemento es: {}", elemento);
    }
    // Acceso por índice (puede pánico si el índice está fuera de rango)
    println!("También puedo acceder desde el índice: {}", vector[0]);

    // Para acceder al último elemento
    let ultimo = vector.last();
    if let Some(elemento) = ultimo {
        println!("El último elemento es: {}", elemento);
    }
    println!("También puedo acceder desde el índice: {}", vector[vector.len() - 1]);
}
```

#### Agregando y modificando elementos

```rust
fn main() {
    let mut vector = Vec::new();
    for i in 10..18 {
        vector.push(i);
    }

    // Otra forma de agregar elementos (extend)
    let arreglo = [1, 2, 3];
    vector.extend(arreglo);
    println!("El último elemento es: {}", vector[vector.len() - 1]);

    // Modificando elementos
    for i in 1..vector.len() {
        vector[i] += 4;
    }
    println!("{:?}", vector);
}
```

#### Eliminando elementos y simulando una pila

```rust
fn main() {
    let mut vector = Vec::new();
    for i in 1..7 {
        vector.push(i);
    }
    println!("{:?}", vector);

    // Eliminando un elemento de determinado índice
    vector.remove(1);
    println!("{:?}", vector);

    // Simulando una pila
    let elemento = vector.pop(); // pop() retorna Option<T>
    if let Some(desapilado) = elemento {
        println!("Desapilo el: {}", desapilado);
    }
    while let Some(desapilado) = vector.pop() {
        println!("Desapilo el: {}", desapilado);
    }
}
```

#### Otras formas de instanciar Vec

```rust
fn main() {
    let vector: Vec<i32> = vec![1];
    let otro_vector = vec![1, 2, 3];
    let otro_mas_vector = vec![0; 5]; // Vector con 5 ceros

    println!("{:?}", vector);
    println!("{:?}", otro_vector);
    println!("{:?}", otro_mas_vector);
}
```

---

### Sequences: VecDeque

Es una cola de doble extremo. Se puede agregar al final (`push_back`), al principio (`push_front`) y sacar del final y del principio.

```rust
use std::collections::VecDeque;

fn main() {
    let mut buf = VecDeque::new();
    for i in 1..5 {
        buf.push_back(i);
    }
    // [1, 2, 3, 4]
    for i in 10..15 {
        buf.push_front(i);
    }

    if let Some(numero) = buf.pop_front() {
        println!("{}", numero);
    }
    if let Some(numero) = buf.pop_back() {
        println!("{}", numero);
    }
}
```

#### Accediendo y modificando datos

```rust
use std::collections::VecDeque;

fn main() {
    let mut deque: VecDeque<u32> = VecDeque::with_capacity(10);
    for i in 1..3 {
        deque.push_front(i);
    }

    // Puedo acceder por índice
    println!("{}", deque[0]);
    // Puedo acceder por método get
    if let Some(valor) = deque.get(0) {
        println!("{}", valor);
    }

    // Modificando directamente por posición
    deque[0] = 22;
    // Modificando a través de get_mut
    if let Some(elem) = deque.get_mut(1) {
        *elem = 7;
    }
    println!("{:?}", deque);
}
```

---

### Sequences: LinkedList

```rust
use std::collections::LinkedList;

fn main() {
    let mut list1 = LinkedList::new();
    for i in 1..3 {
        list1.push_back(i);
    }
    for i in 3..7 {
        list1.push_front(i);
    }
}
```

#### Operaciones más importantes

```rust
// list1.back();              // retorna un Option con el último elemento si existe
// list1.front();             // retorna un Option con el primer elemento si existe
// list1.back_mut();          // retorna un Option con el último elemento mutable si existe
// list1.front_mut();         // retorna un Option con el primer elemento mutable si existe
// list1.clone();             // clona la lista en una nueva lista
// list1.contains(&4);        // retorna un boolean si contiene o no el elemento
// list1.is_empty();          // retorna un boolean si está o no vacía
// list1.len();               // retorna la longitud de elementos de la lista
// list1.pop_back();          // retorna el último elemento eliminándolo de la lista
// list1.pop_front();         // retorna el primer elemento eliminándolo de la lista
// list1.clear();             // limpia toda la lista y la deja vacía
```

---

### Maps: HashMap

#### Creación y agregado de elementos

```rust
use std::collections::HashMap;

fn main() {
    let mut balances = HashMap::new();
    balances.insert(1, 10.0);
    balances.insert(2, 0.0);
    balances.insert(3, 150_000.0);
    balances.insert(4, 2_000.0);

    for (id, balance) in balances {
        println!("{} tiene $ {}", id, balance);
    }
}
```

#### Obtener y modificar un elemento

```rust
use std::collections::HashMap;

fn main() {
    let mut balances: HashMap<i32, f64> = HashMap::new();
    balances.insert(1, 10.0);
    balances.insert(2, 0.0);

    let id = 2;
    let balance: Option<&mut f64> = balances.get_mut(&id);
    match balance {
        Some(balance) => *balance = *balance + 12.0,
        None => (),
    }
    
    if let Some(balance) = balances.get(&id) {
        println!("{}", balance);
    }
}
```

#### Otra forma de construir

```rust
use std::collections::HashMap;

fn main() {
    let balances = HashMap::from([
        (1, 10.0),
        (2, 0.0),
    ]);

    // Obtener solo las claves
    for id in balances.keys() {
        println!("{}", id);
    }
    // Obtener solo los valores
    for val in balances.values() {
        println!("{}", val);
    }
}
```

#### Otros métodos importantes

```rust
// balances.remove(&3);               // Elimina la clave-valor y retorna un Option con el valor
// balances.values_mut();             // Retorna los valores para poder modificarlos
// balances.get_key_value(&1);        // Retorna un Option con el par clave-valor
// balances.contains_key(&5);         // Retorna un bool
// balances.entry(5).or_insert(0.0);  // Inserta la clave-valor solo si no existe
// balances.len();
// balances.is_empty();
// balances.clear();
```

---

### Maps: BTreeMap

Los `BTreeMap` a diferencia de los `HashMap` están optimizados para búsquedas ordenadas (Árbol Binario) y su interfaz es similar. Las operaciones son prácticamente las mismas.

```rust
use std::collections::BTreeMap;

fn main() {
    let mut balances = BTreeMap::from([
        (1, 10.0),
        (2, 0.0),
    ]);

    // Operaciones extra de BTreeMap
    // balances.pop_first();           // Remueve y retorna un Option con el elemento de clave más pequeña
    // balances.pop_last();            // Remueve y retorna un Option con el elemento de clave más grande
    // balances.first_key_value();     // Retorna el primer par clave-valor
    // balances.last_key_value();      // Retorna el último par clave-valor
}
```

---

## Alcance y Visibilidad (Módulos y `pub`)

Para definir a un elemento (fn, struct, enum, mod) como **público**, se utiliza la palabra clave `pub` delante de su definición, lo que indica que puede ser accedido desde fuera de donde fue declarado. En caso contrario, será privado y solo podrá accederse dentro del ámbito donde fue definido.

```rust
// ejemplo.rs

mod crate_helper_module {
    // Esta función solo puede ser accedida en este rs
    pub fn crate_helper() {}

    // Esta función solo puede ser accedida desde el scope de crate_helper_module
    fn implementation_detail() {}
}

// Esta es una función pública que puede ser accedida desde cualquier lugar, incluso externamente
pub fn public_api() {}
```

### Submódulos y `super`

```rust
// ejemplo.rs

// Igual a public api
pub mod submodule {
    use crate::ejemplo::crate_helper_module;

    pub fn my_method() {
        crate_helper_module::crate_helper();
    }

    // Solo puede accederse en el scope de submodule
    fn my_implementation() {}

    #[cfg(test)]
    mod test {
        #[test]
        fn test_my_implementation() {
            // Con super accedo jerárquicamente un nivel arriba de definiciones del módulo
            super::my_implementation();
        }
    }
}
```