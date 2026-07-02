program ejercicio5;

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
    // Pos(substr, str) devuelve > 0 si substr esta contenido en str
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
    writeln('0. Salir');
    writeln('===========================================');
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
    end;
  until opcion = '0';
end;

var
  arc: archivo_celulares;

begin
  menu(arc);
end.
