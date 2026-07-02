# Maestro - Detalle

## Algorítmica clásica sobre archivos

* **Archivo maestro:** resume la información sobre el dominio de un problema específico (por ejemplo, el archivo de empleados de un municipio, o el archivo de productos de una empresa).
* **Archivo detalle:** contiene movimientos que modifican la información almacenada en el maestro (por ejemplo, las horas extras o vacaciones de los empleados, o las ventas sobre los productos).

Maestro-detalle es un algoritmo que siempre trabaja de la misma manera: lo único que cambia de un ejercicio a otro es el tipo de dato del archivo. Lo que sí puede variar es la cantidad de archivos de detalle involucrados; en los casos que se desarrollan a continuación se trabaja con un único archivo de detalle. Cuando se usan otras algorítmicas similares (maestro-detalle, corte de control, merge), todos los archivos intervinientes deben estar siempre ordenados por el mismo criterio.

## Importante

* Hay que analizar las precondiciones de cada caso particular antes de escribir el algoritmo.
* Si no se evalúan correctamente, el algoritmo puede no procesar algún registro, intentar leer el fin de archivo, o directamente fallar en su ejecución.

---

## Caso 1: un registro del detalle modifica un único registro del maestro

### Precondiciones

* Existe un único archivo maestro.
* Existe un único archivo detalle que modifica al maestro.
* Cada registro del detalle modifica a un solo registro del maestro, que seguro existe.
* No todos los registros del maestro son necesariamente modificados.
* Cada elemento del maestro que se modifica es alterado por un solo elemento del archivo detalle.
* Ambos archivos están ordenados por igual criterio.

### Definición de tipos

```pascal
type
  producto = record
    cod: string[4];
    descripcion: string[30];
    pu: real; {precio unitario}
    stock: integer;
  end;

  venta_prod = record
    cod: string[4];
    cant_vendida: integer;
  end;

  maestro = file of producto;
  detalle = file of venta_prod;
```

### Variables y apertura de archivos

```pascal
var
  mae: maestro;
  det: detalle;
  regm: producto;
  regd: venta_prod;

begin { Inicio del programa }
  assign(mae, 'maestro.dat');
  assign(det, 'detalle.dat');
  reset(mae);
  reset(det);
```

Como ambos archivos ya tienen información, se abren con `reset` (nunca con `rewrite`, que los dejaría vacíos).

### Algoritmo

```pascal
  {Loop archivo detalle}
  while not(EOF(det)) do begin
    read(mae, regm); // Lectura archivo maestro
    read(det, regd); // Lectura archivo detalle

    {Se busca en el maestro el producto del detalle}
    while (regm.cod <> regd.cod) do
      read(mae, regm);

    {Se modifica el stock del producto con la cantidad vendida de ese producto}
    regm.stock := regm.stock - regd.cant_vendida;

    {Se reubica el puntero en el maestro}
    seek(mae, filepos(mae)-1);

    {Se actualiza el maestro}
    write(mae, regm);
  end; // Fin while archivo detalle
  close(det);
  close(mae);
end.
```

### Cómo funciona

El recorrido se controla por el archivo de detalle: mientras no se llegue a su fin, se lee un registro de cada archivo. Como ambos están ordenados por el mismo criterio pero los códigos pueden no coincidir de entrada (por ejemplo, el detalle puede arrancar en el código 500 mientras el maestro arranca en el código 1), se avanza en el maestro con el `while` interno hasta encontrar el registro correspondiente al detalle leído. Al salir de ese `while` hay garantía de haber encontrado el producto, porque la precondición asegura que todo lo que aparece en el detalle existe en el maestro.

Una vez encontrado el producto, se le decrementa el stock con la cantidad vendida. Como la búsqueda avanzó el puntero del maestro una posición de más (quedó apuntando al siguiente registro), antes de escribir es necesario retroceder una posición con `seek(mae, filepos(mae)-1)`. Finalmente se actualiza el registro en el maestro. Al terminar de procesar todo el detalle, se cierran ambos archivos para que los cambios se guarden en disco.

---

## Caso 2: un registro del maestro puede ser modificado por uno o varios registros del detalle

### Precondiciones

* Existe un único archivo maestro.
* Existe un único archivo detalle que modifica al maestro.
* Cada registro del detalle modifica a un registro del maestro que seguro existe.
* No todos los registros del maestro son necesariamente modificados.
* Cada elemento del archivo maestro puede no ser modificado, o puede ser modificado por uno o más elementos del detalle.
* Ambos archivos están ordenados por igual criterio.

La diferencia con el caso 1 es que ahora, para un mismo producto, pueden venir varios registros en el detalle (por ejemplo, tres ventas distintas del mismo código de producto), por lo que es necesario totalizar esos registros antes de actualizar el maestro.

### Definición de tipos

```pascal
type
  producto = record
    cod: string[4];
    descripcion: string[30];
    pu: real;
    stock: integer;
  end;

  venta_prod = record
    cod: string[4];
    cant_vendida: integer;
  end;

  detalle = file of venta_prod;
  maestro = file of producto;
```

### Variables y apertura de archivos

```pascal
var
  mae: maestro;
  det: detalle;
  regm: producto;
  regd: venta_prod;
  cod_actual: string[4];
  tot_vendido: integer;

begin {Inicio del programa}
  assign(mae, 'maestro');
  assign(det, 'detalle');
  reset(mae);
  reset(det);
```

### Algoritmo

```pascal
  {Loop archivo detalle}
  while not(EOF(det)) do begin
    read(mae, regm); // Lectura archivo maestro
    read(det, regd); // Lectura archivo detalle

    {Se busca en el maestro el producto del detalle}
    while (regm.cod <> regd.cod) do
      read(mae, regm);

    {Se totaliza la cantidad vendida del detalle}
    cod_actual := regd.cod;
    tot_vendido := 0;
    while (regd.cod = cod_actual) do begin
      tot_vendido := tot_vendido + regd.cant_vendida;
      read(det, regd);
    end;

    {Se actualiza el stock del producto con la cantidad vendida del mismo}
    regm.stock := regm.stock - tot_vendido;

    {se reubica el puntero en el maestro}
    seek(mae, filepos(mae)-1);

    {se actualiza el maestro}
    write(mae, regm);
  end;
  close(det);
  close(mae);
end.
```

### Diferencia con el caso anterior

Se agrega una iteración interna que permite agrupar (totalizar) todos los registros del detalle que corresponden al mismo código antes de actualizar el maestro: se guarda el código actual, se inicializa el total en cero y se va sumando mientras el código del detalle coincida con el actual.

### Problema de esta solución

Este algoritmo tiene un error intencional, pensado para mostrar un error común tanto en exámenes como en la práctica. El `read(det, regd)` que está dentro del `while` de totalización no controla si se llegó al fin del archivo de detalle. Si el registro que se está totalizando es justamente el último del archivo, esa lectura intenta leer sobre la marca de fin de archivo y el programa falla. Una solución posible sería agregar un `if not(EOF(det))` antes de esa lectura, pero esto puede generar otro problema: al salir de la iteración interna y volver a la iteración principal, se vuelve a leer un registro del detalle, perdiendo el que ya se había leído. La forma más robusta de resolver esto es usando un procedimiento de lectura controlada, que se desarrolla en el caso siguiente.

---

## Caso 3: solución robusta con centinela (valor alto)

Para evitar los problemas de control de fin de archivo del caso anterior, se introduce un procedimiento `leer` que encapsula la lectura y, al llegar al fin del archivo, carga en el registro un valor centinela (`valoralto`) en lugar de fallar. De esta forma no hace falta verificar `EOF` antes de cada lectura del detalle: alcanza con comparar el código leído contra `valoralto` para saber si se terminó el archivo.

Una particularidad a tener en cuenta: siempre que se pasan archivos como parámetro a un procedimiento, deben pasarse por variable (`var`).

### Definición de tipos

```pascal
const
  valoralto = 'ZZZZ';

type
  str4 = string[4];
  producto = record
    cod: str4;
    descripcion: string[30];
    pu: real;
    stock: integer;
  end;

  venta_prod = record
    cod: str4;
    cant_vendida: integer;
  end;

  detalle = file of venta_prod;
  maestro = file of producto;
```

### Variables y procedimiento `leer`

```pascal
var
  mae: maestro; regm: producto;
  det: detalle; regd: venta_prod;
  total: integer; aux: str4;

procedure leer(var archivo: detalle; var dato: venta_prod);
begin
  if (not(EOF(archivo))) then
    read(archivo, dato)
  else
    dato.cod := valoralto;
end;
```

El procedimiento `leer` recibe el archivo de detalle y un registro: si no se llegó al fin de archivo, hace la lectura normal; si se llegó al fin, asigna el código centinela `valoralto` al registro. Usarlo evita tener que repetir el chequeo de `EOF` en cada punto del programa donde se lee del detalle, reduciendo el riesgo de perder registros o de leer la marca de fin de archivo por error.

### Programa principal

```pascal
begin
  assign(mae, 'maestro');
  assign(det, 'detalle');
  reset(mae);
  reset(det);
  read(mae, regm);
  leer(det, regd);
```

### Algoritmo

```pascal
  {Se procesan todos los registros del archivo detalle}
  while (regd.cod <> valoralto) do begin
    aux := regd.cod;
    total := 0;

    {Se totaliza la cantidad vendida de productos iguales en el archivo de detalle}
    while (aux = regd.cod) do begin
      total := total + regd.cant_vendida;
      leer(det, regd);
    end;

    {se busca el producto del detalle en el maestro}
    while (regm.cod <> aux) do
      read(mae, regm);

    {se modifica el stock del producto con la cantidad total vendida de ese producto}
    regm.stock := regm.stock - total;

    {se reubica el puntero en el maestro}
    seek(mae, filepos(mae)-1);

    {se actualiza el maestro}
    write(mae, regm);

    {se avanza en el maestro}
    if (not(EOF(mae))) then
      read(mae, regm);
  end;
  close(det);
  close(mae);
end.
```

### Cómo funciona

El ciclo principal ahora se controla comparando el código leído del detalle contra `valoralto`, en lugar de usar `EOF` directamente. Dentro de la totalización, se usa `leer` en vez de `read`, por lo que ya no hay riesgo de leer fin de archivo: cuando se llega al final del detalle, el procedimiento carga `valoralto` y el `while` de totalización corta correctamente, ya sea porque cambió el código o porque se llegó al fin de archivo.

Para la búsqueda en el maestro sí se usa `read` directo (sin el procedimiento `leer`), porque la precondición garantiza que todo código que aparece en el detalle existe en el maestro, así que nunca se va a leer su fin de archivo en esa búsqueda. Al encontrarlo, se actualiza el stock con el total acumulado, se retrocede una posición con `seek` (por el mismo motivo que en los casos anteriores) y se escribe el registro actualizado. El `read(mae, regm)` final, protegido con `if not(EOF(mae))`, avanza el maestro para la siguiente iteración, aunque en rigor no es estrictamente necesario.

---

## Extensión a varios archivos de detalle

Esta misma lógica puede extenderse al caso de tener varios archivos de detalle en simultáneo (por ejemplo, cinco archivos de detalle en vez de uno), siempre que todos estén ordenados por el mismo criterio. La idea general es: en cada paso, tomar el registro más chico entre todos los detalles (según el criterio de orden) y usarlo para actualizar el registro correspondiente del maestro, repitiendo el proceso de la algorítmica del caso 3 (con centinela) para cada uno de los archivos de detalle.
