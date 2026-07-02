program ej5;
type
    prenda = record
        cod_prenda: integer;
        descripcion: string;
        colores: string;
        tipo_prenda: string;
        stock: integer;
        precio_unitario: real;
    end;
    arch_maestro = file of prenda;
    arch_detalle = file of integer;

procedure bajaLogica(var maestro: arch_maestro; var detalle: arch_detalle);
var
    p: prenda;
    cod_baja: integer;
    encontre: boolean;
begin
    reset(maestro);
    reset(detalle);
    while not eof(detalle) do begin
        read(detalle, cod_baja);
        seek(maestro, 0);
        encontre := false;
        while not eof(maestro) and not encontre do begin
            read(maestro, p);
            if p.cod_prenda = cod_baja then begin
                encontre := true;
                p.stock := -1; 
                seek(maestro, filepos(maestro) - 1);
                write(maestro, p);
            end;
        end;
    end;
    close(maestro);
    close(detalle);
end;

procedure efectivizarBajas(var maestro: arch_maestro);
var
    aux: arch_maestro;
    p: prenda;
begin
    reset(maestro);
    assign(aux, 'auxiliar.dat');
    rewrite(aux);
    while not eof(maestro) do begin
        read(maestro, p);
        if p.stock >= 0 then
            write(aux, p);
    end;
    close(maestro);
    close(aux);
    erase(maestro);
    rename(aux, 'maestro_prendas.dat'); 
end;

var
    maestro: arch_maestro;
    detalle: arch_detalle;
begin
    assign(maestro, 'maestro_prendas.dat');
    assign(detalle, 'detalle_bajas.dat');
    // bajaLogica(maestro, detalle);
    // efectivizarBajas(maestro);
end.