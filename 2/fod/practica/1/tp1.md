# Fundamentos de Organización de Datos - Práctica 1

Creación, consulta y mantenimiento de archivos secuenciales - Algorítmica Básica.

## Objetivos:
* Comprender operaciones básicas sobre archivos binarios.
* Crear, recorrer y actualizar archivos binarios combinando sus operaciones básicas.
* Generar y utilizar archivos de texto para la carga y exportación de información.

1. Realizar un algoritmo que cree un archivo binario de números enteros no ordenados y permita incorporar datos al archivo. Los números son ingresados desde el teclado. La carga finaliza cuando se ingresa el número 30000, que no debe incorporarse al archivo. El nombre del archivo debe ser proporcionado por el usuario desde el teclado.

2. Realizar un algoritmo, que utilizando el archivo de números enteros no ordenados creado en el ejercicio 1, informe por pantalla cantidad de números menores a 15000 y el promedio de los números ingresados. El nombre del archivo a procesar debe ser proporcionado por el usuario una única vez. Además, el algoritmo deberá listar el contenido del archivo en pantalla. Resolver el ejercicio realizando un único recorrido del archivo.

3. Realizar un programa que presente un menú con opciones para:
   a. Crear un archivo binario de registros no ordenados de empleados y completarlo con datos ingresados desde teclado. De cada empleado se registra: número de empleado, apellido, nombre, edad y DNI. Algunos empleados pueden ingresan el DNI con valor 0, lo que significa que al momento de la carga puede no tenerlo. La carga finaliza cuando se ingresa el String 'fin' como apellido.
   b. Abrir el archivo anteriormente generado y
      i. Listar en pantalla los datos de empleados que tengan un nombre o apellido determinado, el cual se proporciona desde el teclado.
      ii. Listar en pantalla los empleados de a uno por línea.
      iii. Listar en pantalla los empleados mayores de 70 años, próximos a jubilarse.
   NOTA: El nombre del archivo a crear o utilizar debe ser proporcionado por el usuario.

4. Agregar al menú del programa del ejercicio 3, opciones para:
   a. Añadir uno o más empleados al final del archivo con sus datos ingresados por teclado. Tener en cuenta que no se debe agregar al archivo un empleado con un número de empleado ya registrado (control de unicidad).
   b. Modificar la edad de un empleado dado.
   c. Exportar el contenido del archivo a un archivo de texto llamado "todos_empleados.txt".
   d. Exportar a un archivo de texto llamado "faltaDNIEmpleado.txt", los empleados que no tengan cargado el DNI (DNI en 0).
   NOTA: Las búsquedas deben realizarse por número de empleado.

5. Realizar un programa para una tienda de celulares, que presente un menú con opciones para:
   a. Crear un archivo de registros no ordenados de celulares y cargarlo con datos ingresados desde un archivo de texto denominado "celulares.txt". Los registros correspondientes a los celulares deben contener: código de celular, nombre, descripción, marca, precio, stock mínimo y stock disponible. El formato del archivo de texto de carga se especifica en la NOTA 2 ubicada al final del ejercicio.
   b. Listar en pantalla los datos de aquellos celulares que tengan un stock menor al stock mínimo.
   c. Listar en pantalla los celulares del archivo cuya descripción contenga una cadena de caracteres proporcionada por el usuario.
   d. Exportar el archivo binario creado en el inciso a) a un archivo de texto denominado "celulares.txt" con todos los celulares del mismo. El archivo de texto generado podría ser utilizado en un futuro como archivo de carga (ver inciso a), por lo que debería respetar el formato dado para este tipo de archivos en la NOTA 2.
   NOTA 1: El nombre del archivo binario de celulares debe ser proporcionado por el usuario.
   NOTA 2: El archivo de carga debe editarse de manera que cada celular se especifique en tres líneas consecutivas. En la primera se especifica: código de celular, el precio y marca, en la segunda el stock disponible, stock mínimo y la descripción y en la tercera nombre en ese orden. Cada celular se carga leyendo tres líneas del archivo "celulares.txt".
   Ejemplo de Archivo:
   101 250000 Samsung
   15 5 Galaxy A15 128GB
   Galaxy A15
   102 320000 Motorola
   3 6 Moto G84 256GB color azul
   Moto G84
   104 950000 Apple
   2 4 iPhone 15 256GB negro
   iPhone 15

6. Agregar al menú del programa del ejercicio 5, opciones para:
   a. Añadir uno o más celulares al final del archivo con sus datos ingresados por teclado.
   b. Modificar el stock de un celular dado.
   c. Exportar el contenido del archivo binario a un archivo de texto denominado: "SinStock.txt", con aquellos celulares que tengan stock 0.
   NOTA: Las búsquedas deben realizarse por nombre de celular.

7. Realizar un programa que permita:
   a. Crear un archivo binario a partir de la información almacenada en un archivo de texto. El nombre del archivo de texto es: "novelas.txt". La información en el archivo de texto consiste en: código de novela, nombre, género y precio de diferentes novelas argentinas. Los datos de cada novela se almacenan en dos líneas en el archivo de texto. La primera línea contendrá la siguiente información: código novela, precio y género, y la segunda línea almacenará el nombre de la novela.
   b. Abrir el archivo binario y permitir la actualización del mismo. Se debe poder agregar una novela y modificar una existente. Las búsquedas se realizan por código de novela.
   NOTA: El nombre del archivo binario es proporcionado por el usuario desde el teclado.

IMPORTANTE: Se recomienda implementar los ejercicios prácticos en Dev-Pascal. El ejecutable puede descargarse desde la plataforma Moodle.