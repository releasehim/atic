program ejercicio4;

type
  empleado = record
    nro: integer;
    apellido: string[20];
    nombre: string[20];
    edad: integer;
    dni: longint;
  end;

  archivo_empleados = file of empleado;

procedure leerEmpleado(var e: empleado);
begin
  write('Ingrese apellido (o ''fin'' para terminar): ');
  readln(e.apellido);
  if (e.apellido <> 'fin') then
  begin
    write('Ingrese nombre: ');
    readln(e.nombre);
    write('Ingrese numero de empleado: ');
    readln(e.nro);
    write('Ingrese edad: ');
    readln(e.edad);
    write('Ingrese DNI (0 si no posee): ');
    readln(e.dni);
  end;
end;

procedure mostrarEmpleado(e: empleado);
begin
  writeln('Nro: ', e.nro, ' | Apellido: ', e.apellido, ' | Nombre: ', e.nombre, ' | Edad: ', e.edad, ' | DNI: ', e.dni);
end;

function existeEmpleado(var arc: archivo_empleados; nro: integer): boolean;
var
  e: empleado;
  aux: boolean;
begin
  reset(arc);
  aux := false;
  while not eof(arc) and not aux do
  begin
    read(arc, e);
    if (e.nro = nro) then
      aux := true;
  end;
  close(arc);
  existeEmpleado := aux;
end;

procedure crearArchivo(var arc: archivo_empleados);
var
  e: empleado;
  nombre_fisico: string;
begin
  write('Ingrese el nombre del archivo a crear: ');
  readln(nombre_fisico);
  assign(arc, nombre_fisico);
  rewrite(arc);
  
  writeln('--- Carga de empleados ---');
  leerEmpleado(e);
  while (e.apellido <> 'fin') do
  begin
    write(arc, e);
    leerEmpleado(e);
  end;
  
  close(arc);
  writeln('Archivo creado con exito.');
end;

procedure listarPorNombreApellido(var arc: archivo_empleados);
var
  e: empleado;
  busqueda: string[20];
  encontro: boolean;
begin
  write('Ingrese el nombre o apellido a buscar: ');
  readln(busqueda);
  
  reset(arc);
  encontro := false;
  writeln('Resultados de la busqueda:');
  while not eof(arc) do
  begin
    read(arc, e);
    if (e.nombre = busqueda) or (e.apellido = busqueda) then
    begin
      mostrarEmpleado(e);
      encontro := true;
    end;
  end;
  close(arc);
  
  if not encontro then
    writeln('No se encontraron empleados con ese nombre o apellido.');
end;

procedure listarTodos(var arc: archivo_empleados);
var
  e: empleado;
begin
  reset(arc);
  writeln('Listado de todos los empleados:');
  while not eof(arc) do
  begin
    read(arc, e);
    mostrarEmpleado(e);
  end;
  close(arc);
end;

procedure listarMayores70(var arc: archivo_empleados);
var
  e: empleado;
  encontro: boolean;
begin
  reset(arc);
  encontro := false;
  writeln('Empleados mayores de 70 anos (proximos a jubilarse):');
  while not eof(arc) do
  begin
    read(arc, e);
    if (e.edad > 70) then
    begin
      mostrarEmpleado(e);
      encontro := true;
    end;
  end;
  close(arc);
  
  if not encontro then
    writeln('No hay empleados mayores de 70 anos.');
end;

procedure agregarEmpleados(var arc: archivo_empleados);
var
  e: empleado;
begin
  writeln('--- Adicion de empleados ---');
  write('Ingrese apellido del nuevo empleado (o ''fin'' para terminar): ');
  readln(e.apellido);
  
  while (e.apellido <> 'fin') do
  begin
    write('Ingrese numero de empleado: ');
    readln(e.nro);
    
    if existeEmpleado(arc, e.nro) then
    begin
      writeln('Error: Ya existe un empleado con el numero ', e.nro, '. No se agregara.');
    end
    else
    begin
      write('Ingrese nombre: ');
      readln(e.nombre);
      write('Ingrese edad: ');
      readln(e.edad);
      write('Ingrese DNI (0 si no posee): ');
      readln(e.dni);
      
      reset(arc);
      seek(arc, fileSize(arc));
      write(arc, e);
      close(arc);
      writeln('Empleado agregado con exito.');
    end;
    
    writeln;
    write('Ingrese apellido del nuevo empleado (o ''fin'' para terminar): ');
    readln(e.apellido);
  end;
end;

procedure modificarEdad(var arc: archivo_empleados);
var
  nro_buscado: integer;
  nueva_edad: integer;
  e: empleado;
  encontrado: boolean;
begin
  write('Ingrese el numero de empleado a modificar: ');
  readln(nro_buscado);
  
  reset(arc);
  encontrado := false;
  while not eof(arc) and not encontrado do
  begin
    read(arc, e);
    if (e.nro = nro_buscado) then
    begin
      encontrado := true;
      writeln('Empleado encontrado:');
      mostrarEmpleado(e);
      write('Ingrese la nueva edad: ');
      readln(nueva_edad);
      e.edad := nueva_edad;
      
      seek(arc, filepos(arc) - 1);
      write(arc, e);
      writeln('Edad modificada con exito.');
    end;
  end;
  close(arc);
  
  if not encontrado then
    writeln('No se encontro ningun empleado con el numero ', nro_buscado);
end;

procedure exportarTodos(var arc: archivo_empleados);
var
  txt: text;
  e: empleado;
begin
  assign(txt, 'todos_empleados.txt');
  rewrite(txt);
  reset(arc);
  
  while not eof(arc) do
  begin
    read(arc, e);
    writeln(txt, e.nro, ' ', e.edad, ' ', e.dni, ' ', e.apellido, ' ', e.nombre);
  end;
  
  close(arc);
  close(txt);
  writeln('Archivo "todos_empleados.txt" exportado con exito.');
end;

procedure exportarSinDNI(var arc: archivo_empleados);
var
  txt: text;
  e: empleado;
begin
  assign(txt, 'faltaDNIEmpleado.txt');
  rewrite(txt);
  reset(arc);
  
  while not eof(arc) do
  begin
    read(arc, e);
    if (e.dni = 0) then
      writeln(txt, e.nro, ' ', e.edad, ' ', e.dni, ' ', e.apellido, ' ', e.nombre);
  end;
  
  close(arc);
  close(txt);
  writeln('Archivo "faltaDNIEmpleado.txt" exportado con exito.');
end;

procedure menu(var arc: archivo_empleados);
var
  opcion: char;
  nombre_fisico: string;
  abierto: boolean;
begin
  abierto := false;
  repeat
    writeln;
    writeln('================ M E N U ================');
    writeln('1. Crear un archivo de empleados');
    writeln('2. Abrir archivo existente y realizar consultas');
    writeln('3. Agregar uno o mas empleados');
    writeln('4. Modificar la edad de un empleado');
    writeln('5. Exportar todos a "todos_empleados.txt"');
    writeln('6. Exportar sin DNI a "faltaDNIEmpleado.txt"');
    writeln('0. Salir');
    writeln('=========================================');
    write('Seleccione una opcion: ');
    readln(opcion);
    
    // Auto-open file if we chose operations that require an open file and none is active
    if (opcion in ['2', '3', '4', '5', '6']) and not abierto then
    begin
      write('Ingrese el nombre del archivo a abrir: ');
      readln(nombre_fisico);
      assign(arc, nombre_fisico);
      {$I-}
      reset(arc);
      {$I+}
      if (ioresult <> 0) then
        writeln('Error: No se pudo abrir el archivo.')
      else
      begin
        close(arc);
        abierto := true;
      end;
    end;
    
    case opcion of
      '1':
        begin
          crearArchivo(arc);
          abierto := true;
        end;
      '2':
        begin
          if abierto then
          begin
            repeat
              writeln;
              writeln('--- Consultas ---');
              writeln('a. Buscar empleados por nombre o apellido');
              writeln('b. Listar todos los empleados');
              writeln('c. Listar empleados mayores de 70 anos');
              writeln('z. Volver al menu principal');
              write('Seleccione una consulta: ');
              readln(opcion);
              
              case opcion of
                'a', 'A': listarPorNombreApellido(arc);
                'b', 'B': listarTodos(arc);
                'c', 'C': listarMayores70(arc);
              end;
            until (opcion = 'z') or (opcion = 'Z');
            opcion := ' '; // reset opcion for outer menu
          end;
        end;
      '3':
        begin
          if abierto then agregarEmpleados(arc);
        end;
      '4':
        begin
          if abierto then modificarEdad(arc);
        end;
      '5':
        begin
          if abierto then exportarTodos(arc);
        end;
      '6':
        begin
          if abierto then exportarSinDNI(arc);
        end;
    end;
  until opcion = '0';
end;

var
  arc: archivo_empleados;

begin
  menu(arc);
end.
