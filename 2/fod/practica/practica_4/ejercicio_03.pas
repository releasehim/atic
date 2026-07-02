{==============================================================================
  PRÁCTICA 4 - Ejercicio 3
  Fundamentos de Organización de Datos - UNLP
  Tema: Árbol B+ como índice con hojas enlazadas
==============================================================================

  Enunciado:
  Los árboles B+ representan una mejora sobre los árboles B dado que
  conservan la propiedad de acceso indexado a los registros del archivo
  de datos por alguna clave, pero permiten además un recorrido secuencial
  rápido. Analice ventajas y desventajas de mantener en las hojas los datos.

  a. ¿Cómo se organizan los elementos en un árbol B+?
  b. ¿Qué característica distintiva presentan los nodos hojas?
  c. Definir estructuras en Pascal para álbum de alumnos + índice B+.
  d. Proceso de búsqueda por DNI en B+ vs B.
  e. Búsqueda por rango [40000000..45000000] en B+.
  f. Pros y contras de tener datos en las hojas.
==============================================================================}

{ ============================================================
  INCISO A: Organización de elementos en un árbol B+
  ============================================================

  En un ÁRBOL B+, los elementos se organizan de la siguiente manera:

  NODOS INTERNOS (no hoja):
    - Solo contienen CLAVES SEPARADORAS (copias de claves).
    - Su función es GUIAR LA BÚSQUEDA hacia el subárbol correcto.
    - NO contienen datos reales ni enlaces al archivo de datos.
    - Una clave en un nodo interno es la COPIA de la clave menor
      del subárbol derecho (o la clave separadora que divide los hijos).

  NODOS HOJA:
    - Contienen TODAS LAS CLAVES REALES del árbol (no copias).
    - Cada clave en un nodo hoja va acompañada de su ENLACE (NRR)
      al registro correspondiente en el archivo de datos.
    - Están ordenados: todas las claves de una hoja son menores
      que todas las claves de la hoja siguiente.
    - Están enlazados entre sí mediante punteros (campo 'sig').

  PROPIEDAD CLAVE:
    - TODAS las claves del archivo aparecen en las hojas.
    - Algunas claves también aparecen como separadores en nodos internos
      (pero son copias, no los datos reales).
    - Esto garantiza que la búsqueda SIEMPRE termina en una hoja.
  ============================================================ }


{ ============================================================
  INCISO B: Característica distintiva de los nodos hoja
  ============================================================

  La característica más importante de los nodos hoja en un árbol B+ es
  que están ENLAZADOS ENTRE SÍ mediante punteros, formando una
  LISTA ENLAZADA ORDENADA de todas las hojas del árbol.

  ¿POR QUÉ están enlazados?
    - Para permitir RECORRIDO SECUENCIAL ORDENADO de todos los registros
      sin necesidad de volver a la raíz y re-traversar el árbol.
    - Facilita las BÚSQUEDAS POR RANGO: una vez encontrada la primera
      clave del rango en una hoja, se puede avanzar hoja por hoja
      siguiendo los punteros 'sig' hasta salir del rango.
    - Esto simula el comportamiento de un archivo secuencial ordenado
      manteniendo a la vez el acceso directo eficiente del árbol B+.

  El último nodo hoja apunta a -1 (o nil) indicando el fin de la lista.
  ============================================================ }


{ ============================================================
  INCISO C: Definición de estructuras en Pascal (árbol B+ simplificado)
  ============================================================
  Se asume que todos los nodos (internos y hoja) tienen el mismo tamaño.
  La distinción entre nodo interno y nodo hoja se hace con el campo 'sig':
    - Si sig >= 0: es un nodo hoja (apunta al siguiente hoja).
    - Si sig = -1 y es hoja: es la última hoja.
    - Los nodos internos tienen sig = -2 (o algún valor especial).
  O bien se puede agregar un campo booleano 'esHoja'.
  ============================================================ }

const
    M = 43; { Orden del árbol B+ (mismo cálculo que ejercicio 2, índice separado) }

type
    { Registro completo del alumno - va al archivo de datos }
    alumno = record
        nombre   : string[30];
        dni      : longint;
        legajo   : integer;
        anioIng  : integer;
    end;

    { Nodo del árbol B+ (tanto internos como hojas comparten estructura) }
    nodoBMas = record
        cant_claves : integer;              { Cantidad de claves en este nodo }
        claves      : array[1..M-1] of longint;  { Claves (DNI) }
        enlaces     : array[1..M-1] of integer;  { NRR en archivoDatos (solo en hojas) }
        hijos       : array[1..M] of integer;    { NRR de nodos hijos (-1 = hoja sin ese hijo) }
        sig         : integer;              { NRR del siguiente nodo hoja (-1 = no hay más) }
                                            { En nodos internos: sig = -2 (no aplica) }
    end;

    { Tipo del archivo de datos (alumnos en orden de llegada) }
    archivoDatos = file of alumno;

    { Tipo del archivo índice B+ }
    arbolBMas = file of nodoBMas;

var
    datos   : archivoDatos;
    indiceBMas : arbolBMas;

{ NOTA SOBRE EL TAMAÑO DEL NODO B+:
  Un nodo B+ tiene el mismo cálculo de M que el árbol B del ejercicio 2
  (si solo claves e índices) porque los nodos internos no tienen enlaces
  al archivo de datos.
  Para simplicidad se usa la misma estructura para ambos tipos de nodo,
  pero en la práctica los nodos internos no usan el campo 'enlaces' ni 'sig'.
}


{ ============================================================
  INCISO D: Búsqueda de alumno con DNI específico en árbol B+
  ============================================================

  PROCESO EN ÁRBOL B+:
  1. Comenzar en la raíz.
  2. Recorrer los nodos INTERNOS comparando la clave buscada
     con los separadores del nodo:
     - Si clave < separador[i] → ir a hijos[i-1].
     - Si clave >= separador[i] → continuar comparando hacia la derecha.
  3. Continuar bajando hasta llegar a un NODO HOJA.
     (Condición: hijos[0] = -1, o campo esHoja = true)
  4. En el nodo hoja, buscar la clave exacta en claves[]:
     - Si se encuentra: usar enlaces[i] para acceder al registro
       en archivoDatos. ¡ENCONTRADO!
     - Si no se encuentra: ¡NO EXISTE la clave!

  DIFERENCIA CLAVE con árbol B:
  - En árbol B: la búsqueda PUEDE terminar en un nodo INTERNO
    si la clave está allí. Se retorna el dato directamente.
  - En árbol B+: la búsqueda SIEMPRE termina en un nodo HOJA,
    independientemente de si la clave aparece también en un nodo
    interno como separador. Los separadores internos son solo copias.
    Esto garantiza un comportamiento más predecible: el costo es
    SIEMPRE exactamente h lecturas (altura del árbol).

  CANTIDAD DE LECTURAS:
  - Árbol B:  entre 1 y h (puede terminar antes si está en nodo interno).
  - Árbol B+: siempre exactamente h (siempre llega hasta una hoja).
  ============================================================ }


{ ============================================================
  INCISO E: Búsqueda por rango [40000000 .. 45000000] en árbol B+
  ============================================================

  PROCESO EFICIENTE EN ÁRBOL B+:

  Paso 1 - Buscar la primera clave del rango (40000000):
    - Recorrer desde la raíz hasta una hoja: O(h) lecturas.
    - Llegar al nodo hoja que contiene (o debería contener) 40000000.

  Paso 2 - Recorrer hojas hacia la derecha:
    - Desde la hoja encontrada, procesar todas las claves >= 40000000.
    - Cuando se agotan las claves de la hoja actual, seguir por sig:
      leer el siguiente nodo hoja (1 lectura adicional).
    - Continuar hasta encontrar una clave > 45000000 o llegar a sig = -1.

  COSTO TOTAL: h + ceil(k / capacidad_hoja) lecturas
    donde k = cantidad de claves en el rango [40M, 45M].

  VENTAJA SOBRE ÁRBOL B:
  - En árbol B, para cada clave del rango habría que re-traversar
    el árbol desde la raíz: costaría k * h lecturas.
  - En árbol B+, el enlace entre hojas permite recorrer el rango
    en forma secuencial: h + pocas lecturas adicionales de hojas.
  - La mejora es especialmente notable cuando k (tamaño del rango) es grande.
  ============================================================ }


{ ============================================================
  INCISO F: Pros y contras de tener datos en las hojas
  ============================================================

  [En este ejercicio el enunciado dice "mantener en las hojas los datos"]
  Esto puede referirse a dos variantes:

  VARIANTE 1: Datos COMPLETOS en las hojas (árbol B+ con datos en hojas)
    PROS:
      + Acceso secuencial eficiente: recorrer hojas enlazadas = recorrer
        datos ordenados sin necesidad de acceder a archivo de datos externo.
      + Una sola lectura de hoja da el dato completo.
      + No necesita archivo de datos separado.
    CONTRAS:
      - Hojas más grandes → menor M para las hojas → árbol más profundo
        (similar al problema del ejercicio 1).
      - Duplicación: claves en nodos internos son copias, y los datos
        completos en hojas. Mayor uso de espacio.

  VARIANTE 2: Datos (registros completos) en archivo separado, solo clave+enlace en hoja
    PROS:
      + M grande tanto en internos como en hojas → árbol muy "chato".
      + Recorrido secuencial eficiente siguiendo los enlaces entre hojas.
      + Búsquedas por rango muy eficientes.
      + El archivo de datos puede estar en cualquier orden.
    CONTRAS:
      - Una lectura adicional al archivo de datos al recuperar el registro.
      - Mayor complejidad de implementación (dos archivos).

  CONCLUSIÓN GENERAL:
    El árbol B+ (con datos en archivo separado y solo claves+enlaces en hojas)
    es el balance ideal: índice pequeño (M grande), acceso secuencial eficiente
    y búsquedas por rango con bajo costo.
  ============================================================ }

begin
    { Este archivo es conceptual/estructural - no requiere código ejecutable }
end.
