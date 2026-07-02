{==============================================================================
  PRÁCTICA 4 - Ejercicio 4
  Fundamentos de Organización de Datos - UNLP
  Tema: Análisis y corrección del algoritmo de búsqueda en árbol B
==============================================================================

  Enunciado:
  Dado el siguiente algoritmo de búsqueda en un árbol B:

  procedure buscar(NRR, clave, NRR_encontrado, pos_encontrada, resultado)
  var clave_encontrada: boolean;
  begin
    if (nodo = null)
      resultado := false; {clave no encontrada}
    else
      posicionarYLeerNodo(A, nodo, NRR);
      claveEncontrada(A, nodo, clave, pos, clave_encontrada);
      if (clave_encontrada) then begin
        NRR_encontrado := NRR;
        pos_encontrada := pos;
        resultado := true;
      end
      else
        buscar(nodo.hijos[pos], clave, NRR_encontrado, pos_encontrada, resultado)
  end;

  a. PosicionarYLeerNodo(): qué hace y cómo enviar parámetros.
  b. claveEncontrada(): qué hace y cómo enviar parámetros.
  c. ¿Existe algún error? Corregir si es necesario.
  d. ¿Qué cambios son necesarios para árbol B+?
==============================================================================}

const
    M = 8; { Orden del árbol B (usado en los ejemplos) }

type
    alumno = record
        nombre   : string[30];
        dni      : longint;
        legajo   : integer;
        anioIng  : integer;
    end;

    nodo = record
        cant_claves : integer;
        datos       : array[1..M-1] of alumno;
        hijos       : array[1..M] of integer;
    end;

    arbolB = file of nodo;

{ ============================================================
  INCISO A: PosicionarYLeerNodo()
  ============================================================

  QUÉ HACE:
    Posiciona el cursor del archivo arbolB en la posición NRR
    (número de registro relativo) y lee el nodo almacenado en
    esa posición, cargándolo en la variable 'unNodo'.

  PARÁMETROS Y MODO DE ENVÍO:
    - A (archivo arbolB):    POR REFERENCIA (var).
        Motivo: los archivos en Pascal SIEMPRE se pasan por
        referencia. No se puede copiar un archivo.
    - unNodo (nodo):         POR REFERENCIA (var).
        Motivo: el procedimiento llena el nodo con los datos
        leídos del disco. Si fuera por valor, los cambios no
        se verían fuera del procedimiento.
    - NRR (integer):         POR VALOR.
        Motivo: solo se necesita conocer el número de posición.
        No es necesario modificarlo dentro del procedimiento.
  ============================================================ }

procedure posicionarYLeerNodo(var A: arbolB; var unNodo: nodo; NRR: integer);
begin
    { Posicionar el cursor del archivo en la posición NRR }
    seek(A, NRR);
    { Leer el nodo de esa posición y cargarlo en unNodo }
    read(A, unNodo);
end;


{ ============================================================
  INCISO B: claveEncontrada()
  ============================================================

  QUÉ HACE:
    Busca la clave 'clave' dentro del array de datos del nodo 'unNodo'.
    Si la encuentra, devuelve true y establece 'pos' con el índice
    donde se encontró la clave en el array.
    Si no la encuentra, devuelve false y 'pos' queda con el índice
    del hijo al que se debe bajar para continuar la búsqueda.

  PARÁMETROS Y MODO DE ENVÍO:
    - unNodo (nodo):           POR VALOR.
        Motivo: solo se lee el nodo, no se modifica.
        (En Pascal podría pasarse por referencia por eficiencia,
        pero conceptualmente es por valor ya que no se modifica)
    - clave (longint):         POR VALOR.
        Motivo: solo se usa su valor para comparar.
    - pos (integer):           POR REFERENCIA (var).
        Motivo: el resultado (posición encontrada o índice del hijo)
        debe ser visible fuera de la función.
    - Retorna boolean:         FUNCIÓN que devuelve true/false.
  ============================================================ }

function claveEncontrada(unNodo: nodo; clave: longint; var pos: integer): boolean;
var
    i: integer;
begin
    i := 1;
    { Búsqueda de la clave en el array }
    while (i <= unNodo.cant_claves) and (clave > unNodo.datos[i].dni) do begin
        i := i + 1;
    end;

    if (i <= unNodo.cant_claves) and (unNodo.datos[i].dni = clave) then begin
        pos := i;
        claveEncontrada := true;
    end else begin
        { Si clave < datos[i], baja por hijo i. Si recorrió todos, i = cant_claves + 1, baja por el último hijo }
        pos := i;
        claveEncontrada := false;
    end;
end;


{ ============================================================
  INCISO C: Errores en el código original y versión corregida
  ============================================================

  ERRORES ENCONTRADOS EN EL CÓDIGO ORIGINAL:

  1. FALTA DECLARACIÓN DE 'nodo' como variable local:
     El código usa la variable 'nodo' sin declararla. Dentro de
     'buscar' se llama posicionarYLeerNodo(A, nodo, NRR) pero
     'nodo' no está declarada en el var local del procedimiento.
     → Solución: declarar 'unNodo: nodo' como variable local.
     (También hay conflicto de nombres: 'nodo' es un tipo Y se usa
     como variable, lo que es ambiguo y genera error.)

  2. DEBERÍA SER FUNCTION NO PROCEDURE:
     El procedimiento retorna un resultado (resultado := true/false),
     pero en Pascal lo correcto es usar una function para retornar
     un valor booleano, o usar un parámetro var resultado.
     Si se usa procedure con var resultado, es válido pero el original
     no declara 'resultado' como parámetro var.

  3. FALTA begin-end ALREDEDOR DEL else:
     En Pascal, el bloque else de un if debe estar envuelto en
     begin-end si contiene más de una sentencia. El original tiene:
       else
         posicionarYLeerNodo(...)  { <-- sin begin-end }
         claveEncontrada(...)
     Esto es un error de sintaxis: solo la primera línea pertenece al else.
     → Solución: agregar begin-end alrededor del bloque else.

  4. VARIABLE 'pos' NO DECLARADA:
     La variable 'pos' se usa en claveEncontrada(..., pos, ...) pero
     no está declarada en el var del procedimiento buscar.
     → Solución: declarar 'pos: integer' como variable local.

  5. LLAMADA INCORRECTA A claveEncontrada:
     En el original se llama como procedure con parámetros (A, nodo, clave, pos, clave_encontrada),
     pero debería ser function que retorna boolean.
     → Solución: usar 'if claveEncontrada(unNodo, clave, pos) then ...'

  VERSIÓN CORREGIDA:
  ============================================================ }

{ Versión corregida del algoritmo de búsqueda }
function buscar(var A: arbolB; NRR: integer; clave: longint;
                var NRR_encontrado: integer; var pos_encontrada: integer): boolean;
var
    unNodo : nodo;    { Variable local para almacenar el nodo leído }
    pos    : integer; { Variable local para la posición dentro del nodo }
begin
    if NRR = -1 then begin
        { Nodo nulo: se llegó a un hijo vacío, la clave no existe }
        buscar := false;
    end else begin
        { Leer el nodo en la posición NRR del archivo }
        posicionarYLeerNodo(A, unNodo, NRR);

        if claveEncontrada(unNodo, clave, pos) then begin
            { La clave fue encontrada en este nodo }
            NRR_encontrado := NRR;    { NRR del nodo actual }
            pos_encontrada := pos;    { Posición dentro del array del nodo }
            buscar := true;
        end else begin
            { La clave no está en este nodo: bajar por el hijo indicado por pos }
            buscar := buscar(A, unNodo.hijos[pos], clave, NRR_encontrado, pos_encontrada);
        end;
    end;
end;


{ ============================================================
  INCISO D: Cambios para árbol B+
  ============================================================

  En un árbol B+, la búsqueda debe SIEMPRE TERMINAR EN UN NODO HOJA.
  Las claves en nodos internos son copias/separadores, no datos reales.
  El registro completo (o el enlace al archivo de datos) está solo en hojas.

  CAMBIOS NECESARIOS:

  1. DETECTAR SI EL NODO ES HOJA:
     Se necesita saber si se llegó a un nodo hoja. Esto se puede hacer:
     a. Añadiendo un campo 'esHoja: boolean' al registro nodo.
     b. Usando el campo 'sig': si sig >= -1, es hoja; si sig = -2, es interno.
     c. Verificando que todos los hijos sean -1 (indican nodo hoja).

  2. MODIFICAR LA LÓGICA DE BÚSQUEDA:
     - En nodos INTERNOS: solo se usan los separadores para guiar la
       búsqueda hacia el hijo correcto. Incluso si el separador coincide
       con la clave buscada, NO se retorna true: se sigue bajando
       hacia el hijo derecho de ese separador.
     - Solo en un nodo HOJA se verifica si la clave existe realmente
       y se retorna true con el enlace al registro.

  PSEUDOCÓDIGO DEL BUSCAR ADAPTADO PARA B+:

  function buscarBMas(var A: arbolBMas; NRR: integer; clave: longint;
                      var NRR_encontrado: integer; var pos_encontrada: integer): boolean;
  var unNodo: nodoBMas; pos: integer;
  begin
      if NRR = -1 then
          buscarBMas := false
      else begin
          posicionarYLeerNodo(A, unNodo, NRR);
          if unNodo.esHoja then begin
              { En hoja: verificar si la clave existe exactamente }
              if claveEncontradaEnHoja(unNodo, clave, pos) then begin
                  NRR_encontrado := NRR;
                  pos_encontrada := pos;
                  buscarBMas := true;
              end else
                  buscarBMas := false;
          end else begin
              { En nodo interno: solo usar separadores para guiar la búsqueda }
              encontrarHijoCorrespondiente(unNodo, clave, pos);
              buscarBMas := buscarBMas(A, unNodo.hijos[pos], clave, NRR_encontrado, pos_encontrada);
          end;
      end;
  end;

  DIFERENCIA FUNDAMENTAL:
  - En árbol B:  puede retornar true en cualquier nivel (nodo interno o hoja).
  - En árbol B+: solo retorna true desde un nodo HOJA.
                 En nodos internos siempre continúa la búsqueda hacia abajo.
  ============================================================ }

begin
    { Este archivo es conceptual/estructural y de definición de procedimientos }
    { Para ejecutar la búsqueda, se llamaría a buscar(archivoIdx, NRR_raiz, claveDeseada, ...) }
end.
