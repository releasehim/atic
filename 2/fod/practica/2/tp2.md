                         ​ Introducción a las Bases de Datos

                  ​ Fundamentos de Organización de Datos

                                                    ​     Práctica 2

                                  Archivos Secuenciales ordenados - Algorítmica Clásica

 1.​ Una empresa posee un archivo que contiene información sobre los ingresos percibidos por diferentes
     empleados en concepto de comisión. De cada empleado se conoce: código de empleado, nombre y
     monto de la comisión.

         La información del archivo se encuentra ordenada por código de empleado, y cada empleado puede
         aparecer más de una vez en el archivo de comisiones.

         Se solicita realizar un procedimiento que reciba el archivo anteriormente descrito y lo compacte. Como
         resultado, deberá generar un nuevo archivo en el cual cada empleado aparezca una única vez, con el
         valor total acumulado de sus comisiones.

         Nota: No se conoce a priori la cantidad de empleados. Además, el archivo debe ser recorrido una única
         vez.

2.​     El encargado de ventas de un negocio de productos de limpieza desea administrar el stock de los productos
       que comercializa. Para ello, dispone de un archivo maestro en el que se registran todos los productos.

       De cada producto se almacena la siguiente información: código de producto, nombre comercial, precio de venta,
      stock actual y stock mínimo.

      Diariamente se genera un archivo detalle donde se registran todas las ventas realizadas. De cada venta se
      almacena: código de producto y cantidad de unidades vendidas.

      Se solicita desarrollar un programa que permita:

      a) Actualizar el archivo maestro a partir del archivo detalle, teniendo en cuenta que:

         ●​ Ambos archivos están ordenados por código de producto.



                                                                                                                  1
    ●​ Cada registro del archivo maestro puede ser actualizado por cero, uno o más registros del archivo
       detalle.
    ●​ El archivo detalle sólo contiene registros cuyos códigos existen en el archivo maestro.

 b) Generar un archivo de texto llamado “stock_minimo.txt” que contenga aquellos productos cuyo stock actual se
 encuentre por debajo del stock mínimo permitido.

3.​ A partir de información sobre la alfabetización en la Argentina, se desea actualizar un archivo maestro
    que contiene los siguientes datos: nombre de la provincia, cantidad de personas alfabetizadas y total de
    encuestados.

    Para ello, se dispone de dos archivos detalle, provenientes de distintas agencias de censo. Cada uno de
    estos archivos contiene: nombre de la provincia, código de localidad, cantidad de personas alfabetizadas
    y cantidad de encuestados.

    Se solicita desarrollar los módulos necesarios para actualizar el archivo maestro a partir de la
    información contenida en ambos archivos detalle.

    Nota: Todos los archivos están ordenados por nombre de provincia. En los archivos detalle pueden
    existir cero, uno o más registros por cada provincia.

4.​ Se cuenta con un archivo maestro de productos de una cadena de venta de alimentos congelados. De
    cada producto se almacena la siguiente información: código de producto, nombre, descripción, stock
    disponible, stock mínimo y precio.

    Diariamente se recibe un archivo detalle por cada una de las 30 sucursales de la cadena. Cada archivo
    detalle contiene: código de producto y cantidad vendida.

    Se solicita desarrollar un procedimiento que reciba los 30 archivos detalle y actualice el stock del archivo
    maestro.

    Además, deberá generarse un archivo de texto que informe, para aquellos productos cuyo stock
    disponible se encuentre por debajo del stock mínimo, los siguientes datos: nombre del producto,
    descripción, stock disponible y precio.

    Analizar alternativas para la generación de dicho informe: realizarlo en el mismo procedimiento de
    actualización o en un procedimiento separado, indicando las ventajas y desventajas de cada opción.

    Nota: Todos los archivos se encuentran ordenados por código de producto. En cada archivo detalle
    puede haber cero, uno o más registros para un mismo producto.




                                                                                                              2
5.​ Suponga que trabaja en una oficina donde se encuentra instalada una red local (LAN). La misma está
    conformada por 5 máquinas conectadas entre sí y a un servidor central.

 Semanalmente, cada máquina genera un archivo detalle de logs que registra las sesiones abiertas por los
 usuarios en cada terminal, junto con su duración. Cada archivo contiene los siguientes campos: código de
 usuario, fecha y tiempo de sesión.

 Se solicita desarrollar un procedimiento que reciba los archivos detalle y genere un archivo maestro con la
 siguiente información: código de usuario, fecha y tiempo total de sesiones abiertas.

 Notas:

    ●​ Cada archivo detalle está ordenado por código de usuario y fecha.
    ●​ Un usuario puede iniciar más de una sesión el mismo día, ya sea en la misma máquina o en
       diferentes máquinas.
    ●​ El archivo maestro debe crearse en la siguiente ubicación física: /var/log.

6.​ Se desea modelar la información necesaria para un sistema de recuento de casos de COVID del
    Ministerio de Salud de la Provincia de Buenos Aires.

 Diariamente se reciben 10 archivos detalle provenientes de distintos municipios. La información contenida
 en cada uno de ellos es la siguiente: código de localidad, código de cepa, cantidad de casos activos,
 cantidad de casos nuevos, cantidad de casos recuperados y cantidad de casos fallecidos.

 El ministerio cuenta con un archivo maestro que almacena la siguiente información: código de localidad,
 nombre de la localidad, código de cepa, nombre de la cepa, cantidad de casos activos, cantidad de casos
 nuevos, cantidad de casos recuperados y cantidad de casos fallecidos.

 Todos los archivos están ordenados por código de localidad y código de cepa.

 Se solicita desarrollar el procedimiento que permita actualizar el archivo maestro a partir de los 10 archivos
 detalle, teniendo en cuenta el siguiente criterio:

    ●​    A la cantidad de casos fallecidos del maestro se le debe sumar el valor recibido en el detalle.
    ●​    A la cantidad de casos recuperados del maestro se le debe sumar el valor recibido en el detalle.
    ●​    La cantidad de casos activos del maestro debe actualizarse con el valor recibido en el detalle.
    ●​    La cantidad de casos nuevos del maestro debe actualizarse con el valor recibido en el detalle.

 Realizar las declaraciones necesarias, el programa principal y los procedimientos que se requieran para
 efectuar la actualización solicitada.

                                                                                                               3
 Además, informar la cantidad de localidades que poseen más de 50 casos activos, independientemente de
 que hayan sido actualizadas o no.

7.​ Se dispone de un archivo maestro con información de los alumnos de la Facultad de Informática. Cada
    registro del archivo maestro contiene: código de alumno, apellido, nombre, cantidad de cursadas
    aprobadas y cantidad de materias con final aprobado. El archivo maestro está ordenado por código de
    alumno.

 Además, se dispone de dos archivos detalle con información sobre el desempeño académico de los
 alumnos: un archivo de cursadas y un archivo de exámenes finales.

 El archivo de cursadas contiene información sobre las materias cursadas por los alumnos. Cada registro
 incluye: código de alumno, código de materia, año de cursada y resultado (solo interesa determinar si la
 cursada fue aprobada o desaprobada).

 Por su parte, el archivo de exámenes finales contiene información sobre los exámenes rendidos. Cada
 registro incluye: código de alumno, código de materia, fecha del examen y nota obtenida.

 Ambos archivos detalle están ordenados por código de alumno y código de materia, y pueden contener
 cero, uno o más registros por alumno.

 Un alumno puede cursar una misma materia varias veces, así como también rendir el examen final en
 múltiples ocasiones.

 Se solicita desarrollar un programa que actualice el archivo maestro, modificando la cantidad de cursadas
 aprobadas y la cantidad de materias con final aprobado, a partir de la información contenida en los archivos
 detalle.

 Las reglas de actualización son las siguientes:

    ●​ Si un alumno aprueba una cursada, se incrementa en uno la cantidad de cursadas aprobadas.
    ●​ Si un alumno aprueba un examen final (nota mayor o igual a 4), se incrementa en uno la cantidad de
       materias con final aprobado.

 Notas:

    ●​ Los archivos deben procesarse en un único recorrido.
    ●​ No es necesario verificar inconsistencias en la información de los archivos detalle. En particular, se
       garantiza que un alumno no puede aprobar más de una vez la cursada de una misma materia. De
       manera análoga, tampoco puede aprobar más de una vez el examen final de una misma materia.

8.​ Se desea gestionar la información correspondiente al consumo de yerba mate en las distintas provincias
    de la Argentina.

                                                                                                                4
 Para ello, se dispone de un archivo maestro que contiene la siguiente información: código de provincia,
 nombre de la provincia, cantidad de habitantes y cantidad total histórica de kilos de yerba consumidos.

 Mensualmente, se reciben 16 archivos detalle con información relevada sobre el consumo de yerba mate
 en distintos puntos del país. Cada archivo detalle contiene: código de provincia y cantidad de kilos de yerba
 consumidos en ese relevamiento.

 Un archivo detalle puede contener información correspondiente a una o varias provincias, y una misma
 provincia puede aparecer cero, uno o más veces en los distintos archivos detalle.

 Tanto el archivo maestro como los archivos detalle están ordenados por código de provincia.

 Se solicita desarrollar un programa que actualice el archivo maestro a partir de la nueva información de
 consumo.

 Además, se debe informar por pantalla aquellas provincias (código y nombre) cuya cantidad total histórica
 de yerba consumida supere los 10.000 kilos, indicando también el promedio de consumo por habitante.

 Para este informe deben considerarse tanto las provincias actualizadas como aquellas que no hayan
 recibido modificaciones.

 Nota: Cada archivo debe recorrerse una única vez.

9.​ Se cuenta con un archivo que posee información de las ventas que realiza una empresa a los diferentes
    clientes. Se necesita obtener un reporte con las ventas organizadas por cliente. Para ello, se deberá
    informar por pantalla: los datos personales del cliente, el total mensual (mes por mes cuánto compró) y
    finalmente el monto total comprado en el año por el cliente. Además, al finalizar el reporte, se debe
    informar el monto total de ventas obtenido por la empresa.

    El formato del archivo maestro está dado por: cliente (cod cliente, nombre y apellido), año, mes, día y
    monto de la venta. El orden del archivo está dado por: cod cliente, año y mes.

    Nota: tenga en cuenta que puede haber meses en los que los clientes no realizaron compras. No es
    necesario que informe tales meses en el reporte.

10.​ Se necesita contabilizar los votos de las diferentes mesas electorales registradas por provincia y
     localidad. Para ello, se posee un archivo con la siguiente información: código de provincia, código de
     localidad, número de mesa y cantidad de votos en dicha mesa. Presentar en pantalla un listado como se
     muestra a continuación:

           Código de Provincia

           Código de Localidad                Total de Votos

           ................................   ......................

           ................................   ......................


                                                                                                            5
            Total de Votos Provincia: ____

            Código de Provincia

            Código de Localidad                           Total de Votos

            ................................               ......................

            Total de Votos Provincia: ___

            …………………………………………………………..

            Total General de Votos: ___

        NOTA: La información está ordenada por código de provincia y código de localidad.

11.​ Se tiene información en un archivo de las horas extras realizadas por los empleados de una empresa en
     un mes. Para cada empleado se tiene la siguiente información: departamento, división, número de
     empleado, categoría y cantidad de horas extras realizadas por el empleado. Se sabe que el archivo se
     encuentra ordenado por departamento, luego por división y, por último, por número de empleado.
     Presentar en pantalla un listado con el siguiente formato:

        Departamento

        División

    ​    Número de Empleado                    Total de Hs. Importe a cobrar

               ......                              ..........             .........

               ......                              ..........             .........

            Total de horas división: ____

            Monto total por división: ____



         División

            .................

         Total horas departamento: ____

         Monto total departamento: ____

        Para obtener el valor de la hora se debe cargar un arreglo desde un archivo de texto al iniciar el
        programa con el valor de la hora extra para cada categoría. La categoría varía de 1 a 15. En el
        archivo de texto debe haber una línea para cada categoría con el número de categoría y el valor de la
        hora, pero el arreglo debe ser de valores de horas, con la posición del valor coincidente con el
        número de categoría.




                                                                                                           6
12.​ La empresa de software ‘X’ posee un servidor web donde se encuentra alojado el sitio web de la
     organización. En dicho servidor, se almacenan en un archivo todos los accesos que se realizan al sitio.
     La información que se almacena en el archivo es la siguiente: año, mes, día, idUsuario y tiempo de
     acceso al sitio de la organización. El archivo se encuentra ordenado por los siguientes criterios: año,
     mes, día e idUsuario.

    Se debe realizar un procedimiento que genere un informe en pantalla, para ello se indicará el año
    calendario sobre el cual debe realizar el informe. El mismo debe respetar el formato mostrado a
    continuación:


    Año : ---
        Mes:-- 1
             día:-- 1
                  ​idUsuario 1 Tiempo Total de acceso en el dia 1 mes 1
                  ​--------       ​    ​
    ​         ​ idUsuario N Tiempo total de acceso en el dia 1 mes 1
             Tiempo total acceso dia 1 mes 1​ ​
    ​    -------------
    ​    día N
    ​    ​         idUsuario 1 Tiempo Total de acceso en el dia N mes 1
                  ​--------       ​    ​
    ​         ​ idUsuario N Tiempo total de acceso en el dia N mes 1
             Tiempo total acceso dia N mes 1
         Total tiempo de acceso mes 1
         ------​
         Mes 12
    ​    día 1
                  ​idUsuario 1 Tiempo Total de acceso en el dia 1 mes 12
                  ​--------       ​    ​
    ​         ​ idUsuario N Tiempo total de acceso en el dia 1 mes 12
             Tiempo total acceso dia 1 mes 12​​
    ​    -------------
    ​    día N
    ​    ​         idUsuario 1 Tiempo Total de acceso en el dia N mes 12           ​
                       --------     ​ ​
    ​         ​ idUsuario N Tiempo total de acceso en el dia N mes 12
             Tiempo total acceso dia N mes 12
         Total tiempo de acceso mes 12
     Total tiempo de acceso año

    Se deberá tener en cuenta las siguientes aclaraciones:
       ●​ El año sobre el cual realizará el informe de accesos debe leerse desde el teclado.
       ●​ El año puede no existir en el archivo, en tal caso, debe informarse en pantalla “año no
          encontrado”.
       ●​ Debe definir las estructuras de datos necesarias.
       ●​ El recorrido del archivo debe realizarse una única vez procesando sólo la información necesaria.



                                                                                                          7
13.​ Suponga que usted es administrador de un servidor de correo electrónico. En los logs del mismo
     (información guardada acerca de los movimientos que ocurren en el server) que se encuentra en la
     siguiente ruta: /var/log/logmail.dat se guarda la siguiente información: nro_usuario, nombreUsuario,
     nombre, apellido, cantidadMailEnviados. Diariamente el servidor de correo genera un archivo con la
     siguiente información: nro_usuario, cuentaDestino, cuerpoMensaje. Este archivo representa todos los
     correos enviados por los usuarios en un día determinado. Ambos archivos están ordenados por
     nro_usuario y se sabe que un usuario puede enviar cero, uno o más mails por día.
             a.​ Realice el procedimiento necesario para actualizar la información del log en un día particular.
                 Defina las estructuras de datos que utilice su procedimiento.
             b.​ Genere un archivo de texto que contenga el siguiente informe dado un archivo detalle de un
                 día determinado:

                   nro_usuarioX…………..cantidadMensajesEnviados

                   ………….​

                   nro_usuarioX+n………..cantidadMensajesEnviados

                Nota: tener en cuenta que en el listado deberán aparecer todos los usuarios que existen en
                el sistema. Considere la implementación de esta opción de las siguientes maneras:

                        i- Como un procedimiento separado del punto a).

                       ii- En el mismo procedimiento de actualización del punto a). Qué cambios se
                       requieren en el procedimiento del punto a) para realizar el informe en el mismo
                       recorrido?

14.​ Una compañía aérea dispone de un archivo maestro donde guarda información sobre sus próximos
     vuelos. En dicho archivo se tiene almacenado el destino, fecha, hora de salida y la cantidad de asientos
     disponibles. La empresa recibe todos los días dos archivos detalles para actualizar el archivo maestro.
     En dichos archivos se tiene destino, fecha, hora de salida y cantidad de asientos comprados. Se sabe
     que los archivos están ordenados por destino más fecha y hora de salida, y que en los detalles pueden
     venir 0, 1 ó más registros por cada uno del maestro. Se pide realizar los módulos necesarios para:
          a.​ Actualizar el archivo maestro sabiendo que no se registró ninguna venta de pasaje sin asiento
              disponible.

          b.​ Generar una lista con aquellos vuelos (destino y fecha y hora de salida) que tengan menos de
              una cantidad específica de asientos disponibles. La misma debe ser ingresada por teclado.

       NOTA: El archivo maestro y los archivos detalles sólo pueden recorrerse una vez.

15.​ Se desea modelar la información de una ONG dedicada a la asistencia de personas con carencias
     habitacionales. La ONG cuenta con un archivo maestro conteniendo información como se indica a
     continuación: Código pcia, nombre provincia, código de localidad, nombre de localidad, #viviendas sin
     luz, #viviendas sin gas, #viviendas de chapa, #viviendas sin agua, # viviendas sin sanitarios.
                                                                                                              8
    Mensualmente reciben detalles de las diferentes provincias indicando avances en las obras de ayuda en
    la edificación y equipamientos de viviendas en cada provincia. La información de los detalles es la
    siguiente: Código pcia, código localidad, #viviendas con luz, #viviendas construidas, #viviendas con
    agua, #viviendas con gas, #entrega sanitarios.

    Se debe realizar el procedimiento que permita actualizar el maestro con los detalles recibidos, se reciben
    10 detalles. Todos los archivos están ordenados por código de provincia y código de localidad.

    Para la actualización del archivo maestro, se debe proceder de la siguiente manera:

         ●​ Al valor de viviendas sin luz se le resta el valor recibido en el detalle.
         ●​ Idem para viviendas sin agua, sin gas y sin sanitarios.
         ●​ A las viviendas de chapa se le resta el valor recibido de viviendas construidas

    La misma combinación de provincia y localidad aparecen a lo sumo una única vez.

    Realice las declaraciones necesarias, el programa principal y los procedimientos que requiera para la
    actualización solicitada e informe cantidad de localidades sin viviendas de chapa (las localidades pueden
    o no haber sido actualizadas).

16.​ La editorial X, autora de diversos semanarios, posee un archivo maestro con la información
     correspondiente a las diferentes emisiones de los mismos. De cada emisión se registra: fecha, código
     de semanario, nombre del semanario, descripción, precio, total de ejemplares y total de ejemplares
     vendidos.

    Mensualmente se reciben 100 archivos detalles con las ventas de los semanarios en todo el país. La
    información que poseen los detalles es la siguiente: fecha, código de semanario y cantidad de
    ejemplares vendidos. Realice las declaraciones necesarias, la llamada al procedimiento y el
    procedimiento que recibe el archivo maestro y los 100 detalles y realice la actualización del archivo
    maestro en función de las ventas registradas. Además deberá informar fecha y semanario que tuvo más
    ventas y la misma información del semanario con menos ventas.

    Nota: Todos los archivos están ordenados por fecha y código de semanario. No se realizan ventas de
    semanarios si no hay ejemplares para hacerlo

17.​ Una concesionaria de motos de la Ciudad de Chascomús, posee un archivo con información de las
     motos que posee a la venta. De cada moto se registra: código, nombre, descripción, modelo, marca y
     stock actual. Mensualmente se reciben 10 archivos detalles con información de las ventas de cada uno
     de los 10 empleados que trabajan. De cada archivo detalle se dispone de la siguiente información:
     código de moto, precio y fecha de la venta. Se debe realizar un proceso que actualice el stock del
     archivo maestro desde los archivos detalles. Además se debe informar cuál fue la moto más vendida.

    NOTA: Todos los archivos están ordenados por código de la moto y el archivo maestro debe ser recorrido
    sólo una vez y en forma simultánea con los detalles.

18.​ Se cuenta con un archivo con información de los casos de COVID-19 registrados en los diferentes hospitales
     de la Provincia de Buenos Aires cada día. Dicho archivo contiene: código de localidad, nombre de localidad,
                                                                                                               9
    código de municipio, nombre de municipio, código de hospital, nombre de hospital, fecha y cantidad de casos
    positivos detectados. El archivo está ordenado por localidad, luego por municipio y luego por hospital.

    Escriba la definición de las estructuras de datos necesarias y un procedimiento que haga un listado con el
    siguiente formato:


    Nombre: Localidad 1
    ​     Nombre: Municipio 1
    ​     ​          Nombre Hospital 1……………..Cantidad de casos Hospital 1
    ​     ​          ……………………..
    ​     ​          Nombre Hospital N…………….Cantidad de casos Hospital N
    ​     Cantidad de casos Municipio 1
    ​     …………………………………………………………………….
              Nombre Municipio N
    ​     ​          Nombre Hospital 1……………..Cantidad de casos Hospital 1
    ​     ​          ……………………..
    ​     ​          Nombre Hospital N…………….Cantidad de casos Hospital N
    ​     Cantidad de casos Municipio N
    Cantidad de casos Localidad 1
    -----------------------------------------------------------------------------------------
    Nombre Localidad N
    Nombre Municipio 1
    ​     ​          Nombre Hospital 1……………..Cantidad de casos Hospital 1
    ​     ​          ……………………..
    ​     ​          Nombre Hospital N…………….Cantidad de casos Hospital N
    ​     Cantidad de casos Municipio 1
    ​     …………………………………………………………………….
              Nombre Municipio N
    ​     ​          Nombre Hospital 1……………..Cantidad de casos Hospital 1
    ​     ​          ……………………..
    ​     ​          Nombre Hospital N…………….Cantidad de casos Hospital N
    ​     Cantidad de casos Municipio N
    Cantidad de casos Localidad N

    Cantidad de casos Totales en la Provincia
    ​
    Además del informe en pantalla anterior, es necesario exportar a un archivo de texto la siguiente información:
    nombre de localidad, nombre de municipio y cantidad de casos del municipio, para aquellos municipios cuya
    cantidad de casos supere los 1500. El formato del archivo de texto deberá ser el adecuado para recuperar la
    información con la menor cantidad de lecturas posibles.

    NOTA: El archivo debe recorrerse solo una vez.

19.​ A partir de un siniestro ocurrido se perdieron las actas de nacimiento y fallecimientos de toda la
     provincia de buenos aires de los últimos diez años. En pos de recuperar dicha información, se deberá
     procesar 2 archivos por cada una de las 50 delegaciones distribuidas en la provincia, un archivo de
     nacimientos y otro de fallecimientos y crear el archivo maestro reuniendo dicha información.

                                                                                                                10
  Los archivos detalles con nacimientos, contendrán la siguiente información: nro partida nacimiento,
  nombre, apellido, dirección detallada (calle, nro, piso, depto, ciudad), matrícula del médico, nombre y
  apellido de la madre, DNI madre, nombre y apellido del padre, DNI del padre.
  En cambio, los 50 archivos de fallecimientos tendrán: nro partida nacimiento, DNI, nombre y apellido del
  fallecido, matrícula del médico que firma el deceso, fecha y hora del deceso y lugar.
  Realizar un programa que cree el archivo maestro a partir de toda la información de los archivos
  detalles. Se debe almacenar en el maestro: nro partida nacimiento, nombre, apellido, dirección detallada
  (calle, nro, piso, depto, ciudad), matrícula del médico, nombre y apellido de la madre, DNI madre,
  nombre y apellido del padre, DNI del padre y si falleció, además matrícula del médico que firma el
  deceso, fecha y hora del deceso y lugar. Se deberá, además, listar en un archivo de texto la información
  recolectada de cada persona.

  Nota: Todos los archivos están ordenados por nro partida de nacimiento que es única. Tenga en cuenta
  que no necesariamente va a fallecer en el distrito donde nació la persona y además puede no haber
  fallecido.

IMPORTANTE: Se recomienda implementar los ejercicios prácticos en Dev-Pascal. El ejecutable
puede descargarse desde la plataforma moodle.

                                                                                                       11
