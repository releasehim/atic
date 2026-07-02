# 1 - Archivos

## Diapositiva 1
Fundamentos de Organización de Datos
1
Archivos

## Diapositiva 2
### Tipos de Archivos
* **Texto (Text):** Caracteres estructurados en líneas. Lectura/escritura con conversión automática de tipos. El acceso es exclusivamente secuencial. Útiles para importar y exportar datos.
* **Registros de longitud fija o tipados (File of <tipo_dato>)**
* **Bloques de bytes (File)**

## Diapositiva 3
### Operaciones básicas
#### Definición de Archivos tipados
Dos formas:
* **Forma 1:**
  ```pascal
  var archivo_logico: file of tipo_de_dato;
  ```
* **Forma 2:**
  ```pascal
  type archivo = file of tipo_de_datos;
  var archivo_logico: archivo;
  ```

## Diapositiva 4
### Ejemplo Definición
```pascal
type
  persona = record
    dni: string[8];
    apellido: string[25];
    nombre: string[25];
    direccion: string[25];
    sexo: char;
  end;
  
  archivo_enteros = file of integer;
  archivo_string = file of string;
  archivo_personas = file of persona;

var
  enteros: archivo_enteros;
  texto: archivo_string;
  personas: archivo_personas;
```

## Diapositiva 5
### Operaciones: assign
```pascal
assign(nombre_logico, nombre_fisico);
```
Realiza una correspondencia entre el archivo lógico y el archivo físico.

#### Ejemplo:
```pascal
assign(enteros, 'c:\archivos\enteros.dat');
assign(texto, 'c:\archivos\texto.dat');
assign(personas, 'c:\archivos\personas.dat');
```

## Diapositiva 6
### Operaciones: Apertura y creación de archivos
* `rewrite(nombre_logico);` -> Crea un archivo nuevo.
* `reset(nombre_logico);` -> Abre un archivo existente.

#### Ejemplo:
```pascal
rewrite(enteros);
reset(personas);
```

## Diapositiva 7
### Operaciones: Cierre de archivos
```pascal
close(nombre_logico);
```
Transfiere la información del buffer al disco.

#### Ejemplo:
```pascal
close(enteros);
close(personas);
```

## Diapositiva 8
### Operaciones: Lectura y escritura de archivos
* `read(nombre_logico, var_dato);`
  El tipo de dato de la variable `var_dato` es igual al tipo de datos de los elementos del archivo.
* `write(nombre_logico, var_dato);`

#### Ejemplo:
```pascal
read(enteros, e);  { e: integer }
write(personas, p); { p: persona }
```

## Diapositiva 9
### Operaciones adicionales
* `EOF(nombre_logico);`
  Controla el fin de archivo.
* `fileSize(nombre_logico);`
  Devuelve el tamaño de un archivo.
* `filePos(nombre_logico);`
  Devuelve la posición actual del puntero en el archivo. En longitud fija, los registros se numeran de `0..N-1`.
* `seek(nombre_logico, pos);`
  Establece la posición del puntero en el archivo.

## Diapositiva 10
### Ejemplo: Creación de archivo (Parte 1)
```pascal
program creacion_archivo;
type
  persona = record
    dni: string[8];
    apellidoyNombre: string[30];
    direccion: string[40];
    sexo : char;
    salario : real;
  end;
  archivo_personas = file of persona;

var
  personas: archivo_personas;
  nombre_fisico: string[12];
  per: persona;
```

## Diapositiva 11
### Ejemplo: Creación de archivo (Parte 2)
```pascal
begin
  write('Ingrese el nombre del archivo: ');
  readln(nombre_fisico);
  {enlace entre el nombre lógico y el nombre físico}
  assign(personas, nombre_fisico);
  {apertura del archivo para creación}
  rewrite(personas);
```

## Diapositiva 12
### Ejemplo: Creación de archivo (Parte 3)
```pascal
  {lectura del DNI una persona}
  write('Ingrese el dni de la persona: ');
  readln(per.dni);
  while (per.dni <> '') do begin
    {lectura del resto de los datos de la persona}
    write('Ingrese el apellido y nombre de la persona: ');
    readln(per.apellidoyNombre);
    write('Ingrese la dirección de la persona: ');
    readln(per.direccion);
    write('Ingrese el sexo de la persona: ');
    readln(per.sexo);
    write('Ingrese el salario de la persona: ');
    readln(per.salario);
    {escritura del registro de la persona en el archivo}
    write(personas, per);
    {lectura del DNI de una nueva persona}
    write('Ingrese otro dni o blanco para terminar: ');
    readln(per.dni);
  end;
  {cierre del archivo}
  close(personas);
end.
```

## Diapositiva 13
¿Dudas?
