# Clase 6: Iterators, Manejo de errores, Prelude y Archivos

## Temario
- Iterators
- Manejo de errores
- Prelude
- Archivos

---

## Iterators

### ¿Qué es?
Iterator es un patrón de diseño de comportamiento que te permite recorrer elementos de una colección sin exponer su representación subyacente (lista, pila, árbol, etc.).

### Iterator en Rust
Rust implementa el trait `Iterator` para sus colecciones, permitiendo que se usen como tales.

```rust
#[derive(Debug)]
struct Algo {
    d: i32,
}

impl Default for Algo {
    fn default() -> Self {
        Algo { d: 0 }
    }
}

fn main() {
    use std::collections::LinkedList;
    use std::collections::HashMap;

    let mut v = vec![Algo::default(), Algo::default(), Algo::default(), Algo::default()];
    let mut l = LinkedList::from([Algo::default(), Algo::default(), Algo::default(), Algo::default()]);
    let mut hm = HashMap::from([
        (1, Algo::default()),
        (2, Algo::default()),
        (3, Algo::default())
    ]);

    let mut iter_v = v.iter();
    let mut iter_l = l.iter();
    let mut iter_hm = hm.iter();

    // Métodos de Iterator
    let iter_v_clone = iter_v.clone();
    iter_v.next();
    iter_v.cycle();
    iter_v.enumerate();
    iter_v.take(3);
    iter_v.step_by(3);
    iter_v.skip(2);

    let otro = iter_v_clone.chain(iter_l);
    for i in otro {
        println!("{:?}", i);
    }
}
```

### Iterator y Closures
Los métodos de `Iterator` aceptan closures para filtrar o transformar datos.

```rust
iter_v.all(closure);          // Retorna true si todos los elementos cumplen la condición
iter_v.any(closure);          // Retorna true si al menos uno cumple la condición
iter_v.filter(closure);       // Filtra los elementos que cumplen la condición
iter_v.filter_map(closure);   // Filtra y mapea en un solo paso
iter_v.skip_while(closure);   // Salta elementos mientras se cumpla la condición
```

### Implementando Iterator en un Struct personalizado

Podemos implementar el trait `Iterator` manualmente para nuestras propias estructuras.

```rust
struct Caja {
    c: i32,
}

impl Default for Caja {
    fn default() -> Self {
        Caja { c: 0 }
    }
}

impl Iterator for Caja {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.c < 10 {
            self.c += 1;
            return Some(self.c);
        }
        None
    }
}

fn main() {
    let mut a = Caja::default();
    while let Some(v) = a.next() {
        println!("{}", v);
    }
}
```

---

## Manejo de errores

Rust agrupa los errores en dos categorías:

1.  **Recuperables:** Por ejemplo, un archivo no encontrado. El error se informa pero el programa puede continuar.
2.  **Irrecuperables:** Señalan bugs, como acceder a una posición inválida de un arreglo.

Rust no tiene excepciones. En su lugar, usa el tipo `Result<T, E>` para errores recuperables y la macro `panic!` para errores irrecuperables.

### Panic!

```rust
fn main() {
    let v = vec![1, 2, 3];
    v[v.len()]; // Intencionalmente fuera de rango
}

// Error en ejecución:
// thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 3'
```

#### Invocando `panic!` manualmente

```rust
fn verify(data: Vec<Data>) {
    if data.is_empty() {
        panic!("No hay data para procesar y es obligatorio");
    }
    // hace más cosas
}
```

### Result

El tipo `Result<T, E>` permite manejar errores recuperables.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

#### Usando `Result` con `stdin()`

```rust
fn main() {
    use std::io::stdin;
    let mut buf = String::new();
    let result = stdin().read_line(&mut buf);

    match result {
        Ok(i) => procesar_entrada(i),
        Err(e) => procesar_error(e),
    }
}
```

#### Usando `Result` con `parse()`

```rust
fn main() {
    let a = "123".to_string();
    let result = a.parse::<i32>();

    match result {
        Ok(data) => println!("El parseo fue correcto: {}", data),
        Err(e) => println!("String inválido para parsear a i32: {}", e),
    }
}
```

#### El operador `?` (Question Mark)
El operador `?` propaga el error automáticamente si falla, o extrae el valor si tiene éxito.

```rust
struct Persona {
    trabajo: Option<Trabajo>,
}

#[derive(Clone, Copy)]
struct Trabajo {
    telefono: Option<NumeroTelefono>,
}

#[derive(Clone, Copy)]
struct NumeroTelefono {
    codigo_de_area: Option<u8>,
    numero: u32,
}

impl Persona {
    fn codigo_area(&self) -> Option<u8> {
        // Si alguno es None, el resultado será None
        self.trabajo?.telefono?.codigo_de_area
    }
}
```

#### `panic!` o `expect()`
Si prefieres que un error irrecuperable detenga el programa, puedes usar `unwrap()` o `expect()`.

```rust
fn conversion_de_tipo(s: String) -> i32 {
    let dato = s.parse::<i32>().expect("Mensaje para el panic");
    // hago algo con dato
    return dato;
}

fn main() {
    let dato = conversion_de_tipo("1".to_string());
    // más instrucciones
}
```

### Errores Custom (Personalizados)

Podemos definir nuestros propios tipos de error implementando el trait `Display`.

```rust
use std::fmt::Display;

struct MiError;

impl Display for MiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Estoy disparando un error custom")
    }
}
```

#### Error Custom con contexto

```rust
struct MiError(String);

impl Display for MiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "El num: {} no es válido!", self.0)
    }
}

fn validar_numero(num: i32) -> Result<i32, MiError> {
    if num > 10 {
        return Err(MiError(num.to_string()));
    }
    Ok(num)
}

fn main() {
    let n = 10;
    let r = validar_numero(n);
    match r {
        Ok(v) => println!("El num:{} es correcto!", v),
        Err(e) => println!("Error: {}", e),
    }
}
```

### Ejemplo completo de Errores Custom (Roles y Permisos)

```rust
use std::fmt::Display;

#[derive(Debug)]
enum Rol {
    Admin,
    Staff,
}

#[derive(Debug)]
struct Usuario {
    username: String,
    rol: Rol,
}

impl Usuario {
    fn es_staff(&self) -> bool {
        match self.rol {
            Rol::Staff => true,
            _ => false,
        }
    }
    fn to_string(&self) -> String {
        self.username.clone()
    }
}

#[derive(Debug)]
struct Movimiento<'a> {
    usuario: &'a Usuario,
    producto_id: u8,
    tipo: String,
}

enum Estado {
    INI,
    ACT,
    FIN,
}

impl Estado {
    fn estado_siguiente(&self) -> &Estado {
        match self {
            Estado::INI => &Estado::ACT,
            Estado::ACT => &Estado::FIN,
            Estado::FIN => &Estado::FIN,
        }
    }
    fn es_ini(&self) -> bool {
        match self {
            Estado::INI => true,
            _ => false,
        }
    }
    fn to_string(&self) -> String {
        match self {
            Estado::INI => "INI".to_string(),
            Estado::ACT => "ACT".to_string(),
            Estado::FIN => "FIN".to_string(),
        }
    }
    fn cambiar_estado<'a>(&self, usuario: &'a Usuario) -> Result<&Estado, PermisoError> {
        let result = Permisos::puede_cambiar_estado(self, self.estado_siguiente(), usuario);
        match result {
            Ok(_) => Ok(self.estado_siguiente()),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug, Clone)]
struct PermisoError(String, String);

impl Display for PermisoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "El usuario: {} no posee los permisos para realizar el cambio de estado a: {}", self.0, self.1)
    }
}

struct Permisos;

impl Permisos {
    fn puede_cambiar_estado<'a>(estado_actual: &'a Estado, estado_siguiente: &Estado, usuario: &Usuario) -> Result<&'a Estado, PermisoError> {
        if usuario.es_staff() && estado_actual.es_ini() {
            return Err(PermisoError(usuario.to_string(), estado_siguiente.to_string()));
        }
        Ok(estado_actual)
    }
}

struct Producto<'a> {
    nombre: String,
    estado: &'a Estado,
    id: u8,
}

impl<'a> Producto<'a> {
    fn cambiar_estado(&mut self, usuario: &'a Usuario) -> Result<Movimiento<'a>, PermisoError> {
        self.estado = self.estado.cambiar_estado(usuario)?;
        let m = Movimiento {
            usuario,
            producto_id: self.id,
            tipo: "Cambio de estado".to_string(),
        };
        Ok(m)
    }
    fn get_estado(&self) -> &Estado {
        self.estado
    }
}

fn main() {
    let u_s = Usuario { username: "staff@staff.com".to_string(), rol: Rol::Staff };
    let u_a = Usuario { username: "admin@admin.com".to_string(), rol: Rol::Admin };
    let mut p = Producto { nombre: "pi".to_string(), id: 1, estado: &Estado::INI };

    let r = p.cambiar_estado(&u_s);
    match r {
        Ok(mov) => println!("Movimiento creado: {:#?}", mov),
        Err(e) => println!("Error: {}", e),
    }

    let r = p.cambiar_estado(&u_a);
    match r {
        Ok(mov) => println!("Movimiento creado: {:#?}", mov),
        Err(e) => println!("Error: {}", e),
    }
}
```

---

## Prelude

El **Prelude** es la lista de "cosas" que Rust importa automáticamente en cada programa. Se mantiene lo más pequeño posible y hace foco en los traits que se usan en la mayoría de los programas.

```rust
// https://doc.rust-lang.org/std/prelude/index.html
use std::collections::HashMap;

fn main() {
    let o: Option<i32> = None;
    let r: Result<i32, std::num::ParseIntError>;
    let v: Vec<i32>;
    let s: String;
    let algo: HashMap<i32, i32> = HashMap::new();
}
```

---

## Archivos

El struct `File` representa un archivo abierto (envuelve un *file descriptor*) y da acceso de lectura y/o escritura al archivo subyacente. Dado que pueden suceder muchas cosas mal en la E/S de archivos, todos los métodos de archivo devuelven el tipo `io::Result<T>`, que es un alias para `Result<T, io::Error>`. Esto hace que la falla de todas las operaciones de E/S sean explícitas.

### Archivos: Abrir y leer (`open`)

```rust
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = "src/archivo1.txt";

    // Abre el archivo del path en modo lectura, retorna io::Result<File>
    let mut archivo = match File::open(path) {
        Err(e) => panic!("No se pudo abrir por: {}", e),
        Ok(archivo) => archivo,
    };

    // Lee el contenido en un string, retorna io::Result<usize>
    let mut s = String::new();
    match archivo.read_to_string(&mut s) {
        Err(e) => panic!("No se puede leer por: {}", e),
        Ok(_) => print!("Contiene:\n{}", s),
    }

    // Cuando termina el scope, el archivo se cierra automáticamente
}
```

### Archivos: Crear y escribir (`create`)

```rust
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = "src/archivo2.txt";

    // Abre el archivo en modo solo escritura, retorna io::Result<File>
    let mut archivo = match File::create(path) {
        Err(e) => panic!("No se puede crear porque: {}", e),
        Ok(archivo) => archivo,
    };

    // Escribe un string al archivo, retorna io::Result<()>
    match archivo.write_all("Limpieza total".as_bytes()) {
        Err(e) => panic!("No puede escribir porque: {}", e),
        Ok(_) => println!("Escribió correctamente en: {}", path),
    };
}
```

### Archivos: Leer línea por línea (`read_lines` con `BufReader`)

```rust
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let path = "src/archivo1.txt";
    let archivo = File::open(path).unwrap();

    let mut lineas = BufReader::new(archivo).lines();

    for linea in lineas {
        println!("{:?}", linea);
    }
}
```