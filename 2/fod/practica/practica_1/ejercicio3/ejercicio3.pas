program ejercicio3;

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
    writeln('0. Salir');
    writeln('=========================================');
    write('Seleccione una opcion: ');
    readln(opcion);
    
    case opcion of
      '1':
        begin
          crearArchivo(arc);
          abierto := true;
        end;
      '2':
        begin
          if not abierto then
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
    end;
  until opcion = '0';
end;

var
  arc: archivo_empleados;

begin
  menu(arc);
end.
