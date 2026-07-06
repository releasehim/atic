# Práctica 4: Generics, Traits, Closures, Iterator

**UNLP. Facultad de Informática.**  
**Seminario de Lenguajes opción Rust - Cursada 2026**

> **Nota:** Para todos los ejercicios realizar los tests de unidad correspondientes.

---

### 1. Números primos con Trait y Closure

Escriba una función que reciba un vector de números enteros y retorne la cantidad de números primos. Cree un *trait* para la determinación del número primo e impleméntelo según corresponda. Utilice la función `iter` sobre el vector y aplique un *closure* para resolverlo.

---

### 2. Estructura `Persona` y funciones de filtrado

Dado el siguiente `struct`:

```rust
struct Persona<'a> {
    nombre: &'a str,
    apellido: &'a str,
    direccion: &'a str,
    ciudad: &'a str,
    salario: f64,
    edad: u8,
}
```

Resuelva los siguientes puntos:

1. **a.** Escriba una función que reciba un vector de personas y otro parámetro que indica un salario. Debe retornar un listado de personas donde el salario es mayor al parámetro recibido.
2. **b.** Escriba una función que reciba un vector de personas, edad y el nombre de una ciudad. Debe retornar las personas mayores al parámetro edad y que viven en la ciudad pasada por parámetro.
3. **c.** Escriba una función que reciba un vector de personas y un nombre de una ciudad. Debe retornar `true` si todas las personas viven en la ciudad pasada por parámetro, `false` caso contrario.
4. **d.** Escriba una función que reciba un vector de personas y un nombre de una ciudad. Debe retornar `true` si al menos vive una persona en la ciudad pasada por parámetro, `false` caso contrario.
5. **e.** Escriba una función que reciba un arreglo de personas y una persona. Debe retornar `true` si la persona existe en el arreglo, `false` caso contrario.
6. **f.** Escriba una función que reciba un arreglo de personas. Debe retornar un arreglo con las edades de las personas.
7. **g.** Escriba una función que reciba un arreglo de personas. Debe retornar la persona con el menor salario y la persona con el mayor salario, en caso de que haya más de una persona en cada categoría desempatar por la edad más grande.

> **Nota importante:** Implemente todos los métodos y *traits* que considere para resolver los ejercicios. **Todos los ejercicios deben resolverse con `iterar` y `closure`.**

---

### 3. Plataforma de streaming "StreamingRust"

La plataforma de streaming "StreamingRust" ofrece distintos tipos de suscripciones (`Basic`, `Classic`, `Super`) a sus usuarios. Cada suscripción tiene un costo mensual, una duración de meses y una fecha de inicio. Además, los usuarios pueden pagar por sus suscripciones con distintos medios de pago: `Efectivo`, `MercadoPago`, `Tarjeta de Crédito`, `Transferencia Bancaria` y `Cripto`. Cada medio de pago tiene sus datos correspondientes a excepción de `Efectivo`.

Los usuarios solo pueden tener una suscripción activa a la vez.

Implemente las estructuras, funciones asociadas y *traits* necesarios para resolver las siguientes acciones:

* Crear un usuario con una determinada suscripción y medio de pago.
* Dado un usuario hacer un **upgrade** sobre la suscripción. Es decir si está en `Basic` pasa a `Classic` y si está en `Classic` pasa a `Super`.
* Dado un determinado usuario, hacer un **downgrade** sobre una suscripción, si la suscripción es del tipo `Basic` al hacerlo se cancelará la suscripción.
* Dado un usuario cancelar la suscripción.
* Saber el medio de pago que es más utilizado por los usuarios sobre las suscripciones activas.
* Saber cuál es la suscripción más contratada por los usuarios sobre las suscripciones activas.
* Saber cuál fue el medio de pago más utilizado.
* Saber cuál fue la suscripción más contratada.

---

### 4. Sistema de ventas de productos

Se requiere implementar un sistema de ventas de productos. De cada producto se conoce el `nombre`, una `categoría` y un `precio base`. Algunos productos pueden tener descuentos aplicables dependiendo de la categoría.

Además, se debe registrar al **vendedor** que realizó la venta y al **cliente**.

* De los **clientes** se conoce: nombre, apellido, dirección, DNI.
* Del **vendedor**: número de legajo, antigüedad y salario.
* Los **clientes** pueden tener un beneficio de descuento si tienen suscripción al *newsletter*, de ser así se tiene el correo electrónico del mismo.

El sistema debe permitir registrar las ventas realizadas y asociar el **medio de pago** utilizado. Los medios de pago aceptados son: `tarjeta de crédito`, `tarjeta de débito`, `transferencia bancaria` y `efectivo`.

Implemente las estructuras, funciones asociadas y *traits* necesarios para resolver las siguientes acciones:

* Crear una venta con: fecha, cliente, vendedor, medio de pago y un listado de productos con sus cantidades.
* Calcular el precio final de una venta en base a los productos que hay en ella. Para calcularlo tenga en cuenta que pueden haber determinados productos de alguna categoría donde debería aplicarse un descuento. Tanto la categoría como el porcentaje de descuento a aplicar son datos que le brinda el sistema. Es decir el sistema tiene una lista de las categorías con el descuento a aplicar. Además se debe aplicar un porcentaje de descuento general si el cliente tiene suscripción al *newsletter*.
* Para llevar un control de las ventas realizadas, se debe implementar un reporte que permita visualizar las ventas totales por categoría de producto y otro por vendedor.

---

### 5. Plataforma de intercambio de criptoactivos (XYZ)

La empresa XYZ es una plataforma de intercambio de criptoactivos que permite a los usuarios comprar y vender distintas criptomonedas. La plataforma permite el registro de usuarios y la gestión de sus balances en distintas criptomonedas y en dinero *fiat*.

* De los **usuarios** se conoce: nombre, apellido, e-mail, DNI, y si está validada su identidad o no.
* Cada **usuario** tiene un balance de las criptomonedas que se ofrecen en la plataforma.
* De las **criptomonedas** se conoce: nombre, prefijo y un listado de blockchains donde se pueden enviar o recibir.
* De cada **blockchain** se conoce el nombre y prefijo.

Implemente las estructuras, funciones asociadas y *traits* necesarios para resolver las siguientes acciones relacionadas al usuario:

1. **Ingresar dinero**: se recibe un monto en *fiat* de un usuario y se acredita al balance de *fiat* de dicho usuario. Además se crea una transacción del hecho donde los datos que se guardan son: fecha, tipo (`ingreso de dinero`), monto, usuario.
2. **Comprar determinada criptomoneda**: dado un monto de *fiat* se compra una cantidad de determinada criptomoneda. Tenga en cuenta que al momento de realizar la operación se obtiene del sistema la cotización actual de la criptomoneda para acreditar la correspondiente proporción en el balance de *fiat*. Luego de ello se registra la transacción con los siguientes datos: fecha, usuario, criptomoneda, tipo: `compra de cripto`, monto de cripto y cotización.
3. **Vender determinada criptomoneda**: dado un monto de cripto se vende por *fiat*. Tenga en cuenta que al momento de realizar la operación se obtiene del sistema la cotización actual de la criptomoneda para acreditar la correspondiente proporción en el balance de *fiat* y desacreditar en el balance de la criptomoneda. Luego de ello se registra la transacción con los siguientes datos: fecha, usuario, criptomoneda, tipo: `venta de cripto`, monto de cripto y cotización.
4. **Retirar criptomoneda a blockchain**: dado un monto de una cripto y una blockchain se le descuenta del balance de dicha cripto al usuario el monto; la blockchain devuelve un *hash* que representa una transacción en ella (esto hágalo retornando el nombre de la blockchain + un número random). Luego se genera una transacción con los siguientes datos: fecha, usuario, tipo: `retiro cripto`, blockchain, hash, cripto, monto, cotización.
5. **Recibir criptomoneda de blockchain**: dado un monto de una cripto y una blockchain se le acredita al balance de dicha cripto al usuario el monto. Luego se genera una transacción con los siguientes datos: fecha, usuario, tipo: `recepción cripto`, blockchain, cripto, monto, cotización.
6. **Retirar fiat por determinado medio**: dado un monto de *fiat* se le descuenta dicho monto del balance al usuario y se genera una transacción con la siguiente información: fecha, usuario, tipo: `retiro fiat`, monto y medio (puede ser `MercadoPago` o `Transferencia Bancaria`).

> **Nota importante sobre validaciones:**
>
> * Tanto para comprar, vender y retirar, el usuario **debe estar validado**.
> * Se debe validar siempre que haya balance suficiente para realizar la operación en los casos de compra, venta o retiro.

**Además, la empresa desea saber lo siguiente en base a sus operaciones:**

* Cuál es la criptomoneda que más cantidad de ventas tiene.
* Cuál es la criptomoneda que más cantidad de compras tiene.
* Cuál es la criptomoneda que más volumen de ventas tiene.
* Cuál es la criptomoneda que más volumen de compras tiene.
