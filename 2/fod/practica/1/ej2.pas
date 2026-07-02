program ejercicio2;

type
  archivo_enteros = file of integer;

var
  archivo: archivo_enteros;
  nombre_fisico: string;
  num: integer;
  cant_menores: integer;
  suma_total: integer;
  cant_total: integer;
  promedio: real;

begin
  write('Ingrese el nombre del archivo a procesar: ');
  readln(nombre_fisico);
  
  assign(archivo, nombre_fisico);
  
  // Usamos try-except o simplemente abrimos con reset
  {$I-} // desactiva control de errores de E/S directo para verificar si existe
  reset(archivo);
  {$I+}
  
  if (ioresult <> 0) then
  begin
    writeln('Error: El archivo no existe o no se pudo abrir.');
  end
  else
  begin
    cant_menores := 0;
    suma_total := 0;
    cant_total := 0;
    
    writeln('Contenido del archivo:');
    while not eof(archivo) do
    begin
      read(archivo, num);
      writeln(num);
      
      cant_total := cant_total + 1;
      suma_total := suma_total + num;
      if (num < 15000) then
        cant_menores := cant_menores + 1;
    end;
    
    close(archivo);
    
    writeln('----------------------------------------');
    writeln('Cantidad de numeros menores a 15000: ', cant_menores);
    if (cant_total > 0) then
    begin
      promedio := suma_total / cant_total;
      writeln('Promedio de los numeros del archivo: ', promedio:0:2);
    end
    else
    begin
      writeln('El archivo esta vacio, no se puede calcular el promedio.');
    end;
  end;
end.
