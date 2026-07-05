# Clase 4: Collections (Parte 2) y Generics

## Temario
- **Collections (Segunda Parte):**
  - Sets: `HashSet`, `BTreeSet`
  - Extra: `BinaryHeap`
  - Conclusiones
- **Generics**

---

## Set: HashSet

### ¿Qué son?
Es un `HashMap` con la diferencia fundamental de que **no tiene valores** asociados; solo almacena claves únicas.

```rust
use std::collections::HashSet;

fn main() {
    let mut ids = HashSet::from([1, 2, 3]);
    let mut otros_ids = HashSet::from([10, 2, 30]);

    ids.insert(4);

    // Operaciones de conjuntos
    let diferencia: HashSet<_> = ids.difference(&otros_ids).collect();
    let interseccion: HashSet<_> = ids.intersection(&otros_ids).collect();
    let union: HashSet<_> = ids.union(&otros_ids).collect();

    println!("Diferencia: {:?}", diferencia);
    println!("Intersección: {:?}", interseccion);
    println!("Unión: {:?}", union);

    // Otras operaciones
    ids.remove(&3);
    println!("Tamaño: {}", ids.len());
    println!("¿Está vacío?: {}", ids.is_empty());
}
```
> *Más información: [https://doc.rust-lang.org/std/collections/hash_set/struct.HashSet.html](https://doc.rust-lang.org/std/collections/hash_set/struct.HashSet.html)*

---

## Set: BTreeSet

### ¿Qué son?
Es un conjunto **ordenado** basado en Árboles Binarios (B-Tree). Almacena sus elementos en orden ascendente.

```rust
use std::collections::BTreeSet;

fn main() {
    let mut ids = BTreeSet::from([1, 2, 3]);
    for id in &ids {
        println!("{id}");
    }
    
    // Tiene las mismas operaciones que HashSet (diferencia, intersección, unión, etc.)
}
```
> *Más información: [https://doc.rust-lang.org/std/collections/struct.BTreeSet.html](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html)*

---

## Extra: BinaryHeap

### ¿Qué es?
Es una cola de prioridad implementada como un montículo binario (*binary heap*). Por defecto funciona como un **max-heap** (el elemento más grande tiene la prioridad más alta).

```rust
use std::collections::BinaryHeap;

fn main() {
    let mut max_heap = BinaryHeap::from([1, 2, 3]);
    max_heap.push(4);
    
    // Imprime los elementos (no necesariamente en orden, sino como una estructura interna)
    println!("max heap: {:?}", max_heap);
    
    if let Some(e) = max_heap.pop() { // Saca el más grande (4)
        println!("Pop: {}", e);
    }
    
    if let Some(e) = max_heap.peek() { // Mira el más grande sin sacarlo (3)
        println!("Peek: {}", e);
    }
    
    println!("max heap: {:?}", max_heap);
}
```

### BinaryHeap: ¿Y el min-heap?
Para utilizar un **min-heap** (donde el elemento más pequeño tiene la prioridad más alta), se puede envolver el valor con el tipo `Reverse`.

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    let mut min_heap = BinaryHeap::new();
    min_heap.push(Reverse(1));
    min_heap.push(Reverse(2));
    min_heap.push(Reverse(3));
    
    println!("min heap: {:?}", min_heap);
    
    if let Some(e) = min_heap.pop() { // Saca el más pequeño (1)
        println!("Pop: {}", e.0);
    }
    
    if let Some(e) = min_heap.peek() { // Mira el más pequeño (2)
        println!("Peek: {}", e.0);
    }
    
    println!("min heap: {:?}", min_heap);
}
```
> *Más información: [https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html](https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html)*

---

## ¿Cuándo usar cada estructura?

### ¿Cuándo usar `Vec`?
- Quiero recopilar elementos para procesarlos o enviarlos a otro lugar más adelante, y no me importan las propiedades de los valores reales que se almacenan.
- Quiero una secuencia de elementos en un orden particular, y solo se agrega al final (o cerca de él).
- Quiero el comportamiento de una pila (*stack*).
- Quiero un arreglo dinámico.
- Quiero un arreglo con manejo en memoria *heap*.

### ¿Cuándo usar `VecDeque`?
- Quiero un `Vec` que admita una inserción eficiente en ambos extremos.
- Quiero manejar la estructura de una cola (*queue*).
- Quiero una cola de dos extremos (*deque*).

### ¿Cuándo usar `LinkedList`?
- Quiero dividir y agregar listas de manera eficiente.
- Estás absolutamente seguro de que realmente querés una lista doblemente enlazada.

### ¿Cuándo usar `HashMap`?
- Deseas asociar claves arbitrarias con un valor arbitrario.
- Querés un caché.
- Querés un *map*, sin funcionalidad adicional.

### ¿Cuándo usar `BTreeMap`?
- Quiero un *map* ordenado por sus claves.
- Me interesa saber cuál es el par clave-valor más pequeño o más grande.

### ¿Cuándo usar *Set* (`HashSet` / `BTreeSet`)?
- Solo querés un conjunto con sus propiedades de membresía y operaciones de teoría de conjuntos (intersección, unión, diferencia).

### ¿Cuándo usar `BinaryHeap`?
- Deseas almacenar un montón de elementos, pero solo querés procesar el "más grande" o "más importante" en un momento dado.
- Querés una cola de prioridad.

---

## Performance (Rendimiento)

### Secuencias
*Nota: Donde ocurren empates, `Vec` suele ser más rápido que `VecDeque`, y `VecDeque` suele ser más rápido que `LinkedList`.*

| Operación       | `Vec`                | `VecDeque`           | `LinkedList`        |
|-----------------|----------------------|----------------------|----------------------|
| `get(i)`        | O(1)                 | O(1)                 | O(min(i, n-i))       |
| `insert(i)`     | O(n-i)*              | O(min(i, n-i))*      | O(min(i, n-i))       |
| `remove(i)`     | O(n-i)*              | O(min(i, n-i))*      | O(min(i, n-i))       |
| `append`        | O(m)*                | O(m)*                | O(min(i, n-i))       |
| `split_off(i)`  | O(n-i)               | O(min(i, n-i))       | O(min(i, n-i))       |

### Maps & Sets
*Para los Sets, todas las operaciones tienen el costo de la operación equivalente en un Map.*

| Operación       | `HashMap`            | `BTreeMap`           |
|-----------------|----------------------|----------------------|
| `get`           | O(1)~                | O(log(n))            |
| `insert`        | O(1)~*               | O(log(n))            |
| `remove`        | O(1)~*               | O(log(n))            |
| `range`         | N/A                  | O(log(n))            |
| `append`        | N/A                  | n+m                  |

---

## Generics

### ¿Para qué sirven?
El tipo *generic* (genérico) se utiliza para generalizar implementaciones, permitiendo mayor flexibilidad en el código al poder trabajar con múltiples tipos sin tener que reescribir la lógica.

### Ejemplo I: `struct Punto<T>`

**Problema inicial (sin genéricos):**
```rust
#[derive(Debug)]
struct Punto {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Punto { x: 1, y: 2 };
    // Error: mismatched types -> se esperaba un entero y se recibe un flotante
    let p2 = Punto { x: 10.5, y: 2 }; 
}
```

**Solución con un parámetro genérico `<T>`:**
```rust
struct Punto<T> {
    x: T,
    y: T,
}

fn main() {
    let p1 = Punto { x: 1, y: 2 };
    // Error: mismatched types -> ambos campos deben ser del mismo tipo T
    let p2 = Punto { x: 10.5, y: 2 }; 
}
```

**Solución con dos parámetros genéricos `<T, V>` (correcta):**
```rust
#[derive(Debug)]
struct Punto<T, V> {
    x: T,
    y: V,
}

fn main() {
    let p1 = Punto { x: 1, y: 2 };
    let p2 = Punto { x: 10.5, y: 2 };
    println!("{:?}", p1);
    println!("{:?}", p2);
}
```

**Advertencia (Genéricos anidados):**
```rust
#[derive(Debug)]
struct Punto<T> {
    x: T,
    y: T,
}

fn main() {
    let p1 = Punto { x: 1, y: 2 };
    let p2 = Punto { x: 10, y: 2 };
    // p_esp contendrá p1 y p2, los cuales son structs Punto<i32>
    let p_esp = Punto { x: p1, y: p2 };
    println!("{:?}", p_esp);
}
```
> *Nota: En el ejemplo anterior, `Punto<T>` debe implementar `Copy` o `Clone` para que `p1` y `p2` puedan moverse dentro de `p_esp`, o bien pasarse por referencia, dependiendo de la semántica de movimiento.*

### Ejemplo II: Caja Genérica `<T>`

**Problema sin genéricos:**
Solo funciona para tipos `i32`.

```rust
struct Caja {
    dato: i32,
    estado: bool,
}

impl Caja {
    fn new(dato: i32) -> Caja {
        Caja { dato, estado: false }
    }
    fn abrir(&mut self) -> i32 {
        self.estado = true;
        self.dato
    }
    fn cerrar(&mut self) {
        self.estado = false;
    }
}

fn main() {
    let mut caja = Caja::new(9);
    caja.abrir();
    caja.cerrar();
}
```

**Error al intentar usarlo con otro tipo:**
```rust
fn main() {
    let mut listado_de_compras = Vec::new();
    listado_de_compras.push((1, "Jabon de manos"));
    listado_de_compras.push((2, "Detergente"));
    // Error: expected 'i32', found struct 'Vec'
    let mut caja = Caja::new(listado_de_compras); 
}
```

**Solución final con Genéricos `<T>`:**
```rust
struct Caja<T> {
    dato: T,
    estado: bool,
}

impl<T> Caja<T> {
    fn new(dato: T) -> Caja<T> {
        Caja { dato, estado: false }
    }
    fn abrir(&mut self) -> &T {
        self.estado = true;
        &self.dato
    }
    fn cerrar(&mut self) {
        self.estado = false;
    }
}

fn main() {
    let mut caja1 = Caja::new(9);
    caja1.abrir();
    println!("Caja1 abierta: {:?}", caja1.estado);

    let mut listado_de_compras = Vec::new();
    listado_de_compras.push((1, "Jabon de manos"));
    let mut caja2 = Caja::new(listado_de_compras);
    caja2.abrir();
    println!("Caja2 abierta: {:?}", caja2.estado);
}
```