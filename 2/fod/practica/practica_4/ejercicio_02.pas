{==============================================================================
  PRÁCTICA 4 - Ejercicio 2
  Fundamentos de Organización de Datos - UNLP
  Tema: Archivo de datos separado + Índice árbol B por DNI
==============================================================================

  Enunciado:
  Una mejora respecto al ejercicio 1: mantener por un lado el archivo que
  contiene la información de los alumnos (archivo de datos NO ordenado)
  y por otro lado un índice al archivo de datos estructurado como árbol B
  que ofrece acceso indexado por DNI de los alumnos.

  a. Defina en Pascal las estructuras para el archivo de alumnos y el índice.
  b. Calcule M para el índice dado: nodo=512 bytes, entero=4 bytes.
     (Considerar M-1 enlaces a registros en el archivo de datos)
  c. ¿Qué implica que el orden del árbol B sea mayor que en el ejercicio 1?
  d. Describe el proceso para buscar el alumno con DNI 12345678.
  e. ¿Qué ocurre si se quiere buscar por legajo?
  f. Búsquedas por rango de DNI [40000000..45000000].
==============================================================================}

{ ============================================================
  INCISO A: Definición de estructuras en Pascal
  ============================================================
  SEPARACIÓN en dos archivos:
   - archivoDatos: almacena los registros completos de alumnos
     de forma NO ordenada (se accede por NRR = posición física).
   - archivoIndice: árbol B que solo almacena claves (DNI) +
     enlace al NRR en archivoDatos + punteros a hijos.
  ============================================================ }

const
    M = 43; { Orden del árbol B índice (calculado en inciso b) }

type
    { Registro completo del alumno - va al archivo de datos }
    alumno = record
        nombre   : string[30]; { Nombre y apellido }
        dni      : longint;    { DNI del alumno }
        legajo   : integer;    { Legajo asignado por la facultad }
        anioIng  : integer;    { Año de ingreso }
    end;

    { Nodo del ÍNDICE (árbol B): solo claves + enlaces + hijos }
    { NO almacena el registro completo, solo el DNI como clave }
    nodoIndice = record
        cant_claves : integer;              { Cantidad de claves en este nodo }
        claves      : array[1..M-1] of longint;  { DNI de los alumnos (clave de búsqueda) }
        enlaces     : array[1..M-1] of integer;  { NRR en archivoDatos para cada clave }
        hijos       : array[1..M] of integer;    { NRR de nodos hijos en archivoIndice (-1=hoja) }
    end;

    { Tipo del archivo de datos (contiene registros de alumnos) }
    archivoDatos = file of alumno;

    { Tipo del archivo índice (contiene nodos del árbol B) }
    arbolB = file of nodoIndice;

var
    datos  : archivoDatos;  { Archivo de datos: alumnos en orden de llegada }
    indice : arbolB;        { Archivo índice: árbol B por DNI }


{ ============================================================
  INCISO B: Cálculo del orden M para el índice
  ============================================================

  Fórmula adaptada (el nodo del índice contiene, por cada clave:
    - la clave (DNI): longint = 4 bytes
    - el enlace al archivo de datos (NRR): integer = 4 bytes
    - los punteros a hijos: integer = 4 bytes):

  Tamaño del nodo = (M-1)*tam_clave + (M-1)*tam_enlace + M*tam_hijo + tam_cant
  512 = (M-1)*4 + (M-1)*4 + M*4 + 4
  512 = 4M - 4 + 4M - 4 + 4M + 4
  512 = 12M - 4
  512 + 4 = 12M
  516 = 12M
  M = 516 / 12
  M = 43

  Respuesta: El orden del árbol B índice es M = 43.
  Cada nodo del índice puede almacenar hasta M-1 = 42 claves (DNI).

  COMPARACIÓN con Ejercicio 1:
    - Ejercicio 1 (datos completos): M = 8  (nodo almacena registros de 64 bytes)
    - Ejercicio 2 (solo índice):     M = 43 (nodo almacena solo claves de 4+4 bytes)
  ============================================================ }


{ ============================================================
  INCISO C: Implicancias de un mayor orden M
  ============================================================

  Al separar el índice de los datos, el orden M aumenta de 8 a 43.
  Esto tiene las siguientes implicancias:

  1. ÁRBOL MÁS ANCHO Y MENOS PROFUNDO:
     Con M=43, cada nodo puede tener hasta 42 claves y 43 hijos.
     El árbol necesita muchos menos niveles para indexar el mismo
     número de registros. Altura ≈ log_43(N) vs log_8(N).

  2. MENOS LECTURAS POR OPERACIÓN:
     Al ser el árbol más "chato", hay menos accesos a disco para
     buscar, insertar o eliminar una clave. Esto mejora notablemente
     el rendimiento de las operaciones de acceso.

  3. MEJOR APROVECHAMIENTO DE LA CACHÉ:
     Nodos más ricos en claves permiten que con una sola lectura se
     descarte una gran porción del árbol.

  4. LECTURA ADICIONAL AL ARCHIVO DE DATOS:
     Al encontrar la clave en el índice, se debe hacer una lectura
     adicional al archivoDatos para obtener el registro completo.
     Pero esta lectura directa por NRR es O(1) (acceso aleatorio).

  CONCLUSIÓN: Mayor M → árbol menos profundo → menos lecturas de disco
  → mejor rendimiento general, a costa de una lectura adicional al
  archivo de datos al momento de recuperar el registro completo.
  ============================================================ }


{ ============================================================
  INCISO D: Proceso de búsqueda del alumno con DNI 12345678
  ============================================================

  PROCESO PASO A PASO:
  1. Abrir archivoIndice. Leer el nodo RAÍZ (NRR conocido, ej: nodo 0
     o almacenado en una variable de control del archivo).

  2. Dentro del nodo raíz, buscar la clave 12345678 en el array claves[]:
     a. Busca secuencialmente (o binarialmente) en claves[1..cant_claves].
     b. Si claves[i] = 12345678 → ¡CLAVE ENCONTRADA en el índice!
        Usar enlaces[i] como NRR en archivoDatos.
        Leer el registro del alumno de archivoDatos en esa posición.
        Fin de búsqueda.
     c. Si 12345678 < claves[1] → seguir por hijos[0] (subárbol izquierdo).
     d. Si claves[i] < 12345678 < claves[i+1] → seguir por hijos[i].
     e. Si 12345678 > claves[cant_claves] → seguir por hijos[cant_claves].

  3. Leer el nodo hijo indicado y repetir desde el paso 2.

  4. Si se llega a hijos[x] = -1 (nodo hoja, no hay hijo) → NO ENCONTRADO.

  RESUMEN DE LECTURAS:
    - 1 lectura del índice por nivel del árbol (como máximo h lecturas).
    - 1 lectura del archivoDatos al encontrar la clave.
    - Total peor caso: h + 1 lecturas (h = altura del árbol).
  ============================================================ }


{ ============================================================
  INCISO E: Búsqueda por número de legajo
  ============================================================

  El índice árbol B está organizado por DNI como clave.
  Si se quiere buscar por LEGAJO, este índice NO es útil para
  guiar la búsqueda, ya que los nodos están ordenados por DNI.

  OPCIONES:

  1. BÚSQUEDA SECUENCIAL (sin índice adicional):
     - Recorrer todos los nodos del árbol B leyendo todos los registros,
       o bien leer secuencialmente el archivoDatos completo.
     - Costo: O(N) lecturas (N = cantidad de alumnos o nodos).
     - Ineficiente para archivos grandes.

  2. CREAR UN SEGUNDO ÍNDICE POR LEGAJO:
     - Mantener un segundo árbol B con clave = legajo y valor = NRR
       del registro en archivoDatos.
     - Este segundo árbol permite acceso eficiente O(log M N) por legajo.
     - Costo adicional: espacio para el segundo árbol y mantenimiento
       de ambos índices al insertar/eliminar alumnos.

  CONCLUSIÓN: Para acceso eficiente por legajo se necesita un índice
  adicional organizado por legajo. Un archivo puede tener múltiples
  índices sobre diferentes campos.
  ============================================================ }


{ ============================================================
  INCISO F: Búsquedas por rango de DNI [40000000 .. 45000000]
  ============================================================

  PROBLEMA CON ÁRBOL B PARA BÚSQUEDAS POR RANGO:

  En un árbol B, al encontrar una clave que cae en el rango, la
  búsqueda del SIGUIENTE valor en el rango no es trivial porque:

  1. Los nodos hoja NO están enlazados entre sí.
  2. Después de encontrar una clave en el rango, para buscar la
     siguiente se debe volver a la raíz y realizar otra búsqueda.
  3. Claves del rango pueden estar distribuidas en distintas ramas
     del árbol (izquierda y derecha del valor medio).

  PROCESO (ineficiente) para rango en árbol B:
    a. Buscar 40000000 → costo O(h).
    b. Procesar los resultados del rango en ese camino.
    c. Para la siguiente clave del rango, hay que re-traversar el árbol.
    d. Esto puede implicar k * h lecturas donde k = claves en el rango.

  ALTERNATIVA EFICIENTE: Usar un ÁRBOL B+ en lugar de árbol B.
    - En el árbol B+, todos los nodos hoja están enlazados en una
      lista doblemente (o simplemente) enlazada.
    - Para un rango [40M, 45M]:
        1. Buscar 40000000 → llegar a la hoja que lo contiene: O(h).
        2. Recorrer los enlaces de hoja en hoja hacia la derecha
           hasta superar 45000000: O(k/capacidad_hoja).
    - Mucho más eficiente para búsquedas por rango.
    - Esto se estudia en el Ejercicio 3.
  ============================================================ }

begin
    { Este archivo es conceptual/estructural - no requiere código ejecutable }
end.
