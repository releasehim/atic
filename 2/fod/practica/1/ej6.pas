program ejercicio6;

type
  celular = record
    codigo: integer;
    nombre: string[20];
    descripcion: string[50];
    marca: string[20];
    precio: real;
    stockMin: integer;
    stockDisp: integer;
  end;

  archivo_celulares = file of celular;

function trimSpace(s: string): string;
var
  i: integer;
begin
  i := 1;
  while (i <= length(s)) and (s[i] = ' ') do
    i := i + 1;
  trimSpace := copy(s, i, length(s) - i + 1);
end;

procedure mostrarCelular(c: celular);
begin
  writeln('Cod: ', c.codigo, ' | Nombre: ', c.nombre, ' | Marca: ', c.marca, 
          ' | Precio: ', c.precio:0:2, ' | Stock: ', c.stockDisp, 
          ' | Min Stock: ', c.stockMin, ' | Desc: ', c.descripcion);
end;

procedure crearArchivoBinario(var arc: archivo_celulares);
var
  txt: text;
  c: celular;
  nombre_fisico: string;
begin
  write('Ingrese el nombre del archivo binario a crear: ');
  readln(nombre_fisico);
  assign(arc, nombre_fisico);
  
  assign(txt, 'celulares.txt');
  {$I-}
  reset(txt);
  {$I+}
  if (ioresult <> 0) then
  begin
    writeln('Error: No se pudo encontrar el archivo "celulares.txt" en el directorio.');
    exit;
  end;
  
  rewrite(arc);
  
  while not eof(txt) do
  begin
    // Linea 1: codigo, precio, marca
    read(txt, c.codigo);
    read(txt, c.precio);
    readln(txt, c.marca);
    c.marca := trimSpace(c.marca);
    
    // Linea 2: stockDisp, stockMin, descripcion
    read(txt, c.stockDisp);
    read(txt, c.stockMin);
    readln(txt, c.descripcion);
    c.descripcion := trimSpace(c.descripcion);
    
    // Linea 3: nombre
    readln(txt, c.nombre);
    c.nombre := trimSpace(c.nombre);
    
    write(arc, c);
  end;
  
  close(arc);
  close(txt);
  writeln('Archivo binario creado y cargado con exito.');
end;

procedure listarBajoStock(var arc: archivo_celulares);
var
  c: celular;
  encontro: boolean;
begin
  reset(arc);
  encontro := false;
  writeln('Celulares con stock menor al stock minimo:');
  while not eof(arc) do
  begin
    read(arc, c);
    if (c.stockDisp < c.stockMin) then
    begin
      mostrarCelular(c);
      encontro := true;
    end;
  end;
  close(arc);
  
  if not encontro then
    writeln('No hay celulares con stock por debajo del minimo.');
end;

procedure buscarDescripcion(var arc: archivo_celulares);
var
  c: celular;
  busqueda: string;
  encontro: boolean;
begin
  write('Ingrese la cadena a buscar en la descripcion: ');
  readln(busqueda);
  
  reset(arc);
  encontro := false;
  writeln('Resultados de la busqueda:');
  while not eof(arc) do
  begin
    read(arc, c);
    if (pos(busqueda, c.descripcion) > 0) then
    begin
      mostrarCelular(c);
      encontro := true;
    end;
  end;
  close(arc);
  
  if not encontro then
    writeln('No se encontraron celulares con esa descripcion.');
end;

procedure exportarTexto(var arc: archivo_celulares);
var
  txt: text;
  c: celular;
begin
  assign(txt, 'celulares.txt');
  rewrite(txt);
  reset(arc);
  
  while not eof(arc) do
  begin
    read(arc, c);
    writeln(txt, c.codigo, ' ', c.precio:0:2, ' ', c.marca);
    writeln(txt, c.stockDisp, ' ', c.stockMin, ' ', c.descripcion);
    writeln(txt, c.nombre);
  end;
  
  close(arc);
  close(txt);
  writeln('Archivo "celulares.txt" exportado con exito.');
end;

procedure agregarCelulares(var arc: archivo_celulares);
var
  c: celular;
begin
  writeln('--- Adicion de celulares ---');
  write('Ingrese nombre del nuevo celular (o ''fin'' para terminar): ');
  readln(c.nombre);
  
  while (c.nombre <> 'fin') do
  begin
    write('Ingrese codigo: ');
    readln(c.codigo);
    write('Ingrese marca: ');
    readln(c.marca);
    write('Ingrese precio: ');
    readln(c.precio);
    write('Ingrese stock disponible: ');
    readln(c.stockDisp);
    write('Ingrese stock minimo: ');
    readln(c.stockMin);
    write('Ingrese descripcion: ');
    readln(c.descripcion);
    
    reset(arc);
    seek(arc, fileSize(arc));
    write(arc, c);
    close(arc);
    writeln('Celular agregado con exito.');
    
    writeln;
    write('Ingrese nombre del nuevo celular (o ''fin'' para terminar): ');
    readln(c.nombre);
  end;
end;

procedure modificarStock(var arc: archivo_celulares);
var
  nombre_buscado: string;
  nuevo_stock: integer;
  c: celular;
  encontrado: boolean;
begin
  write('Ingrese el nombre del celular a buscar para modificar stock: ');
  readln(nombre_buscado);
  
  reset(arc);
  encontrado := false;
  while not eof(arc) and not encontrado do
  begin
    read(arc, c);
    if (c.nombre = nombre_buscado) then
    begin
      encontrado := true;
      mostrarCelular(c);
      write('Ingrese el nuevo stock disponible: ');
      readln(nuevo_stock);
      c.stockDisp := nuevo_stock;
      
      seek(arc, filepos(arc) - 1);
      write(arc, c);
      writeln('Stock modificado con exito.');
    end;
  end;
  close(arc);
  
  if not encontrado then
    writeln('No se encontro ningun celular con el nombre: ', nombre_buscado);
end;

procedure exportarSinStock(var arc: archivo_celulares);
var
  txt: text;
  c: celular;
begin
  assign(txt, 'SinStock.txt');
  rewrite(txt);
  reset(arc);
  
  while not eof(arc) do
  begin
    read(arc, c);
    if (c.stockDisp = 0) then
    begin
      writeln(txt, c.codigo, ' ', c.precio:0:2, ' ', c.marca);
      writeln(txt, c.stockDisp, ' ', c.stockMin, ' ', c.descripcion);
      writeln(txt, c.nombre);
    end;
  end;
  
  close(arc);
  close(txt);
  writeln('Archivo "SinStock.txt" exportado con exito.');
end;

procedure menu(var arc: archivo_celulares);
var
  opcion: char;
  nombre_fisico: string;
  abierto: boolean;
begin
  abierto := false;
  repeat
    writeln;
    writeln('================ CELULARES ================');
    writeln('1. Crear archivo binario desde "celulares.txt"');
    writeln('2. Listar celulares con bajo stock');
    writeln('3. Buscar celulares por descripcion');
    writeln('4. Exportar archivo binario a "celulares.txt"');
    writeln('5. Agregar uno o mas celulares al final');
    writeln('6. Modificar el stock de un celular');
    writeln('7. Exportar celulares sin stock a "SinStock.txt"');
    writeln('0. Salir');
    writeln('===========================================');
    write('Seleccione una opcion: ');
    readln(opcion);
    
    if (opcion in ['2', '3', '4', '5', '6', '7']) and not abierto then
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
          if abierto then listarBajoStock(arc);
        end;
      '3':
        begin
          if abierto then buscarDescripcion(arc);
        end;
      '4':
        begin
          if abierto then exportarTexto(arc);
        end;
      '5':
        begin
          if abierto then agregarCelulares(arc);
        end;
      '6':
        begin
          if abierto then modificarStock(arc);
        end;
      '7':
        begin
          if abierto then exportarSinStock(arc);
        end;
    end;
  until opcion = '0';
end;

var
  arc: archivo_celulares;

begin
  menu(arc);
end.
