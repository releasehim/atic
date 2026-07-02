# 4 - Corte de Control

## Diapositiva 1
Fundamentos de Organización de Datos
1
Archivos
Corte de Control

## Diapositiva 2
### Algorítmica clásica sobre archivos
#### Corte de Control
Proceso mediante el cual la información de un archivo es presentada en forma organizada de acuerdo a la estructura que posee el archivo.

## Diapositiva 3
### Ejemplo
Se almacena en un archivo la información de ventas de una cadena de electrodomésticos. Dichas ventas han sido efectuadas por los vendedores de cada sucursal de cada ciudad de cada provincia del país.

Es necesario informar al gerente de ventas de la empresa, el total vendido en cada sucursal, ciudad y provincia, así como el total final.

## Diapositiva 4
### Ejemplo – Formato (Parte 1)
```text
Provincia: .……… 
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
```

## Diapositiva 5
### Ejemplo – Formato (Parte 2)
```text
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

## Diapositiva 6
### Ejemplo: Precondiciones
* El archivo se encuentra ordenado por **provincia, ciudad y sucursal**.
* En diferentes provincias pueden existir ciudades con el mismo nombre, y en diferentes ciudades pueden existir sucursales con igual denominación.

## Diapositiva 7
### Ejemplo: Definición de tipos
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

## Diapositiva 8
### Ejemplo: variables y procedimiento leer
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

## Diapositiva 9
### Ejemplo: programa principal
```pascal
begin
  assign(archivo, 'archivo_ventas');
  reset(archivo);
  
  leer(archivo, reg);
  total := 0;
```

## Diapositiva 10
### Ejemplo: algoritmo (Bucles anidados)
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

## Diapositiva 11
### Ejemplo: algoritmo (Bucle de detalle e impresión)
```pascal
        while (prov = reg.provincia) and (ciudad = reg.ciudad) and (sucursal = reg.sucursal) do begin
          write('Vendedor:', reg.vendedor);
          writeln(reg.monto);
          totSuc := totSuc + reg.monto;
          leer(archivo, reg);
        end;
```

## Diapositiva 12
### Ejemplo: algoritmo (Cierre de niveles y acumuladores)
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
