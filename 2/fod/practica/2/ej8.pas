program ejercicio8;

const
    DF = 16;
    valoralto = 9999;

type
    provincia = record
        codigo: integer;
        nombre: string[30];
        habitantes: integer;
        kilos: real;
    end;

    detalle_reg = record
        codigo: integer;
        kilos: real;
    end;

    maestro = file of provincia;
    detalle = file of detalle_reg;

    vecDetalles = array [1..DF] of detalle;
    vecRegistros = array [1..DF] of detalle_reg;

procedure leer(var det: detalle; var infoDet: detalle_reg);
begin
    if (not eof(det)) then
        read(det, infoDet)
    else
        infoDet.codigo := valoralto;
end;

procedure minimo(var vec: vecDetalles; var vecReg: vecRegistros; var min: detalle_reg);
var
    i, pos: integer;
begin
    min.codigo := valoralto;
    pos := 0;
    for i := 1 to DF do begin
        if (vecReg[i].codigo < min.codigo) then begin
            min := vecReg[i];
            pos := i;
        end;
    end;
    if (pos <> 0) then
        leer(vec[pos], vecReg[pos]);
end;

procedure actualizarMaestro(var mae: maestro; var vec: vecDetalles);
var
    min: detalle_reg;
    infoMae: provincia;
    vecReg: vecRegistros;
    i: integer;
    totalKilos: real;
begin
    reset(mae);
    for i := 1 to DF do begin
        reset(vec[i]);
        leer(vec[i], vecReg[i]);
    end;

    minimo(vec, vecReg, min);

    if not eof(mae) then
        read(mae, infoMae)
    else
        infoMae.codigo := valoralto;

    while (min.codigo <> valoralto) do begin
        // Process unmodified master records that come before the current minimum detail
        while (infoMae.codigo < min.codigo) and (infoMae.codigo <> valoralto) do begin
            if (infoMae.kilos > 10000) then begin
                writeln('Provincia: ', infoMae.nombre, ' (Codigo: ', infoMae.codigo, ') - Consumo Promedio: ', (infoMae.kilos / infoMae.habitantes):0:2);
            end;
            if not eof(mae) then
                read(mae, infoMae)
            else
                infoMae.codigo := valoralto;
        end;

        // Process matching records
        if (infoMae.codigo = min.codigo) then begin
            totalKilos := 0;
            while (min.codigo = infoMae.codigo) do begin
                totalKilos := totalKilos + min.kilos;
                minimo(vec, vecReg, min);
            end;
            infoMae.kilos := infoMae.kilos + totalKilos;

            if (infoMae.kilos > 10000) then begin
                writeln('Provincia: ', infoMae.nombre, ' (Codigo: ', infoMae.codigo, ') - Consumo Promedio: ', (infoMae.kilos / infoMae.habitantes):0:2);
            end;

            // Rewrite the updated record
            seek(mae, filepos(mae) - 1);
            write(mae, infoMae);

            // Read the next record
            if not eof(mae) then
                read(mae, infoMae)
            else
                infoMae.codigo := valoralto;
        end;
    end;

    // Process remaining unmodified master records
    while (infoMae.codigo <> valoralto) do begin
        if (infoMae.kilos > 10000) then begin
            writeln('Provincia: ', infoMae.nombre, ' (Codigo: ', infoMae.codigo, ') - Consumo Promedio: ', (infoMae.kilos / infoMae.habitantes):0:2);
        end;
        if not eof(mae) then
            read(mae, infoMae)
        else
            infoMae.codigo := valoralto;
    end;

    close(mae);
    for i := 1 to DF do
        close(vec[i]);
end;

var
    vecDet: vecDetalles;
    mae: maestro;
    i: integer;
    numStr: string;
begin
    assign(mae, 'maestro');
    for i := 1 to DF do begin
        str(i, numStr);
        assign(vecDet[i], 'detalle' + numStr);
    end;

    actualizarMaestro(mae, vecDet);
end.
