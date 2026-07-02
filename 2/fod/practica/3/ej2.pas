program ej2;
type
    producto = record
        codigo: integer;
        nombre: string;
        descripcion: string;
        precio: real;
        stock: integer;
    end;
    archivo = file of producto;

procedure leerProducto(var p: producto);
begin
    write('Codigo (-1 para fin): '); readln(p.codigo);
    if p.codigo <> -1 then begin
        write('Nombre: '); readln(p.nombre);
        write('Descripcion: '); readln(p.descripcion);
        write('Precio: '); readln(p.precio);
        write('Stock disponible: '); readln(p.stock);
    end;
end;

procedure crearArchivo(var arc: archivo);
var
    p: producto;
begin
    assign(arc, 'productos.dat');
    rewrite(arc);
    leerProducto(p);
    while p.codigo <> -1 do begin
        write(arc, p);
        leerProducto(p);
    end;
    close(arc);
end;

procedure bajaLogica(var arc: archivo);
var
    p: producto;
begin
    reset(arc);
    while not eof(arc) do begin
        read(arc, p);
        if p.stock = 0 then begin
            p.nombre := '@' + p.nombre;
            seek(arc, filepos(arc) - 1);
            write(arc, p);
        end;
    end;
    close(arc);
end;

var
    arc: archivo;
begin
    crearArchivo(arc);
    bajaLogica(arc);
end.