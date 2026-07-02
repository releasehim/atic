program ej3;
type
    libro = record
        codigo: integer;
        genero: string;
        titulo: string;
        autor: string;
        paginas: integer;
        precio: real;
    end;
    archivo = file of libro;

procedure leerLibro(var l: libro);
begin
    write('Codigo (-1 para fin): '); readln(l.codigo);
    if l.codigo <> -1 then begin
        write('Genero: '); readln(l.genero);
        write('Titulo: '); readln(l.titulo);
        write('Autor: '); readln(l.autor);
        write('Paginas: '); readln(l.paginas);
        write('Precio: '); readln(l.precio);
    end;
end;

procedure crearArchivo(var arc: archivo);
var
    l: libro;
    nombre: string;
begin
    write('Nombre del archivo a crear: '); readln(nombre);
    assign(arc, nombre);
    rewrite(arc);
    l.codigo := 0; // cabecera
    write(arc, l);
    leerLibro(l);
    while l.codigo <> -1 do begin
        write(arc, l);
        leerLibro(l);
    end;
    close(arc);
end;

procedure altaLibro(var arc: archivo);
var
    l, cabecera: libro;
begin
    leerLibro(l);
    if l.codigo <> -1 then begin
        reset(arc);
        read(arc, cabecera);
        if cabecera.codigo = 0 then begin
            seek(arc, filesize(arc));
            write(arc, l);
        end else begin
            seek(arc, cabecera.codigo * -1);
            read(arc, cabecera); 
            seek(arc, filepos(arc) - 1);
            write(arc, l);
            seek(arc, 0);
            write(arc, cabecera); 
        end;
        close(arc);
    end;
end;

procedure modificarLibro(var arc: archivo);
var
    l: libro;
    cod: integer;
    encontre: boolean;
begin
    write('Codigo del libro a modificar: '); readln(cod);
    reset(arc);
    encontre := false;
    while not eof(arc) and not encontre do begin
        read(arc, l);
        if l.codigo = cod then begin
            encontre := true;
            write('Nuevo genero: '); readln(l.genero);
            write('Nuevo titulo: '); readln(l.titulo);
            write('Nuevo autor: '); readln(l.autor);
            write('Nuevas paginas: '); readln(l.paginas);
            write('Nuevo precio: '); readln(l.precio);
            seek(arc, filepos(arc) - 1);
            write(arc, l);
        end;
    end;
    close(arc);
    if not encontre then writeln('Libro no encontrado.');
end;

procedure eliminarLibro(var arc: archivo);
var
    l, cabecera: libro;
    cod: integer;
    encontre: boolean;
begin
    write('Codigo de libro a eliminar: '); readln(cod);
    reset(arc);
    read(arc, cabecera);
    encontre := false;
    while not eof(arc) and not encontre do begin
        read(arc, l);
        if l.codigo = cod then begin
            encontre := true;
            seek(arc, filepos(arc) - 1);
            write(arc, cabecera);
            cabecera.codigo := (filepos(arc) - 1) * -1;
            seek(arc, 0);
            write(arc, cabecera);
        end;
    end;
    close(arc);
    if not encontre then writeln('Libro no encontrado.');
end;

procedure exportarLibros(var arc: archivo);
var
    l: libro;
    txt: text;
begin
    reset(arc);
    assign(txt, 'libros.txt');
    rewrite(txt);
    seek(arc, 1); 
    while not eof(arc) do begin
        read(arc, l);
        if l.codigo > 0 then 
            writeln(txt, l.codigo, ' ', l.genero, ' ', l.titulo, ' ', l.autor, ' ', l.paginas, ' ', l.precio:0:2);
    end;
    close(arc);
    close(txt);
end;

procedure menu(var arc: archivo);
var
    opc: integer;
    nombre: string;
begin
    repeat
        writeln('1. Crear archivo');
        writeln('2. Abrir y dar de alta');
        writeln('3. Abrir y modificar');
        writeln('4. Abrir y eliminar');
        writeln('5. Exportar a txt');
        writeln('0. Salir');
        write('Opcion: '); readln(opc);
        if (opc >= 2) and (opc <= 5) then begin
            write('Nombre del archivo: '); readln(nombre);
            assign(arc, nombre);
        end;
        case opc of
            1: crearArchivo(arc);
            2: altaLibro(arc);
            3: modificarLibro(arc);
            4: eliminarLibro(arc);
            5: exportarLibros(arc);
        end;
    until opc = 0;
end;

var
    arc: archivo;
begin
    menu(arc);
end.