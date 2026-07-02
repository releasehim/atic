program ejercicio7;

type
  novela = record
    codigo: integer;
    nombre: string[30];
    genero: string[30];
    precio: real;
  end;

  archivo_novelas = file of novela;

function trimSpace(s: string): string;
var
  i: integer;
begin
  i := 1;
  while (i <= length(s)) and (s[i] = ' ') do
    i := i + 1;
  trimSpace := copy(s, i, length(s) - i + 1);
end;

procedure mostrarNovela(n: novela);
begin
  writeln('Cod: ', n.codigo, ' | Nombre: ', n.nombre, ' | Genero: ', n.genero, ' | Precio: ', n.precio:0:2);
end;

procedure crearArchivoBinario(var arc: archivo_novelas);
var
  txt: text;
  n: novela;
  nombre_fisico: string;
begin
  write('Ingrese el nombre del archivo binario a crear: ');
  readln(nombre_fisico);
  assign(arc, nombre_fisico);
  
  assign(txt, 'novelas.txt');
  {$I-}
  reset(txt);
  {$I+}
  if (ioresult <> 0) then
  begin
    writeln('Error: No se pudo encontrar el archivo "novelas.txt" en el directorio.');
    exit;
  end;
  
  rewrite(arc);
  
  while not eof(txt) do
  begin
    // Linea 1: codigo, precio, genero
    read(txt, n.codigo);
    read(txt, n.precio);
    readln(txt, n.genero);
    n.genero := trimSpace(n.genero);
    
    // Linea 2: nombre
    readln(txt, n.nombre);
    n.nombre := trimSpace(n.nombre);
    
    write(arc, n);
  end;
  
  close(arc);
  close(txt);
  writeln('Archivo binario de novelas creado con exito.');
end;

procedure agregarNovela(var arc: archivo_novelas);
var
  n: novela;
begin
  writeln('--- Agregar una Novela ---');
  write('Ingrese codigo: ');
  readln(n.codigo);
  write('Ingrese nombre: ');
  readln(n.nombre);
  write('Ingrese genero: ');
  readln(n.genero);
  write('Ingrese precio: ');
  readln(n.precio);
  
  reset(arc);
  seek(arc, fileSize(arc));
  write(arc, n);
  close(arc);
  writeln('Novela agregada con exito.');
end;

procedure modificarNovela(var arc: archivo_novelas);
var
  cod_buscado: integer;
  n: novela;
  encontrado: boolean;
begin
  write('Ingrese el codigo de novela a modificar: ');
  readln(cod_buscado);
  
  reset(arc);
  encontrado := false;
  while not eof(arc) and not encontrado do
  begin
    read(arc, n);
    if (n.codigo = cod_buscado) then
    begin
      encontrado := true;
      mostrarNovela(n);
      
      writeln('Ingrese nuevos datos para la novela:');
      write('Ingrese nuevo nombre: ');
      readln(n.nombre);
      write('Ingrese nuevo genero: ');
      readln(n.genero);
      write('Ingrese nuevo precio: ');
      readln(n.precio);
      
      seek(arc, filepos(arc) - 1);
      write(arc, n);
      writeln('Novela modificada con exito.');
    end;
  end;
  close(arc);
  
  if not encontrado then
    writeln('No se encontro ninguna novela con el codigo: ', cod_buscado);
end;

procedure listarNovelas(var arc: archivo_novelas);
var
  n: novela;
begin
  reset(arc);
  writeln('Listado de novelas:');
  while not eof(arc) do
  begin
    read(arc, n);
    mostrarNovela(n);
  end;
  close(arc);
end;

procedure menu(var arc: archivo_novelas);
var
  opcion: char;
  nombre_fisico: string;
  abierto: boolean;
begin
  abierto := false;
  repeat
    writeln;
    writeln('================ NOVELAS ================');
    writeln('1. Crear archivo binario desde "novelas.txt"');
    writeln('2. Listar todas las novelas');
    writeln('3. Agregar una novela');
    writeln('4. Modificar una novela');
    writeln('0. Salir');
    writeln('=========================================');
    write('Seleccione una opcion: ');
    readln(opcion);
    
    if (opcion in ['2', '3', '4']) and not abierto then
    begin
      write('Ingrese el nombre del archivo binario a abrir: ');
      readln(nombre_fisico);
      assign(arc, nombre_fisico);
      {$I-}
      reset(arc);
      {$I+}
      if (ioresult <> 0) then
        writeln('Error: No se pudo abrir el archivo binario.')
      else
      begin
        close(arc);
        abierto := true;
      end;
    end;
    
    case opcion of
      '1':
        begin
          crearArchivoBinario(arc);
          abierto := true;
        end;
      '2':
        begin
          if abierto then listarNovelas(arc);
        end;
      '3':
        begin
          if abierto then agregarNovela(arc);
        end;
      '4':
        begin
          if abierto then modificarNovela(arc);
        end;
    end;
  until opcion = '0';
end;

var
  arc: archivo_novelas;

begin
  menu(arc);
end.
