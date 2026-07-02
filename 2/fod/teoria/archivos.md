# Archivos

## Tipos de archivos

En la materia se trabajan tres tipos de archivos:

* **Registros de longitud fija o tipados (`file of <tipo_dato>`):** archivos binarios. Se declaran como `file of` un tipo de dato, que puede ser un tipo primitivo de Pascal o un registro definido por el usuario. Un archivo declarado como `file of string` **no es un archivo de texto**: sigue siendo binario, solo que sus elementos son strings. Este es un error común a evitar cuando se pide específicamente un archivo de texto.
* **Texto (Text):** caracteres estructurados en líneas. Lectura/escritura con conversión automática de tipos. El acceso es exclusivamente secuencial. Útiles para importar y exportar datos.
* **Bloques de bytes (File):** se ven en la parte teórica de la materia, no se trabajan en la práctica.

### Formas de acceso a la información

* **Secuencial:** para acceder a un elemento es necesario haber accedido antes a todos los anteriores (en particular, al inmediato anterior). Es la forma de acceso de los archivos de texto.
* **Directo:** se accede al elemento a través de una dirección determinada, logrando llegar a la información en un solo acceso.
* **Secuencial indizado:** se utiliza una estructura auxiliar (por ejemplo, un árbol) que indica en qué posición del archivo está un elemento. A partir de esa estructura se obtiene el número relativo de registro y se accede al archivo. Este tipo de acceso se profundiza al ver árboles y hash.

## Definición de archivos tipados

Existen dos formas de definir un archivo:

* **Forma 1 (en `var`):**
  ```pascal
  var archivo_logico: file of tipo_de_dato;
  ```
* **Forma 2 (en `type`):**
  ```pascal
  type archivo = file of tipo_de_datos;
  var archivo_logico: archivo;
  ```

Conceptualmente lo correcto es definirlo en `type`, ya que se trata de un nuevo tipo de dato, aunque definirlo directamente en `var` también es válido.

### Ejemplo de definición

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

Los tres archivos del ejemplo son binarios: uno de enteros, uno de strings y uno de registros `persona`.

## Operaciones básicas sobre archivos

### `assign`

```pascal
assign(nombre_logico, nombre_fisico);
```

Establece la correspondencia entre el archivo lógico (la variable que se usa en el programa) y el archivo físico (la ubicación real en disco, red, etc.). Es el primer paso obligatorio: sin el `assign`, el programa no sabe de dónde leer o dónde escribir.

#### Ejemplo

```pascal
assign(enteros, 'c:\archivos\enteros.dat');
assign(texto, 'c:\archivos\texto.dat');
assign(personas, 'c:\archivos\personas.dat');
```

### Apertura y creación de archivos

* `rewrite(nombre_logico);` → **Crea** un archivo nuevo. Si el archivo ya existía y tenía contenido, `rewrite` lo borra por completo y deja el archivo vacío. Hay que tener cuidado de no usarlo sobre un archivo cuyo contenido se quiere conservar.
* `reset(nombre_logico);` → **Abre** un archivo existente, sin alterar su contenido.

#### Ejemplo

```pascal
rewrite(enteros);
reset(personas);
```

### Cierre de archivos

```pascal
close(nombre_logico);
```

Transfiere la información del buffer al disco. Si se omite el `close`, es probable que las últimas modificaciones realizadas no se guarden efectivamente en el disco. Conviene tomarlo como una regla fija: siempre cerrar los archivos con los que se trabajó.

#### Ejemplo

```pascal
close(enteros);
close(personas);
```

### Lectura y escritura

* `read(nombre_logico, var_dato);` — el tipo de `var_dato` debe coincidir exactamente con el tipo de los elementos del archivo, o se producirá un error de tipos y el programa no funcionará.
* `write(nombre_logico, var_dato);`

#### Ejemplo

```pascal
read(enteros, e);   { e: integer }
write(personas, p); { p: persona }
```

### Operaciones adicionales

* `EOF(nombre_logico);` — controla el fin de archivo. Es necesario verificarlo antes de cada lectura: si se intenta leer sobre la marca de fin de archivo, se produce un error (por ejemplo, al leer un archivo de enteros, el `EOF` no es un valor entero válido).
* `fileSize(nombre_logico);` — devuelve el tamaño del archivo.
* `filePos(nombre_logico);` — devuelve la posición actual del puntero en el archivo. En archivos de longitud fija, los registros se numeran de `0..N-1`.
* `seek(nombre_logico, pos);` — posiciona el puntero en una ubicación determinada. Se usa típicamente cuando, tras una lectura, el puntero avanzó una posición y es necesario retroceder uno para modificar el registro correcto (en vez del siguiente).

## Ejemplo completo: creación de un archivo de personas

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

begin
  write('Ingrese el nombre del archivo: ');
  readln(nombre_fisico);
  { enlace entre el nombre lógico y el nombre físico }
  assign(personas, nombre_fisico);
  { apertura del archivo para creación }
  rewrite(personas);

  { lectura del DNI de una persona }
  write('Ingrese el dni de la persona: ');
  readln(per.dni);
  while (per.dni <> '') do begin
    { lectura del resto de los datos de la persona }
    write('Ingrese el apellido y nombre de la persona: ');
    readln(per.apellidoyNombre);
    write('Ingrese la dirección de la persona: ');
    readln(per.direccion);
    write('Ingrese el sexo de la persona: ');
    readln(per.sexo);
    write('Ingrese el salario de la persona: ');
    readln(per.salario);
    { escritura del registro de la persona en el archivo }
    write(personas, per);
    { lectura del DNI de una nueva persona }
    write('Ingrese otro dni o blanco para terminar: ');
    readln(per.dni);
  end;
  { cierre del archivo }
  close(personas);
end.
```

El ciclo carga personas mientras el usuario no ingrese un DNI en blanco. Al terminar, es fundamental hacer el `close`: si se omite, las últimas personas cargadas no se bajan al disco.

## Conversión entre archivo de texto y archivo binario

Un archivo de texto se declara con el tipo `text`, que ya existe en Pascal (no se declara como `file of`).

```pascal
type
  votos = record
    codProvincia: integer;
    codLocalidad: integer;
    mesa: integer;
    cantidadVotos: integer;
    descripcion: string;
  end;
  archivo_votos = file of votos;

var
  arch: archivo_votos;
  carga: text;
  v: votos;
```

### Diferencia visual entre texto y binario

Un archivo de texto, abierto con un editor, muestra la información en caracteres legibles organizados en líneas. Un archivo binario, en cambio, muestra símbolos no interpretables a simple vista: para leerlo o escribirlo se necesita un programa que conozca la estructura con la que fue almacenada la información (qué registros, qué tipos de datos, etc.).

### De texto a binario

```pascal
assign(carga, 'votos.txt');
assign(arch, 'votos.dat');
reset(carga);
rewrite(arch);

while not EOF(carga) do begin
  readln(carga, v.codProvincia, v.codLocalidad, v.mesa, v.cantidadVotos, v.descripcion);
  write(arch, v);
end;

close(carga);
close(arch);
```

Mientras no se llegue al fin del archivo de texto, se lee una línea completa con `readln` (separando los campos en la misma instrucción, sin tener que convertir tipo por tipo) y se escribe el registro correspondiente en el archivo binario. Al terminar, se cierran ambos archivos.

Es importante notar que en la lectura y escritura de archivos de texto la conversión de tipos es automática: no es necesario convertir manualmente cada campo (entero, real, etc.) al leerlo o escribirlo.

### De binario a texto

```pascal
assign(arch, 'votos.dat');
assign(carga, 'votos.txt');
reset(arch);
rewrite(carga);

while not EOF(arch) do begin
  read(arch, v);
  writeln(carga, v.codProvincia, ' ', v.codLocalidad, ' ', v.mesa, ' ',
                 v.cantidadVotos, ' ', v.descripcion);
end;

close(carga);
close(arch);
```

Mientras no se llegue al fin del archivo binario, se lee un registro y se escribe una línea en el archivo de texto, separando manualmente cada campo con un espacio en blanco.

### Cuidado con los strings en archivos de texto

En un archivo de texto, el carácter que separa los valores al leer es el espacio en blanco. Esto tiene una consecuencia importante: si una línea contiene varios campos de tipo string, solo puede haber **un campo string por línea, y debe ubicarse al final de la línea**. Si un string no está al final, el espacio en blanco que lo sigue (pensado como separador del siguiente campo) será interpretado como parte del propio string, y la lectura del resto de los campos quedará desalineada. Por eso, en el ejemplo de votos, el campo `descripcion` (string) se coloca al final del registro.
