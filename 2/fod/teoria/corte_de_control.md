# Corte de Control

## Algorítmica clásica sobre archivos

**Corte de control** es el proceso mediante el cual la información de un archivo se presenta de forma organizada, de acuerdo a la estructura (los órdenes) que posee el archivo.

Al igual que maestro-detalle y merge, esta algorítmica siempre se trabaja sobre archivos ordenados, ya sea por uno, dos o N órdenes; sin ese orden, no tiene sentido aplicarla. Se utiliza cuando es necesario representar, informar o mostrar la información de un archivo (grande o chico) de manera organizada para un destinatario, como podría ser un gerente.

En esencia, "cortar el control" significa usar una estructura de control para frenar el procesamiento del programa: cada vez que cambia un valor de orden (por ejemplo, cambia el producto, el empleado o, en este caso, la sucursal, la ciudad o la provincia), se detiene momentáneamente la ejecución para totalizar o imprimir información. De hecho, esta misma idea de "frenar para totalizar o informar" también está presente, aunque de forma menos explícita, en maestro-detalle y en merge.

## Ejemplo

Se almacena en un archivo la información de ventas de una cadena de electrodomésticos. Dichas ventas han sido efectuadas por los vendedores de cada sucursal, de cada ciudad, de cada provincia del país.

Es necesario informar al gerente de ventas de la empresa el total vendido en cada sucursal, ciudad y provincia, así como el total final de la empresa.

### Formato del informe

```text
Provincia: ………
Ciudad: …………
  Sucursal: ………..
    Vendedor 1 Total $$
    …
    Vendedor N Total $$
    Total Sucursal: Total $$
  Sucursal: ……….
    Vendedor 1 Total $$
    …
    Vendedor N Total $$
    Total Sucursal: Total $$
  ……
  Total Ciudad: $$
Ciudad: …………
  …
  Total Ciudad: $$
Total Provincia: $$
Provincia: .………
…
Total Ciudad: $$
Total Provincia: $$
Total Empresa: $$
```

Es decir: se informa cada sucursal con el detalle de venta de cada vendedor y el total de esa sucursal; al cambiar de ciudad se informa el total de la ciudad antes de pasar a la siguiente; al cambiar de provincia se informa el total de esa provincia; y al terminar de procesar todo el archivo se informa el total general de la empresa.

### Precondiciones

* El archivo se encuentra ordenado por **provincia, ciudad y sucursal**.
* En diferentes provincias pueden existir ciudades con el mismo nombre (por ejemplo, "Colón" existe como ciudad en Buenos Aires, Santa Fe y Entre Ríos), y en diferentes ciudades pueden existir sucursales con igual denominación. Esta precondición es discutible, ya que en general la razón social debería distinguir cada sucursal, pero a los fines del ejercicio hay que tenerla en cuenta.

Esto implica que, al construir el algoritmo, hay que tener cuidado de no confundir ni sumar entre sí los totales de dos ciudades distintas que comparten nombre, ni los de dos sucursales distintas que comparten nombre, como si fueran la misma.

### Definición de tipos

```pascal
program ejemplo;
const
  valor_alto = 'ZZZ';
type
  nombre = string[30];
  reg_venta = record
    vendedor: integer;
    monto: real;
    sucursal: nombre;
    ciudad: nombre;
    provincia: nombre;
  end;

  ventas = file of reg_venta;
```

El archivo de ventas es binario (`file of reg_venta`). Se define también una constante `valor_alto`, que se usará como centinela de fin de archivo junto con el procedimiento `leer`.

### Variables y procedimiento `leer`

```pascal
var
  reg: reg_venta;
  archivo: ventas;
  total, totProv, totCiudad, totSuc: real;
  prov, ciudad, sucursal: nombre;

procedure leer(var archivo: ventas; var dato: reg_venta);
begin
  if (not(EOF(archivo))) then
    read(archivo, dato)
  else
    dato.provincia := valor_alto;
end;
```

Como se pidieron cuatro totales (por sucursal, por ciudad, por provincia y general), se declaran cuatro acumuladores. Además, se necesita una variable de control por cada orden del archivo (provincia, ciudad y sucursal), cuyo tipo debe coincidir con el del campo correspondiente en el registro. El procedimiento `leer` funciona igual que en maestro-detalle: si no es fin de archivo, lee normalmente; si lo es, carga el valor centinela. Recordar siempre pasar los archivos por variable (`var`) a los procedimientos.

### Programa principal

```pascal
begin
  assign(archivo, 'archivo_ventas');
  reset(archivo);

  leer(archivo, reg);
  total := 0;
```

Se asigna el archivo físico al lógico, se abre con `reset` (porque ya está cargado), se lee el primer registro con el procedimiento `leer` y se inicializa el total general de la empresa en cero.

### Algoritmo: bucles anidados por orden

```pascal
  while (reg.provincia <> valor_alto) do begin
    writeln('Provincia:', reg.provincia);
    prov := reg.provincia;
    totProv := 0;

    while (prov = reg.provincia) do begin
      writeln('Ciudad:', reg.ciudad);
      ciudad := reg.ciudad;
      totCiudad := 0;

      while (prov = reg.provincia) and (ciudad = reg.ciudad) do begin
        writeln('Sucursal:', reg.sucursal);
        sucursal := reg.sucursal;
        totSuc := 0;
```

### Algoritmo: bucle de detalle e impresión por vendedor

```pascal
        while (prov = reg.provincia) and (ciudad = reg.ciudad) and (sucursal = reg.sucursal) do begin
          write('Vendedor:', reg.vendedor);
          writeln(reg.monto);
          totSuc := totSuc + reg.monto;
          leer(archivo, reg);
        end;
```

### Algoritmo: cierre de niveles y acumulación de totales

```pascal
        writeln('Total Sucursal', totSuc);
        totCiudad := totCiudad + totSuc;
      end; {while (prov = reg.provincia) and (ciudad = reg.ciudad)}

      writeln('Total Ciudad', totCiudad);
      totProv := totProv + totCiudad;
    end; {while(prov = reg.provincia)}

    writeln('Total Provincia', totProv);
    total := total + totProv;
  end; {while(reg.provincia <> valor_alto)}

  writeln('Total Empresa', total);
  close(archivo);
end.
```

## Cómo se construye este tipo de algoritmo

Hay una receta general, válida para cualquier corte de control:

* **Cantidad de `while`:** se necesita un `while` por cada orden del archivo, más uno adicional para el control de fin de archivo. En este ejemplo hay tres órdenes (provincia, ciudad, sucursal), por lo tanto hay cuatro `while`: el de fin de archivo (o valor alto), el de provincia, el de provincia+ciudad, y el de provincia+ciudad+sucursal.
* **Condiciones de cada `while`:** cada `while`, de afuera hacia adentro, va acumulando las condiciones de los `while` anteriores. El primero controla solo el primer orden (provincia); el segundo controla el primero y el segundo orden (provincia y ciudad); el tercero controla los tres órdenes (provincia, ciudad y sucursal). Si hubiera más órdenes, se seguirían concatenando de la misma manera.
* **Inicialización de contadores:** antes de cada `while` se inicializa en cero el contador correspondiente a ese nivel (por eso, cuando cambia de ciudad, el total de ciudad se reinicia automáticamente, simplemente por estar declarado antes de ese `while`) y se guarda en la variable de control (`prov`, `ciudad`, `sucursal`) el valor que se está procesando, para luego informarlo.
* **Acumulación al salir de cada `while`:** al salir de cada `while` (sin importar si el motivo fue un cambio de sucursal, de ciudad o de provincia, o el fin de archivo) hay que imprimir el total de ese nivel y sumarlo al nivel inmediatamente superior: al salir del `while` de sucursal se suma a la ciudad, al salir del de ciudad se suma a la provincia, y al salir del de provincia se suma al total general de la empresa.
* **Lectura en el `while` más interno:** es fundamental hacer la lectura del siguiente registro (`leer(archivo, reg)`) únicamente dentro del `while` más interno, el que recorre los registros de un mismo vendedor/sucursal. Si se omite esa lectura, el programa queda en un bucle infinito, porque la condición de provincia, ciudad y sucursal nunca cambia.
* **Cierre final:** cuando se sale del `while` más externo (porque ya no quedan registros para procesar), se imprime el total de la empresa y se cierra el archivo.

Siguiendo esta receta —un `while` por orden más uno para el fin de archivo, condiciones acumuladas de afuera hacia adentro, contadores inicializados antes de cada `while` y totalizados al salir de cada uno, y la lectura ubicada únicamente en el `while` más interno— el corte de control funciona siempre de la misma manera, sin importar cuántos órdenes tenga el archivo.
