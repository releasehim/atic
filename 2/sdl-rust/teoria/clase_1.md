# Seminario Rust 2026 — Clase 1: Introducción al Lenguaje

## 1. Presentación y Metodología del Curso

### Dinámica de la Cursada

*

**Clases:** Instancias teóricas y prácticas. Las primeras clases se enfocarán fuertemente en definiciones y conceptos arquitectónicos del lenguaje antes de pasar de lleno a la resolución de problemas.

* **Comunicación y consultas:** Centralizadas a través del servidor oficial de **Discord**.
* **Evaluación:**
*

**2 entregas de ejercicios prácticos** realizados de manera individual.

*

**1 Trabajo final** de carácter grupal.

* **Cuestionarios teóricos:** Se habilitará un cuestionario conceptual después de cada clase a través de la plataforma, con límite de entrega hasta la siguiente clase teórica.

*

**Material didáctico:** Todo el contenido estará disponible en la plataforma **Moodle**.

---

## 2. ¿Qué es Rust?

>
> **Definición fundamental:** Rust es un lenguaje de programación multiparadigma, compilado y de código abierto que se centra de manera estricta en la seguridad, la concurrencia y el rendimiento.
>
>

### Desglose de Conceptos Clave

* **Multiparadigma:** Soporta e integra múltiples estilos de programación (como el paradigma funcional y la orientación a objetos).
* **Código Abierto (Open Source):** Su código fuente es público, permitiendo la auditoría y colaboración comunitaria a diferencia de los entornos de código cerrado.
* **Concurrencia:** Capacidad de dividir una tarea en subtareas para ejecutarlas de forma paralela (paralelismo). Rust destaca en este ámbito al proveer mecanismos seguros para compartir memoria y evitar conflictos sobre los recursos de la máquina.
*

**Compilado:** El código fuente de alto nivel escrito por el desarrollador es traducido íntegramente por un **compilador** (`rustc`) a código máquina (instrucciones binarias nativas). Esto genera un **archivo ejecutable independiente** que corre sin intermediarios en la plataforma objetivo.

---

## 3. Gestión y Seguridad de Memoria: El Paradigma de Rust

El diseño de Rust está pensado para ayudar a escribir código seguro y eficiente mediante un **manejo de memoria en tiempo de compilación sin recolector de basura (Garbage Collector)**.

### El Problema de la Memoria en otros Lenguajes

En la mayoría de los lenguajes, a medida que un programa se ejecuta, van quedando "cajoncitos" de memoria con datos basura (referencias que ya no se usarán). Si esto no se limpia, el programa agota la memoria RAM de la máquina.

Para solucionar esto, los lenguajes tradicionales toman dos caminos:

1. **Limpieza Manual (Ej: C/C++):** El desarrollador gestiona la memoria a mano. Es propenso a errores humanos catastróficos.
2. **Recolector de Basura / Garbage Collector (Ej: Java):** Un proceso asíncrono analiza y limpia la memoria en tiempo de ejecución.

### El Costo del Garbage Collector (*Overhead*)

El uso de un Garbage Collector genera una sobrecarga o **overhead** (consumo extra de recursos de la CPU y la RAM) en tiempo de ejecución. El programa se vuelve más pesado y menos performante porque debe ejecutar el recolector en paralelo a la lógica de negocio.

### La Solución de Rust: Desplazar el Costo a la Compilación

Rust elimina el Garbage Collector e implementa un sistema intermedio regido por reglas estrictas de **propiedad y préstamos (*Ownership and Borrowing*)**.

Para entender este balance, podemos evaluar dónde impacta el esfuerzo del sistema:

| Característica | Lenguajes con Garbage Collector (Ej: Java) | Rust |
| --- | --- | --- |
| **Manejo de Memoria** | Automático y asíncrono en ejecución. | Estricto, basado en reglas de diseño en compilación. |
| **Sobrecarga (*Overhead*)** | **En tiempo de ejecución.** Mayor uso de CPU/RAM mientras el programa corre. | **En tiempo de compilación.** El proceso de compilación es más lento y linealmente pesado. |
| **Abstracción** | Alto nivel, pero con un costo asociado de rendimiento. | **Abstracciones de costo cero.** Permite alto nivel con rendimiento nativo. |
| **Eficiencia de Ejecución** | Menor (afectada por las pausas de limpieza). | Máxima y optimizada (similar a C/C++). |

> ⚠️ **Nota de Cursada:** A medida que los proyectos de la materia crezcan, notarán que los tiempos de compilación aumentarán de forma lineal. Este es el "costo" que pagamos a cambio de tener un ejecutable hiper-optimizado en producción.

---

## 4. Características Principales del Lenguaje

*

**Sistema de tipos estático y fuertemente tipado:** El tipo de cada variable debe conocerse en tiempo de compilación y no puede cambiar durante la ejecución.

*

**Seguridad de memoria total:** Garantiza que todas las referencias apunten a direcciones válidas. Esto previene errores históricos como:

*

*Uso después de liberar (Use-after-free)*.

*

*Doble liberación (Double free)*.

*

*Corrupción de memoria*.

*

*Null Pointer Exceptions:* No existen los punteros nulos. El compilador obliga explícitamente al desarrollador a controlar el caso de que "no haya nada".

*

**Sin condiciones de carrera (*Data Races*):** Garantiza que dos partes de un programa no puedan modificar el mismo espacio de memoria en simultáneo.

*

**Mensajes de error descriptivos:** El compilador de Rust está diseñado con un enfoque pedagógico; no solo tipifica y señala con precisión el error, sino que en la gran mayoría de los casos explica detalladamente cómo solucionarlo, haciendo innecesario el uso de asistentes externos o IA.

*

**Gestión de errores explícita:** Utiliza el tipo `Result` para obligar al programador a evaluar tanto el éxito como el fracaso de una operación. El compilador "no deja pasar una" y exige un modelado exhaustivo de todos los escenarios posibles.

---

## 5. Un poco de historia y Contexto de Mercado

*

**Orígenes:** El desarrollo comenzó en el año 2006 dentro de la **Fundación Mozilla** (creadores del navegador Firefox). La versión estable 1.0 se liberó en **2015**, por lo que es considerado un lenguaje moderno.

*

**Propósito Inicial:** Programación de sistemas de bajo nivel (Sistemas operativos, motores de navegación y motores de videojuegos).

* **Adopción Actual:** Hoy en día se está utilizando activamente para migrar partes críticas del Kernel de Linux y de los sistemas de Windows. Grandes empresas tecnológicas lo implementan en infraestructura crítica:
*

**Android y Fuchsia OS (Google):** En componentes base de seguridad.

*

**Meta (Instagram/Facebook):** En optimización de servicios.

*

**Amazon (AWS):** Específicamente para la infraestructura de cómputo de sus servicios EC2.

* **Discord:** Migró componentes core de sus sistemas debido a las exigencias de alta performance.

*

**Salida Laboral:** Aunque el mercado actual es más reducido en comparación con lenguajes históricos, la demanda de perfiles especializados es creciente y la oferta de programadores idóneos es sumamente baja, configurando un excelente *timing* para su adopción profesional.

### Estadísticas de Aceptación en la Comunidad (Stack Overflow)

Rust se ha posicionado como el **lenguaje más admirado y deseado por los desarrolladores por 10 años consecutivos** (hasta el 2025 inclusive). Esto se debe a que las validaciones del compilador garantizan que el código que llega a producción esté virtualmente libre de errores de desarrollo.

| Lenguaje | Deseado (*Desired*) | Admirado (*Admired*) |
| --- | --- | --- |
| Python | 39.3% | 56.4% |
| SQL | 35.6% | 56.4% |
| HTML/CSS | 33.8% | 52.1% |
| JavaScript | 33.5% | 46.8% |
| TypeScript | 31.9% | 58.0% |
| **Rust** | **29.2%** | **72.4%** |
| Bash/Shell | 27.4% | 52.8% |
| Go | 23.4% | 56.5% |

---

## 6. Instalación del Entorno

> 💡 **Recomendación Profesional:** Para el desarrollo de software y las materias correlativas de la carrera, se sugiere fuertemente familiarizarse con entornos Linux o MacOS. Los usuarios de Windows experimentarán un proceso más engorroso, por lo que se recomienda encarecidamente utilizar **WSL (Windows Subsystem for Linux)**.

*

**En Linux / Mac OS:** Ejecutar en la terminal el comando oficial basado en `curl`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

```

*

**En Windows Nativo:** Descargar el instalador ejecutable correspondiente a la arquitectura de la máquina desde la web oficial de Rust.

---

## 7. Sintaxis Esencial y Fundamentos

### Hola Mundo de forma Nativa

A modo pedagógico, se puede compilar un único archivo sin herramientas adicionales.

1. Crear un archivo llamado `main.rs`.

2. Escribir el siguiente bloque de código:

```rust
fn main() {
    // println! es una macro que imprime en consola. Lleva un signo de exclamación.
    [cite_start]println!("Seminario Rust 2026!"); [cite: 90]
}

```

1. Compilar desde la terminal: `rustc main.rs`. Esto genera un archivo ejecutable binario.

2. Ejecutar el binario generado: `./main` (en Linux/Mac) o `main.exe` (en Windows).

### Comentarios

Rust adopta la sintaxis tradicional de comentarios de lenguajes como Java:

```rust
[cite_start]// Esto es un comentario de una sola línea [cite: 99]

/*
   Esto es un comentario de bloque
   [cite_start]apto para múltiples líneas. [cite: 102, 103]
*/

```

---

## 8. Variables, Constantes y Gestión de Datos

### Mutabilidad vs Inmutabilidad

En Rust, **todas las variables son inmutables por defecto**. Una vez asignado un valor, no se puede modificar durante la ejecución a menos que se indique lo contrario de forma explícita.

El compilador realiza **inferencia de tipos en tiempo de compilación**; si asignas un valor al declarar, no es obligatorio explicitar el tipo de dato.

```rust
fn main() {
    let numero = 5; [cite_start]// Inmutable por defecto [cite: 112]
    numero = 6;     [cite_start]// <-- ERROR DE COMPILACIÓN (E0384): cannot assign twice to immutable variable [cite: 123]
}

```

Para alterar o actualizar valores existen dos mecanismos:

#### 1. Variables Mutables (`let mut`)

Permite modificar el valor guardado en la **misma dirección de memoria** del sistema, manteniendo el tipo de dato original.

```rust
fn main() {
    [cite_start]let mut numero = 5; [cite: 143]
    numero = numero + 1; // Totalmente válido
}

```

#### 2. Sombreado de Variables (*Shadowing*)

Consiste en volver a declarar una variable utilizando nuevamente la palabra clave `let` empleando el mismo nombre.

```rust
fn main() {
    [cite_start]let numero = 5; [cite: 136]
    let numero = numero + 1; [cite_start]// Shadowing [cite: 138]
}

```

* **Bajo el capó:** El shadowing **no** modifica la dirección de memoria. Reserva un espacio de memoria completamente nuevo y anula (u oculta) el acceso al espacio anterior. El casillero antiguo queda marcado como libre o inválido para el compilador. El shadowing permite, además, cambiar el tipo de dato final de una variable manteniendo el nombre limpio.

### Constantes (`const`)

Se declaran con la palabra reservada `const`.

*

**Regla estricta:** Es **obligatorio indicar explícitamente el tipo de dato**; el compilador no realiza inferencia en las constantes.

* Son estrictamente inmutables y no admiten *shadowing*. Su valor queda fijado e inalterable durante toda la vida del programa.

```rust
[cite_start]const MI_CONSTANTE: u8 = 10; [cite: 150]

```

---

## 9. Tipos de Datos Escalares (*Scalar Types*)

Representan un único valor simple. Si omitimos el tipo de dato numérico, Rust infiere por defecto los enteros a `i32` y los decimales a `f64`.

### A. Enteros (*Integers*)

Se segmentan de acuerdo a su tamaño en bits y a si aceptan signos negativos (*Signed*) o solo positivos (*Unsigned*).

| Tamaño | Con Signo (*Signed*) | Sin Signo (*Unsigned*) |
| --- | --- | --- |
| 8 bits | `i8` | `u8` |
| 16 bits | `i16` | `u16` |
| 32 bits | `i32` *(Por defecto)* | `u32` |
| 64 bits | `i64` | `u64` |
| 128 bits | `i128` | `u128` |
| Arquitectura del Sistema | `isize` | `usize` |

*

**Literales de diseño:** Rust permite usar guiones bajos en números grandes para mejorar la legibilidad visual en el código fuente (Ej: `let num = 32_500;`). También admite formatos Hexadecimal (`0xff`), Octal (`0o77`), Binario (`0b1111_0000`) y representación de Bytes (`b'A'`).

### B. Punto Flotante (*Floating-Point*)

Soportado en precisiones de 32 y 64 bits.

```rust
let x = 3.0;      [cite_start]// f64 por defecto [cite: 172]
let y: f32 = 3.0; [cite_start]// f32 explícito [cite: 173]

```

### C. Booleanos y Operadores

Admite los valores `true` y `false`. Incluye operadores estándar de cortocircuito y lógicos: `&` (AND), `&&` (AND corto), `|` (OR), `||` (OR corto), `^` (XOR), `==`, `!=`, `<`, `>`, `<=`, `>=`.

### D. Caracteres (`char`)

A diferencia de otros lenguajes donde un char equivale a 1 byte (ASCII), en Rust **ocupa 4 bytes** y almacena un **valor Unicode**. Puede contener de forma nativa letras acentuadas, caracteres orientales (chino, japonés, coreano) y emojis. Se delimita exclusivamente con **comillas simples**.

```rust
let c = 'z';
[cite_start]let emoji = '🐱'; [cite: 208]

```

---

## 10. Tipos de Datos Compuestos (*Compound Types*)

### A. Strings: La gran distinción de Rust

Rust maneja los strings de dos formas diferenciadas debido a cómo se organiza la memoria física (Pila/*Stack* vs Montículo/*Heap*):

1.

**`&str` (Slice de String):** Es una cadena inmutable y de longitud fija. El texto literal escrito entre comillas dobles es siempre un `&str` y se almacena en la memoria estática de la pila.

1.

**`String`:** Es una estructura mutable de longitud variable. Almacena la información en el *heap* (montículo), lo que permite que el texto crezca o decrezca de forma dinámica en ejecución.

```rust
fn main() {
    [cite_start]let str_fijo: &str = "Soy un string inmutable"; [cite: 223]
    [cite_start]let mut str_mutable: String = "Soy mutable".to_string(); [cite: 224] // Conversión a String dinámico
    
    [cite_start]str_mutable += " concateno"; [cite: 226]
    [cite_start]println!("{}", str_mutable); [cite: 227]
}

```

* **Manejo de Memoria:** Almacenar datos variables es costoso porque la memoria se asigna secuencialmente. Si un string crece, Rust se encarga de reubicarlo dinámicamente en bloques de "cajones" vacíos contiguos del *heap*, llevando el control en una tabla interna de direcciones de inicio y fin. Para optimizar el rendimiento, lo que sabemos que no va a cambiar se define fijamente como `&str` evitando esta sobrecarga de gestión.

### B. Tuplas (`tuple`)

Colección de **tamaño fijo** que permite agrupar elementos de **diferentes tipos de datos**.

* Admite desestructuración o desempaquetado (*Unpacking*) para asignar sus valores a variables independientes de forma directa.

* El acceso individual a los campos se realiza mediante la nomenclatura de punto seguido del índice numérico (`tupla.0`).

```rust
fn main() {
    [cite_start]let mut tupla: (String, f32, u8) = ("hola".to_string(), 3.0, 3); [cite: 231]
    [cite_start]tupla.0 = "cambio valor".to_string(); [cite: 231] // Modificación respetando el tipo del índice
    
    let (hola, flotante, entero) = tupla; [cite_start]// Unpacking [cite: 234]
}

```

### C. Arreglos (`array`)

Colección de **tamaño fijo** en tiempo de compilación donde **todos los elementos deben ser estrictamente del mismo tipo**.

```rust
fn main() {
    [cite_start]let arreglo = [1, 2, 3, 5]; [cite: 241]
    [cite_start]println!("El tercer elemento es: {}", arreglo[2]); [cite: 242]
}

```

* **Funciones Asociadas (Métodos):** En Rust, los métodos propios de las estructuras se denominan funciones asociadas. El método `.last()` de un arreglo busca obtener su último elemento. Debido a la seguridad de Rust, si el arreglo estuviera vacío, este método devolvería un tipo seguro (`Option`) que evita que el programa rompa. Emplear `.unwrap()` desenvuelve el valor directamente asumiendo que como desarrolladores garantizamos que el dato existe.

---

## 11. Estructura de Proyectos Avanzada con Cargo

Para programas de producción o sistemas de gran envergadura no se compilan archivos `.rs` sueltos de forma directa. Se utiliza **Cargo**, el gestor de paquetes y administrador de proyectos oficial de Rust (equivalente a `npm` en Node o `pip` en Python).

### Creación de un Proyecto

Ejecutando el comando: `cargo new nombre_del_proyecto`. Cargo estructurará el entorno automáticamente:

*

**`Cargo.toml`:** Archivo de configuración donde se definen los metadatos y las dependencias (librerías de terceros) del proyecto.

*

**Carpeta `src/`:** Contiene el código fuente del desarrollo.

* Generará un `main.rs` si el proyecto es una aplicación ejecutable binaria.
* Generará un `lib.rs` si el proyecto se diseña para ser una librería reutilizable por terceros.

### Comandos Esenciales de Cargo

* **`cargo build`:** Descarga las dependencias declaradas en el archivo `.toml`, las instala y compila el proyecto completo, generando el binario ejecutable dentro de la carpeta local `target/`.
* **`cargo run`:** Realiza la compilación del proyecto (build) y ejecuta de forma inmediata el binario resultante.
* **`cargo check`:** Realiza una verificación ultrarrápida de la sintaxis y tipos del código fuente sin compilar el binario. Es la herramienta principal del flujo de trabajo diario para detectar errores al instante sin sufrir demoras de tiempo de compilación.

### Gestión de Paquetes (*Crates*)

Las librerías en Rust se denominan **Crates**. El repositorio central de la comunidad es [crates.io](https://crates.io).

* **Instalación Automática:** Desde la terminal del proyecto, ejecuta `cargo add nombre_de_la_crate` (Ej: `cargo add rand`). Esto modificará automáticamente el archivo `Cargo.toml`.
* **Criterios de Selección de una Crate:** Al buscar librerías de terceros, es vital analizar dos métricas en la web oficial para garantizar seguridad y mantenimiento:

1. El volumen histórico de descargas (a mayor número, mayor validación comunitaria).
2. La fecha de la última actualización (evitar librerías discontinuadas o sin soporte hace años).

---

## 12. Uso de Librerías en Código (Ejemplo Completo)

Ejemplo práctico de importación de la librería estándar de entrada y salida (`std::io::stdin`) para interactuar con la consola de mandos:

```rust
use std::io::stdin; [cite_start]// Importación de módulo estándar [cite: 247]

fn main() {
    [cite_start]println!("Ingrese su nombre: "); [cite: 250]
    
    // Generamos un String vacío en memoria listo para actuar como buffer
    [cite_start]let mut nombre = String::new(); [cite: 251]
    
    // Leemos la línea de la terminal pasándole una referencia mutable del buffer.
    // .read_line() devuelve un tipo Result que puede contener un flujo exitoso o un error.
    // El método .expect() captura de forma explícita el escenario de error; si falla,
    // interrumpe la aplicación de forma segura lanzando un Panic! con el mensaje indicado.
    [cite_start]stdin().read_line(&mut nombre).expect("Error al leer el nombre."); [cite: 252]
    
    [cite_start]println!("Hola, {}!", nombre); [cite: 253]
}

```

---

## 13. Entornos de Desarrollo Recomendados (IDEs)

Rust ofrece compatibilidad e integración con múltiples editores del mercado mediante el motor oficial **`rust-analyzer`**.

### Tabla Comparativa de Soporte

(Simbología: `✓` = Soportado de forma nativa  | `plugin` = Requiere instalación de extensión externa )

| Editor / IDE | Sintaxis (`.rs`) | Sintaxis (`.toml`) | Snippets | Completado | Linting | Formateo | Ir a Def | Debugging |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| **VS Code** | plugin | ✓ | plugin | plugin | plugin | plugin | plugin | ✓ |
| **Vim / Neovim** | ✓ | plugin | plugin | plugin | plugin | plugin | plugin | plugin |
| **Sublime Text** | ✓ | plugin | ✓ | plugin | plugin | plugin | plugin | — |
| **Emacs** | plugin | ✓ | plugin | ✓ | plugin | plugin | plugin | plugin |
| **IntelliJ IDEs** | ✓ | plugin | plugin | plugin | plugin | plugin | plugin | plugin |

### Configuración Recomendada para la Materia

Se aconseja a los alumnos utilizar **Visual Studio Code** junto con las siguientes tres extensiones fundamentales para facilitar el desarrollo en la cursada:

1.

**`rust-analyzer`:** Ejecuta chequeos continuos de código en segundo plano, autocompleta sintaxis y expone advertencias antes de compilar.

1.

**`better-toml`:** Provee resaltado de sintaxis correcto y validación para el archivo `Cargo.toml`.

1.

**`crates`:** Analiza las versiones de las librerías declaradas en el `.toml` y asiste autocompletando las dependencias disponibles en tiempo real.
