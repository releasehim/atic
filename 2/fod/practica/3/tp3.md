                   ​ Introducción a las Bases de Datos

             ​ Fundamentos de Organización de Datos


                                            ​     Práctica 3

                                                    ​


​   Parte 1: Bajas en archivos



1.​ Modificar el ejercicio 4 de la práctica 1 (programa de gestión de empleados) agregando una
    opción que permita realizar bajas físicas en el archivo. La baja debe realizarse a partir del número
    de empleado ingresado por teclado, identificando el registro correspondiente en el archivo. Una
    vez encontrado, se debe reemplazar el registro a eliminar por el último registro del archivo, y luego
    truncando el archivo en la posición del último registro de forma tal de evitar duplicados.




2.​ Definir un programa que genere un archivo con registros de longitud fija con información de
    productos de un comercio. Los datos se ingresan por teclado y de cada producto se almacena:
    código    de   producto,   nombre,    descripción, precio y stock disponible. Implementar un
    procedimiento que, a partir del archivo de datos generado, realice la baja lógica de todos
    aquellos productos cuyo stock disponible sea igual a 0.

    La baja lógica debe indicarse marcando el registro con un carácter especial que se sitúa como
    prefijo en algún campo de tipo string a su elección. Por ejemplo, se puede anteponer el carácter @
    al nombre del producto: ‘@Arroz Gallo 1K’.




3.​ Realizar un programa que gestione un archivo de libros de una librería. De cada libro se registra:
    código, género, título, autor, cantidad de páginas y precio. El programa debe presentar un menú
    con las siguientes opciones:

             a.​ Crear el archivo y cargarlo con datos ingresados por teclado, utilizando la técnica de
                lista invertida para recuperar espacio libre en el archivo.
         b.​ Abrir el archivo existente y permitir su mantenimiento mediante las siguientes
            operaciones:

                 i.​ Dar de alta un libro leyendo la información desde el teclado. Para esta
                    operación, en caso de ser posible, deberá recuperarse el espacio libre usando la
                    lista invertida.

                ii.​ Modificar los datos de un libro leyendo la información desde el teclado. El
                    código del libro no puede ser modificado.

                iii.​ Eliminar un libro cuyo código es ingresado por teclado.

         c.​ Exportar el contenido del archivo de libros a un archivo de texto llamados “libros.txt”,
            excluyendo los registros marcados como borrados.

NOTAS:

●​ Debe utilizar una lista invertida para la recuperación del espacio libre.
         ○​ El primer registro del archivo se utiliza como cabecera de la lista.
                ■​ El campo código de la cabecera tiene el valor cero (0) si no hay espacio libre.
                ■​ Si el campo código de la cabecera tiene un valor negativo, indica la posición del
                    primer registro a reutilizar.
         ○​ Los registros libres (aquellos marcados como borrados) utilizan el campo código como
            enlace, almacenando la posición en forma negativa del siguiente registro en la lista
            invertida
         ○​ En la operación de alta:
                ■​ Si la cabecera indica que hay espacio libre, se debe reutilizar el primer registro
                    disponible. Además, se debe actualizar la cabecera con la siguiente posición en
                    la lista invertida de espacios libres.
                ■​ Si la cabecera indica que no hay espacio libre, se debe agregar el nuevo registro
                    al final del archivo.
         ○​ En la operación de baja:
                ■​ El registro borrado se debe incorporar a la lista invertida de espacios libres. Al ser
                    una lista invertida (o pila), el último registro borrado es el próximo a ser reutilizado.
                    Para ello, en el registro borrado se almacena el valor actual de la cabecera,
                    mientras que la cabecera se actualiza con la posición (en valor negativo) del
                    registro borrado.
●​ Tanto en la creación como en la apertura el nombre del archivo debe ser proporcionado por el
   usuario.
4.​ Dada la siguiente estructura:

   ​​ type

  ​​ ​         reg_flor = record

           ​      nombre: String[45];
           ​      codigo: integer;
               end;

​ ​ ​          tArchFlores = file of reg_flor;

Se desea implementar un sistema de gestión de flores utilizando un archivo con reutilización de
espacio.

    ●​ Las bajas lógicas se realizan apilando los registros eliminados.
    ●​ Las altas deben reutilizar los espacios libres disponibles antes de agregar nuevos registros al final
         del archivo.
    ●​ El registro en la posición 0 se utiliza como cabecera de la pila de registros borrados.

   Política de reutilización:

    ●​ Si el campo código del registro cabecera es 0, significa que no hay registros borrados
         disponibles.
    ●​ Si el campo código es -N, indica que el próximo registro libre se encuentra en la posición N del
         archivo.
    ●​ Cada registro borrado debe almacenar en su campo codigo el valor negativo que apunte al
         siguiente registro libre, formando así una pila enlazada.


   a. Implementación requerida

         Implementar el siguiente módulo:

           { Abre el archivo y agrega una flor, recibida como parámetro,

            respetando la política de reutilización de espacio descripta }

           procedure agregarFlor (var a: tArchFlores; nombre: string; codigo: integer);


   b. Listado del archivo

         Realizar un procedimiento que liste el contenido del archivo omitiendo las flores eliminadas (es
         decir, aquellos registros que forman parte de la pila de libres).
       Se permite modificar o agregar estructuras auxiliares si se considera necesario para obtener
       correctamente el listado.


   c. Implemente el siguiente módulo:

   {Abre el archivo y elimina la              flor   recibida     como   parámetro    manteniendo     la
   política descripta anteriormente}

   procedure eliminarFlor (var a: tArchFlores; flor:reg_flor);



5.​ Una cadena de tiendas de indumentaria dispone de un archivo maestro no ordenado que

   contiene la información de las prendas que se encuentran a la venta. De cada prenda se registran

   los siguientes datos: cod_prenda, descripción, colores, tipo_prenda, stock y precio_unitario.


   Debido a un cambio de temporada, es necesario actualizar las prendas disponibles. Para ello, se

   recibe un archivo detalle que contiene los códigos (cod_prenda) de aquellas prendas que

   quedarán obsoletas. Se deberá implementar un procedimiento que reciba ambos archivos y

   realice la baja lógica de las prendas indicadas; para ello, se deberá modificar el campo stock de la

   prenda correspondiente asignándole un valor negativo como marca de eliminación.


   Adicionalmente, se deberá implementar otro procedimiento que permita efectivizar las bajas

   lógicas realizadas sobre el archivo maestro. Para ello, se deberá crear un archivo auxiliar en el cual

   se copien únicamente aquellas prendas que no estén marcadas como eliminadas (es decir,

   aquellas cuyo stock sea mayor o igual a cero).


   Finalmente, una vez completado el proceso de compactación, el archivo auxiliar deberá

   reemplazar al archivo maestro original, adoptando su mismo nombre.


6.​ Se cuenta con un archivo que almacena información sobre especies de aves en peligro de

   extinción. De cada especie se registran los siguientes datos: código, nombre de la especie, familia,

   descripción y zona geográfica. El archivo no se encuentra ordenado por ningún criterio.


   Se desea desarrollar un programa que permita eliminar especies de aves extintas. Para ello, el

   programa deberá contar con dos procedimientos:


   Un procedimiento que, dado el código de una especie, la marque como borrada (baja lógica). En

   caso de querer eliminar múltiples especies, este procedimiento podrá invocarse repetidamente.
   Un   procedimiento    que    realice   la   compactación    del   archivo     (baja física), eliminando

   definitivamente aquellas especies marcadas como borradas. Para ello, cada vez que se elimine un

   registro, se deberá reemplazar su posición con el último registro del archivo y luego eliminar dicho

   último registro, evitando así dejar espacios vacíos y registros duplicados.


   Implemente además una variante de este procedimiento de compactación en la cual el archivo

   sea truncado una única vez al finalizar el proceso.


7.​ Se cuenta con un archivo con información de las diferentes distribuciones de linux existentes. De

   cada distribución se conoce: nombre, año de lanzamiento, número de versión del kernel, cantidad

   de desarrolladores y descripción. El nombre de las distribuciones no puede repetirse. Este archivo

   debe ser mantenido realizando bajas lógicas y utilizando la técnica de reutilización de espacio libre

   llamada lista invertida. Escriba la definición de las estructuras de datos necesarias y los siguientes

   procedimientos:

           a.​ BuscarDistribucion: módulo que recibe por parámetro el archivo, un nombre de

              distribución y devuelve la posición dentro del archivo donde se encuentra el registro

              correspondiente a la distribución dada (si existe) o devuelve -1 en caso de que no

              exista..

           b.​ AltaDistribucion: módulo que recibe como parámetro el archivo y el registro que

              contiene los datos de una nueva distribución, y se encarga de agregar la distribución al

              archivo reutilizando espacio disponible en caso de que exista. El control de unicidad lo

              debe realizar utilizando el módulo anterior. En caso de que la distribución que se quiere

              agregar ya exista se debe informar “ya existe la distribución”.

           c.​ BajaDistribucion: módulo que recibe como parámetro el archivo y el nombre de una

              distribución, y se encarga de dar de baja lógicamente la distribución dada. Para marcar

              una distribución como borrada se debe utilizar el campo cantidad de desarrolladores

              para mantener actualizada la lista invertida. Para verificar que la distribución a borrar

              exista debe utilizar el módulo BuscarDistribucion. En caso de no existir se debe informar

              “Distribución no existente”.
Parte 2: Actualización maestro/detalle, reportes y merge con archivos no ordenados

Para los ejercicios de esta parte de la práctica, teniendo en cuenta que los archivos no están
ordenados por ningún criterio, puede resultar necesario recorrer los archivos más de una vez. La idea
es resolver los ejercicios sin ordenar los archivos dados, y comparar la eficiencia (en cuanto al número
de lecturas/escrituras) de la solución brindada en esta práctica respecto a la solución para el mismo
problema considerando los archivos ordenados.

1.​ El encargado de ventas de un negocio de productos de limpieza desea administrar el stock de los
   productos que vende. Para ello, genera un archivo maestro donde figuran todos los productos que
   comercializa. De cada producto se maneja la siguiente información: código de producto, nombre
   comercial, precio de venta, stock actual y stock mínimo. Diariamente se genera un archivo detalle
   donde se registran todas las ventas de productos realizadas. De cada venta se registran: código de
   producto y cantidad de unidades vendidas. Resuelve los siguientes puntos:

       a.​ Se pide realizar un procedimiento que actualice el archivo maestro con el archivo detalle,
          teniendo en cuenta que:

             i.​     Los archivos no están ordenados por ningún criterio.
             ii.​    Cada registro del maestro puede ser actualizado por 0, 1 ó más registros del archivo
                     detalle.

       b.​ ¿Qué cambios realizaría en el procedimiento del punto anterior si se sabe que cada registro
          del archivo maestro puede ser actualizado por 0 o 1 registro del archivo detalle?

2.​ Se necesita contabilizar los votos de las diferentes mesas electorales registradas por localidad en la
   provincia de Buenos Aires. Para ello, se posee un archivo con la siguiente información: código de
   localidad, número de mesa y cantidad de votos en dicha mesa. Presentar en pantalla un listado
   como se muestra a continuación:

          Código de Localidad                    ​ Total de Votos

          ................................   ​     ......................

          ................................   ​    ......................

          Total General de Votos:                ​ ………………

    NOTAS:

        ●​ La información en el archivo no está ordenada por ningún criterio.
        ●​ Trate de resolver el problema sin modificar el contenido del archivo dado.
        ●​ Puede utilizar una estructura auxiliar, como por ejemplo otro archivo, para llevar el control
            de las localidades que han sido procesadas.
3.​ Suponga que trabaja en una oficina donde está montada una LAN (red local). La misma fue

   construida sobre una topología de red que conecta 5 máquinas entre sí y todas las máquinas se

   conectan con un servidor central. Semanalmente cada máquina genera un archivo de logs

   informando las sesiones abiertas por cada usuario en cada terminal y por cuánto tiempo estuvo

   abierta. Cada archivo detalle contiene los siguientes campos: cod_usuario, fecha, tiempo_sesion.

   Debe realizar un procedimiento que reciba los archivos detalle y genere un archivo maestro con los

   siguientes datos: cod_usuario, fecha, tiempo_total_de_sesiones_abiertas.

   Notas:

      ●​ Los archivos detalle no están ordenados por ningún criterio.
      ●​ Un usuario puede iniciar más de una sesión el mismo día en la misma máquina, o inclusive, en
            diferentes máquinas.
