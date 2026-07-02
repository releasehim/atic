{7. Se dispone de un archivo maestro con información de los alumnos...}
program ejercicio7;
const
    valoralto = 9999;
type
    regAlumno = record
        codigo: integer;
        apellido: string[50];
        nombre: string[50];
        cursadasAprobadas: integer;
        finalesAprobados: integer;
    end;
    
    regCursada = record
        codigoAlumno: integer;
        codigoMateria: integer;
        anio: integer;
        aprobada: boolean;
    end;
    
    regFinal = record
        codigoAlumno: integer;
        codigoMateria: integer;
        fecha: string[10];
        nota: integer;
    end;
    
    maestro = file of regAlumno;
    detalleCursadas = file of regCursada;
    detalleFinales = file of regFinal;

procedure leerCursada(var det: detalleCursadas; var reg: regCursada);
begin
    if (not eof(det)) then
        read(det, reg)
    else
        reg.codigoAlumno := valoralto;
end;

procedure leerFinal(var det: detalleFinales; var reg: regFinal);
begin
    if (not eof(det)) then
        read(det, reg)
    else
        reg.codigoAlumno := valoralto;
end;

procedure actualizarMaestro(var mae: maestro; var detCursadas: detalleCursadas; var detFinales: detalleFinales);
var
    alu: regAlumno;
    cur: regCursada;
    fin: regFinal;
    minCodigo: integer;
begin
    reset(mae);
    reset(detCursadas);
    reset(detFinales);
    
    leerCursada(detCursadas, cur);
    leerFinal(detFinales, fin);
    
    while (cur.codigoAlumno <> valoralto) or (fin.codigoAlumno <> valoralto) do
    begin
        if (cur.codigoAlumno < fin.codigoAlumno) then
            minCodigo := cur.codigoAlumno
        else
            minCodigo := fin.codigoAlumno;
            
        read(mae, alu);
        while (alu.codigo <> minCodigo) do
            read(mae, alu);
            
        while (cur.codigoAlumno = minCodigo) do
        begin
            if (cur.aprobada) then
                alu.cursadasAprobadas := alu.cursadasAprobadas + 1;
            leerCursada(detCursadas, cur);
        end;
        
        while (fin.codigoAlumno = minCodigo) do
        begin
            if (fin.nota >= 4) then
                alu.finalesAprobados := alu.finalesAprobados + 1;
            leerFinal(detFinales, fin);
        end;
        
        seek(mae, filepos(mae)-1);
        write(mae, alu);
    end;
    
    close(mae);
    close(detCursadas);
    close(detFinales);
end;

begin
    // El programa principal solo declara las variables y llama a los modulos si fuera necesario.
end.