{==============================================================================
  PRÁCTICA 4 - Ejercicio 1
  Fundamentos de Organización de Datos - UNLP
  Tema: Árbol B con datos directamente en el nodo (sin índice separado)
==============================================================================

  Enunciado:
  Considere que desea almacenar en un archivo la información correspondiente
  a los alumnos de la Facultad de Informática de la UNLP. De los mismos
  deberá guardarse nombre y apellido, DNI, legajo y año de ingreso.
  Suponga que dicho archivo se organiza como un árbol B de orden M.

  a. Defina en Pascal las estructuras de datos necesarias.
  b. Calcule M dado: nodo=512 bytes, registro=64 bytes, entero=4 bytes.
  c. ¿Qué impacto tiene almacenar toda la info en el árbol B?
  d. ¿Qué clave elegirías?
  e. Proceso de búsqueda por clave (mejor y peor caso).
  f. Búsqueda por criterio diferente al de ordenamiento.
==============================================================================}

{ ============================================================
  INCISO A: Definición de estructuras en Pascal
  ============================================================
  El árbol B de orden M almacena directamente los registros
  de alumnos en cada nodo. Cada nodo puede tener entre
  ceil(M/2)-1 y M-1 claves (excepto la raíz).
  Los hijos son punteros (NRR) dentro del mismo archivo.
  ============================================================ }

const
    M = 8; { Orden del árbol B (calculado en inciso b) }

type
    { Registro con la información de cada alumno }
    alumno = record
        nombre   : string[30]; { Nombre y apellido del alumno }
        dni      : longint;    { DNI - puede usarse como clave }
        legajo   : integer;    { Legajo - también puede ser clave }
        anioIng  : integer;    { Año de ingreso }
    end;

    { Nodo del árbol B: almacena directamente los registros }
    nodo = record
        cant_claves : integer;             { Cantidad de claves almacenadas actualmente }
        datos       : array[1..M-1] of alumno; { Registros de alumnos (claves + datos) }
        hijos       : array[1..M] of integer;  { NRR de los nodos hijos (-1 = sin hijo) }
    end;

    { Tipo del archivo del árbol B (nodos del árbol) }
    arbolB = file of nodo;

var
    archivoArbolB : arbolB; { El archivo árbol B contiene nodos con datos }


{ ============================================================
  INCISO B: Cálculo del orden M
  ============================================================

  Fórmula:  N = (M-1) * A + M * B + C

  Donde:
    N = tamaño del nodo en bytes         = 512
    A = tamaño de un registro (alumno)   = 64 bytes
    B = tamaño de cada enlace a un hijo  = 4 bytes (integer)
    C = tamaño del campo cant_claves     = 4 bytes (integer)

  Sustituyendo:
    512 = (M-1) * 64 + M * 4 + 4
    512 = 64M - 64 + 4M + 4
    512 = 68M - 60
    512 + 60 = 68M
    572 = 68M
    M = 572 / 68
    M = 8.41...  → se ignora la parte decimal → M = 8

  Respuesta:
    El orden del árbol B es M = 8.
    En un nodo de orden 8 entran como máximo M-1 = 7 registros de alumno.
    (En el mejor caso de ocupación, cada nodo puede tener entre 4 y 7 claves)
  ============================================================ }


{ ============================================================
  INCISO C: Impacto de almacenar toda la información en el árbol B
  ============================================================

  Al organizar el ARCHIVO COMPLETO como árbol B (registros completos
  almacenados en los nodos del árbol), el impacto sobre M es:

  1. MENOR ORDEN M: El registro de alumno (64 bytes) es mucho más
     grande que solo una clave (4 bytes para DNI como longint).
     Esto reduce drásticamente M (pasamos de M~43 con solo claves
     a M=8 con registros completos).

  2. ÁRBOL MÁS PROFUNDO: Con M=8 la capacidad por nodo es menor,
     lo que implica más niveles en el árbol para almacenar N alumnos.
     Más profundidad → más accesos a disco por operación.

  3. MÁS ACCESOS A DISCO: Cada nodo ocupa exactamente el tamaño de
     página (512 bytes), pero el árbol crece más en altura, aumentando
     el número de lecturas necesarias para llegar a un dato.

  4. VENTAJA: No se necesita un archivo de datos separado.
     Todo está en el árbol. Menor complejidad de implementación.

  CONCLUSIÓN: Mayor tamaño de registro → menor M → árbol más profundo
  → más lecturas por operación de búsqueda/inserción/eliminación.
  ============================================================ }


{ ============================================================
  INCISO D: Elección de clave de identificación
  ============================================================

  Se elegiría el DNI o el Legajo como clave, ya que:

  - DNI (Document Nacional de Identidad): Es único por persona a nivel
    nacional. Es un longint (4-8 bytes). Garantiza unicidad.

  - Legajo: Es único dentro de la Facultad de Informática de la UNLP.
    Es un número entero asignado al momento de la inscripción.
    También garantiza unicidad en el contexto de la facultad.

  RESPUESTA: Sí hay más de una opción. Tanto el DNI como el Legajo
  sirven como clave de identificación para el árbol B, dado que ambos
  son identificadores únicos para cada alumno.

  Se preferiría el DNI por ser inmutable y reconocido a nivel nacional.
  ============================================================ }


{ ============================================================
  INCISO E: Proceso de búsqueda por clave (DNI o Legajo)
  ============================================================

  PROCESO DE BÚSQUEDA EN ÁRBOL B:
  --------------------------------
  1. Leer la raíz del árbol (posición conocida, ej: NRR 0 o almacenada aparte).
  2. Dentro del nodo, buscar la clave (búsqueda lineal o binaria en el array).
     a. Si la clave coincide con datos[i].dni → ¡ENCONTRADO!
     b. Si la clave es menor que datos[i].dni → ir al hijo hijos[i-1].
     c. Si la clave es mayor que la última → ir al hijo hijos[cant_claves].
  3. Leer el nodo hijo indicado y repetir desde el paso 2.
  4. Si se llega a un hijo = -1 (nodo hoja sin ese hijo) → NO ENCONTRADO.

  MEJOR CASO: 1 lectura de nodo.
    → La clave buscada se encuentra en la RAÍZ del árbol.
    → Solo se lee el nodo raíz.

  PEOR CASO: h lecturas de nodo (h = altura del árbol).
    → La clave buscada se encuentra en un nodo HOJA.
    → Se recorre un camino desde la raíz hasta una hoja:
      raíz (nivel 1) → nodo nivel 2 → ... → hoja (nivel h).
    → Se necesitan exactamente h lecturas.
    → Otro peor caso: la clave NO existe, y se recorre hasta
      una hoja sin encontrarla (también h lecturas).
  ============================================================ }


{ ============================================================
  INCISO F: Búsqueda por criterio diferente al de ordenamiento
  ============================================================

  Si se quiere buscar por un campo que NO es la clave del árbol B
  (por ejemplo: buscar por nombre, o buscar por legajo si el árbol
  está organizado por DNI), el árbol B NO puede ser aprovechado
  para guiar la búsqueda.

  En este caso es necesario hacer una BÚSQUEDA SECUENCIAL:
  - Se deben leer TODOS los nodos del árbol, uno por uno.
  - Para cada nodo, se revisan todos sus registros.

  LECTURAS EN EL PEOR CASO: N lecturas (N = cantidad total de nodos).
    → Se deben examinar todos los nodos del árbol.
    → El registro buscado puede estar en el último nodo examinado,
      o puede no existir (ambos son el peor caso → N lecturas).

  SOLUCIÓN ALTERNATIVA: Crear un segundo árbol B con el otro criterio
  como clave (por ejemplo, un árbol B indexado por Legajo además del
  árbol B indexado por DNI). Esto se trata en el Ejercicio 2.
  ============================================================ }

begin
    { Este archivo es conceptual/estructural - no requiere código ejecutable }
    { Las definiciones de tipos y constantes están declaradas arriba }
end.
