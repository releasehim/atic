program ejercicio1;

type
  archivo_enteros = file of integer;

var
  archivo: archivo_enteros;
  nombre_fisico: string;
  num: integer;

begin
  write('Ingrese el nombre del archivo a crear: ');
  readln(nombre_fisico);
  
  assign(archivo, nombre_fisico);
  rewrite(archivo); // crea el archivo y lo abre para escritura
  
  writeln('Ingrese numeros enteros (finalice con 30000):');
  write('Numero: ');
  readln(num);
  
  while (num <> 30000) do
  begin
    write(archivo, num);
    write('Numero: ');
    readln(num);
  end;
  
  close(archivo);
  writeln('Archivo creado con exito.');
end.
