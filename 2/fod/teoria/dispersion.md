# Fundamentos de Organización de Datos — Hashing (Dispersión de Archivos)

---

## ¿Qué es la dispersión de archivos?

- **Técnica**: permite generar una dirección base única para una clave dada.
- **Mapeo**: convierte la clave en un número (en principio, aleatorio), que luego sirve para determinar dónde se almacena la clave.
- **Función**: se utiliza una función de dispersión para mapear cada clave con una dirección física de almacenamiento.
- **Aplicación**: se usa cuando se requiere acceso rápido por clave.

## Tipos de dispersión

- **Direccionamiento estático**: el espacio disponible para dispersar los registros del archivo está fijado previamente.
- **Direccionamiento dinámico**: el espacio disponible para dispersar los registros del archivo aumenta o disminuye en función de las necesidades.

## Parámetros que influyen sobre el desempeño del ambiente de dispersión

- Capacidad de almacenamiento de cada dirección.
- Densidad de empaquetamiento.
- Función de hash.
- Método de tratamiento de desbordes.

## Conceptos clave

- **Función de dispersión**: una "caja negra" que, a partir de una clave, genera la dirección física donde debe almacenarse el registro.
- **Colisión**: situación en la que un registro es asignado, por la función de dispersión, a una dirección que ya posee uno o más registros.
- **Desborde**: situación en la cual una clave carece de lugar en la dirección asignada por la función de dispersión (la dirección ya está completa).
- **Densidad de empaquetamiento (DE)**: relación entre la cantidad de registros que integran el archivo y el espacio total disponible.

$$\text{DE} = \frac{\text{número de registros}}{\text{espacio Total}}$$

Aunque la función de dispersión sea eficiente y la densidad de empaquetamiento sea baja, es probable que igualmente ocurran desbordes — por eso hace falta un método para resolverlos.

---

## Resolución de colisiones con desborde en direccionamiento estático

Los métodos aplicables para resolver colisiones con desborde en dispersión estática son:

- Saturación progresiva.
- Saturación progresiva encadenada.
- Saturación progresiva encadenada con área de desborde por separado.
- Dispersión doble.

A continuación se desarrollan en detalle las dos últimas técnicas, que son una mejora sobre la saturación progresiva encadenada simple.

### Saturación progresiva encadenada con área de desborde separado

Esta técnica es una mejora de la saturación progresiva encadenada común. En la versión simple, las colisiones y los desbordes posteriores se resolvían **dentro de la misma área principal** de direccionamiento: en esa misma área convivían claves alojadas en su dirección base junto con claves que en realidad eran sinónimos desbordados de otra dirección.

La mejora consiste en garantizar que **todos los desbordes se resuelvan en un área separada**, distinta del área principal. Esto tiene una ventaja directa: al quedar más cantidad de claves efectivamente alojadas en sus propias direcciones base, los algoritmos de búsqueda son más eficientes, porque encuentran más rápido la mayoría de las claves (sin tener que recorrer una cadena de sinónimos).

**Mecánica de la técnica** (ejemplo trabajado en clase, con una memoria de 11 direcciones, 2 registros por dirección, y 15 claves a dispersar en total):

- Las primeras claves se van dispersando sin inconvenientes en el área principal. Puede haber **colisión sin desborde**: por ejemplo, una clave como el 56 puede colisionar con otra clave ya presente en su dirección base, pero si en esa dirección todavía queda lugar para un segundo registro, entra sin generar desborde.
- Cuando una clave (por ejemplo, el 12) llega a una dirección base que ya está completa (por ejemplo, la dirección 1), se produce **colisión y desborde**. La resolución es: se inserta el 12 en la primera cubeta libre del **área de desborde separada** (por ejemplo, la cubeta 0 de esa área), y se actualiza el **enlace** de la dirección base 1 para que apunte a esa cubeta del área separada — de esta manera, el algoritmo de búsqueda sabe que, si no encuentra la clave en la dirección base, debe continuar la búsqueda en el área separada.
- Si llega una nueva clave que también desborda la misma dirección base (por ejemplo, el 23, que también correspondía a la dirección 1), se inserta en la siguiente cubeta libre del área de desborde (por ejemplo, la cubeta 1). Aquí hay que mantener la cadena de sinónimos correctamente:
  - El enlace de la dirección base 1 se actualiza para apuntar ahora a la cubeta más nueva (cubeta 1), que pasa a ser la "cabeza" de la cadena de sinónimos.
  - La cubeta nueva (cubeta 1) hereda, como su propio enlace, el valor que tenía anteriormente la dirección base (en este caso, el puntero a la cubeta 0) — de esta forma, al leer la dirección base se llega primero a la cubeta 1 (con el 23) y desde ahí, siguiendo el enlace, se llega a la cubeta 0 (con el 12), preservando toda la cadena de sinónimos.

**Bajas en esta técnica**: hay dos casos posibles.

- Si la clave a eliminar está en el **área principal** (es decir, alojada en su propia dirección base): se elimina de forma directa. Simplemente se lee la cubeta, se reescribe sin esa clave, y termina la baja — no hace falta modificar ningún enlace.
- Si la clave a eliminar está en el **área de desborde**: hay que tener cuidado de no romper la cadena de sinónimos. Se reescribe la cubeta sin la clave eliminada, y además hay que actualizar el enlace del nodo que apuntaba a ella (su "anterior" en la cadena), de modo que ahora apunte a lo que apuntaba la cubeta eliminada — saltándola, para que el resto de la cadena de sinónimos se siga recorriendo correctamente. Por ejemplo, al eliminar el 12 (que estaba en la cubeta 0 del área de desborde, referenciada por la cubeta 1 donde está el 23): se borra el contenido de la cubeta 0, y se actualiza el enlace de la cubeta 1 (que apuntaba a la 0) para que tome el valor que tenía la cubeta 0 como enlace (en este caso, -1, indicando fin de cadena, ya que no había más sinónimos detrás del 12).

### Dispersión doble

Esta técnica también es progresiva, pero en lugar de ir buscando espacio libre avanzando de a una dirección hacia adelante (como en la saturación progresiva simple), busca el espacio disponible **aplicando una segunda función de dispersión** — de ahí su nombre.

**Funciones utilizadas en el ejemplo trabajado en clase** (memoria con varias direcciones, dispersando 16 claves en total):

- Primera función de dispersión: `h1(clave) = clave mod tamaño_de_tabla` (la misma función habitual usada en las demás técnicas).
- Segunda función de dispersión: `h2(clave) = (clave mod 5) + 1`. El resultado de esta segunda función se llama **desplazamiento**: indica cuánto hay que moverse desde la dirección base para buscar un lugar libre cuando se produce un desborde.

> **¿Por qué se suma 1 al resultado del mod?** Porque el resto de un mod a veces da cero. Si el desplazamiento pudiera ser cero, al sumarlo a la dirección base siempre se volvería a caer en la misma cubeta llena, generando un bucle infinito sin poder resolver nunca el desborde. Sumando 1 se garantiza que el desplazamiento sea siempre de al menos una posición. En general la cátedra da la función ya con el +1 incluido, pero si no se diera así, hay que sumarlo de todas formas para evitar este problema.

**Mecánica del ejemplo**: las primeras claves se dispersan sin inconvenientes, cada una en su dirección base. Cuando llega una clave cuya dirección base ya está ocupada (colisión con desborde), se aplica la segunda función de dispersión a esa clave para obtener el desplazamiento, y se suma ese desplazamiento a la dirección base para encontrar la nueva posición candidata:

- Por ejemplo, una clave con dirección base 1 que desborda: se aplica la segunda función y da un desplazamiento de 3; la nueva dirección candidata es 1 + 3 = 4. Si esa dirección tiene espacio, la clave se inserta ahí.
- Si la nueva dirección candidata **también** está ocupada, se vuelve a aplicar el mismo desplazamiento (el correspondiente a esa clave) sobre la última dirección probada, avanzando circularmente por la tabla hasta encontrar una cubeta con espacio libre. Por ejemplo, si una clave con dirección base 3 desborda con un desplazamiento de 2, se prueba primero la dirección 3+2=5; si esa también está llena, se continúa sumando el mismo desplazamiento: 5+2=7, y se inserta ahí si hay lugar.

**Bajas en esta técnica**: a diferencia de las técnicas anteriores, en dispersión doble **no conviene simplemente borrar y dejar la cubeta vacía**, porque el algoritmo de búsqueda necesita saber si tiene que seguir probando direcciones desplazadas o detenerse. Por eso se utiliza la llamada **marca de inutilización**: cada vez que se da de baja una clave, se coloca una marca especial en su cubeta indicando "aquí había una clave, pero fue borrada". De esa manera, el algoritmo de búsqueda sabe que, si encuentra una marca de este tipo, debe continuar buscando (porque puede haber más sinónimos desplazados más adelante), a diferencia de encontrar una cubeta realmente vacía, donde sí puede detenerse.

---

## Hashing Extensible

El hashing extensible pertenece al direccionamiento **dinámico**: a diferencia de las técnicas de direccionamiento estático, acá se tiene la capacidad de obtener o generar nuevos espacios a medida que se necesitan. Por eso ya no hace falta ningún mecanismo de cadenas, áreas de desborde o desplazamientos para resolver colisiones: la forma de resolverlas es simplemente **pedir más espacio y redispersar** las claves involucradas.

### Componentes y conceptos básicos

- **Tabla de dispersión (en memoria)**: tiene un valor llamado **bits de dispersión globales**, que indica cuántos bits de la función de dispersión se están evaluando para decidir a qué bloque del archivo apunta cada dirección. Siempre se toman los bits **más a la derecha** (los menos significativos) del resultado de la función de hash.
- **Bloques (en disco)**: cada bloque tiene también su propio valor de **bits de dispersión local**, que puede coincidir con el de la tabla o ser menor (nunca mayor, salvo en el instante en que se está resolviendo un desborde).
- Al comenzar, tanto la tabla como el único bloque inicial tienen el indicador de bits en 0, lo que significa que no se evalúa ningún bit de la clave: todas las claves entran sin distinción en el único bloque existente, mientras haya espacio.

### Ejemplo de parámetros

- **Función de dispersión**: retorna 10 bits.
- **Capacidad**: 2 registros por dirección (bloque).
- **Claves**: se van a dispersar 8 claves en total.

### Claves a dispersar

1. **Colapinto** (1011001100)
2. **Verstappen** (1110101000)
3. **Russell** (1010001001)
4. **Stroll** (1010101010)
5. **Alonso** (1010001000)
6. **Hamilton** (1001001011)
7. **Sainz** (1010001111)
8. **Leclerc** (1010100111)

### Estado inicial del archivo

#### Tabla de dispersión
- **Bits de dispersión:** 0
- **Sufijo:** (0) → **#Bloque:** 0

#### Archivo de datos
- **#Bloque 0** | **Bits:** 0 | **Clave R1:** [vacío] | **Clave R2:** [vacío]

### Se agregan Colapinto y Verstappen

Ambos se agregan sin inconvenientes en el bloque 0.

#### Archivo de datos
- **#Bloque 0** | **Bits:** 0 | **Clave R1:** Colapinto (1011001100) | **Clave R2:** Verstappen (1110101000)

### Se agrega Russell → produce desborde

El procedimiento general ante un desborde es siempre el mismo:

1. Se aumenta en uno el valor de bits de dispersión **local** del bloque saturado.
2. Se crea un **nuevo bloque**, con la misma cantidad de bits locales que se acaba de asignar al bloque saturado.
3. Se compara ese nuevo valor de bits locales con el valor de bits **globales** de la tabla:
   - Si son iguales, alcanza con reacomodar los enlaces existentes (no hace falta duplicar la tabla).
   - Si el valor local es mayor que el global (como ocurre la primera vez que se desborda un bloque), se aumenta en uno el valor global de la tabla y se **duplican** las direcciones disponibles.
4. Las claves se redistribuyen entre el bloque original y el nuevo, mirando los bits menos significativos que correspondan según la nueva cantidad de bits a evaluar.
5. La dirección de la tabla donde ocurrió el desborde pasa a apuntar al **nuevo** bloque creado.

> Una aclaración importante sobre el punto 5: que la dirección donde ocurrió el desborde sea la que pase a apuntar al bloque nuevo (y no al revés) es una **convención**, no una regla obligatoria del algoritmo — el resultado final sería igualmente válido si se hiciera al revés, siempre que se mantenga el criterio de forma consistente a lo largo de todo el ejercicio. Es el mismo tipo de convención que, por ejemplo, la de tomar "la menor de las claves mayores" al resolver un overflow en un árbol B, en lugar de "la mayor de las menores": cualquiera de las dos es válida, lo importante es aplicar siempre el mismo criterio.

En el caso de Russell: el bloque 0 tenía 0 bits locales, ahora pasa a tener 1. Se crea el bloque 1, también con 1 bit local. Como 1 (local) es mayor que 0 (global), se aumenta el valor global de la tabla a 1 y se duplican las direcciones. La dirección que desbordó pasa a apuntar al bloque nuevo (bloque 1); la dirección recién creada por la duplicación pasa a apuntar al bloque original (bloque 0). Las claves se redistribuyen mirando el último bit de cada una: Colapinto y Verstappen terminan en 0, por lo que van al bloque 1; Russell termina en 1, por lo que va al bloque 0.

#### Tabla de dispersión
- **Bits de dispersión:** 1
- **Sufijo (0):** #Bloque 1
- **Sufijo (1):** #Bloque 0

#### Archivo de datos
- **#Bloque 0** | **Bits:** 1 | **Clave R1:** Russell (1010001001) | **Clave R2:** [vacío]
- **#Bloque 1** | **Bits:** 1 | **Clave R1:** Colapinto (1011001100) | **Clave R2:** Verstappen (1110101000)

### Se agrega Stroll → produce desborde

Se produce desborde en el bloque 1. Se aumentan en uno los bits de dispersión locales del bloque 1 y se crea el bloque 2 con la misma cantidad de bits locales. Como los bits locales (2) superan a los bits globales de la tabla (1), se aumenta en uno el valor global y se duplican nuevamente las direcciones. Las claves se redistribuyen mirando ahora los dos bits menos significativos. La dirección donde ocurrió el desborde pasa a apuntar al nuevo bloque.

Es importante notar que **el bloque que no participó del desborde (el bloque 0, con Russell) queda exactamente igual que antes**: a lo sumo va a quedar referenciado por más direcciones si se duplicó la tabla, pero su contenido no se toca. Esta es la misma regla que rige en árboles B cuando se resuelve un overflow: los nodos que no están involucrados en el problema deben permanecer idénticos entre un paso y el siguiente. Si alguna cubeta que no tuvo desborde queda distinta al estado anterior, es señal de que algo se hizo mal.

#### Tabla de dispersión
- **Bits de dispersión:** 2
- **Sufijos:**
  - (00): #Bloque 2
  - (01): #Bloque 0
  - (10): #Bloque 1
  - (11): #Bloque 0

#### Archivo de datos
- **#Bloque 0** | **Bits:** 1 | **Clave R1:** Russell (1010001001) | **Clave R2:** [vacío]
- **#Bloque 1** | **Bits:** 2 | **Clave R1:** Stroll (1010101010) | **Clave R2:** [vacío]
- **#Bloque 2** | **Bits:** 2 | **Clave R1:** Colapinto (1011001100) | **Clave R2:** Verstappen (1110101000)

### Se agregan Alonso (desborde), Hamilton (entra normal), Sainz (desborde) y Leclerc (desborde)

Repitiendo el mismo procedimiento para cada desborde:

#### Tabla de dispersión
- **Bits de dispersión:** 3
- **Sufijos:**
  - (000): #Bloque 3
  - (001): #Bloque 0
  - (010): #Bloque 1
  - (011): #Bloque 4
  - (100): #Bloque 2
  - (101): #Bloque 0
  - (110): #Bloque 1
  - (111): #Bloque 5

#### Archivo de datos
- **#Bloque 0** | **Bits:** 2 | **Clave R1:** Russell (1010001001) | **Clave R2:** [vacío]
- **#Bloque 1** | **Bits:** 2 | **Clave R1:** Stroll (1010101010) | **Clave R2:** [vacío]
- **#Bloque 2** | **Bits:** 3 | **Clave R1:** Colapinto (1011001100) | **Clave R2:** [vacío]
- **#Bloque 3** | **Bits:** 3 | **Clave R1:** Alonso (1010001000) | **Clave R2:** Verstappen (1110101000)
- **#Bloque 4** | **Bits:** 3 | **Clave R1:** Hamilton (1001001011) | **Clave R2:** [vacío]
- **#Bloque 5** | **Bits:** 3 | **Clave R1:** Leclerc (1010100111) | **Clave R2:** Sainz (1010001111)

### Resumen del procedimiento de alta

Siempre que se produce colisión con desborde en hashing extensible, el procedimiento es el mismo, sin excepciones:

1. Se aumenta en uno los bits de dispersión locales del bloque saturado.
2. Se crea un nuevo bloque con esa misma cantidad de bits locales.
3. Se compara ese valor con los bits de dispersión globales de la tabla.
4. Si el valor local es mayor, se aumenta en uno los bits globales de la tabla y se duplican las direcciones; si es igual, solo se reacomodan los enlaces existentes.
5. Se redistribuyen las claves del bloque saturado (y, si corresponde, las del resto de las claves afectadas por la duplicación) según los bits que correspondan.

---

## Bajas en hashing extensible

### Caso simple

Si al eliminar una clave queda otra clave en esa misma dirección, se da de baja de forma directa, sin modificar ningún enlace — es equivalente al caso de una baja en un árbol B que no deja al nodo en underflow: simplemente se reescribe el bloque sin la clave eliminada.

### Doy de baja a Verstappen: se borra normal

#### Archivo de datos (Bloque 3 modificado)
- **#Bloque 3** | **Bits:** 3 | **Clave R1:** Alonso (1010001000) | ~~**Clave R2:** Verstappen (1110101000)~~ [vacío]

### Caso con bloque que queda vacío: ¿se puede liberar?

Cuando una baja deja un bloque completamente vacío, hay que decidir si ese bloque puede **liberarse** (y reabsorberse en otro) o si, por el contrario, debe quedar vacío sin liberar. La regla general para decidirlo es:

1. Se toma la cantidad de bits de dispersión local del bloque que quedó vacío, y se la reduce en uno.
2. Con esa cantidad reducida de bits, se identifican todas las entradas de la tabla cuyo sufijo coincide en esos bits con el del bloque vacío (su "bloque hermano" o hermanos potenciales), **descartando** la o las entradas que apuntaban al bloque que se quiere liberar.
3. Si **todas** esas entradas restantes apuntan a un mismo bloque, el bloque vacío se puede liberar: se actualizan las entradas de la tabla que apuntaban al bloque eliminado para que ahora apunten a ese bloque hermano, y se reduce en uno la cantidad de bits de dispersión local del bloque resultante.
4. Si esas entradas **no** apuntan todas a un mismo bloque, el bloque vacío **no se puede liberar**: simplemente se escribe el bloque vacío, sin tocar la tabla ni los enlaces existentes (las entradas que apuntaban a él van a seguir apuntando al mismo bloque, ahora vacío).

Una observación útil: cuando el bloque que queda vacío tiene la **máxima** cantidad de bits de dispersión (es decir, sus bits locales son iguales a los bits globales de la tabla), siempre va a existir exactamente una única entrada candidata a comparar, así que automáticamente se cumple la condición y el bloque se puede liberar.

### Al dar de baja a Alonso, el bloque 3 queda vacío

El bloque 3 tenía 3 bits de dispersión local (el máximo, igual al global). Su sufijo es 000; reduciendo en uno la cantidad de bits a considerar (es decir, mirando solo "00"), se identifica como posible hermano al bloque cuyo sufijo es 100 (el bloque 2), descartando la propia entrada que señalaba al bloque 3. Como esa única entrada hermana apunta al bloque 2, **se puede liberar** el bloque 3: se libera, se actualiza la entrada correspondiente de la tabla para que apunte al bloque 2, y se reduce en uno la cantidad de bits de dispersión local del bloque 2.

#### Tabla de dispersión
- **Bits de dispersión:** 3
- **Sufijos:**
  - (000): #Bloque **2** (modificado)
  - (001): #Bloque 0
  - (010): #Bloque 1
  - (011): #Bloque 4
  - (100): #Bloque 2
  - (101): #Bloque 0
  - (110): #Bloque 1
  - (111): #Bloque 5

#### Archivo de datos
- **#Bloque 2** | **Bits:** **2** (modificado) | **Clave R1:** Colapinto (1011001100) | **Clave R2:** [vacío]
- **#Bloque 3** | ~~**Bits:** 3~~ [Liberado]

### Al eliminar Stroll, el bloque 1 queda vacío

El bloque 1 tenía 2 bits de dispersión local y estaba referenciado por las direcciones terminadas en 10. Para evaluar si se puede liberar, se reduce en uno la cantidad de bits a considerar (mirando solo el último bit, "0") y se identifican las entradas hermanas, descartando las que apuntan al bloque 1: las entradas terminadas en 00 apuntan todas al mismo bloque (el 2), por lo tanto **se puede liberar** el bloque 1. Se actualizan las referencias que apuntaban a él para que apunten al bloque 2, y se reduce en uno la cantidad de bits de dispersión local de ese bloque.

#### Tabla de dispersión
- **Bits de dispersión:** 3
- **Sufijos:**
  - (000): #Bloque 2
  - (001): #Bloque 0
  - (010): #Bloque **2** (modificado)
  - (011): #Bloque 4
  - (100): #Bloque 2
  - (101): #Bloque 0
  - (110): #Bloque **2** (modificado)
  - (111): #Bloque 5

#### Archivo de datos
- **#Bloque 1** | ~~**Bits:** 2~~ [Liberado]
- **#Bloque 2** | **Bits:** **1** (modificado) | **Clave R1:** Colapinto (1011001100) | **Clave R2:** [vacío]

### Al eliminar Russell, el bloque 0 queda vacío (y no puede liberarse)

El bloque 0 tenía 2 bits de dispersión local y estaba referenciado por las direcciones terminadas en 01. Para evaluar si se puede liberar, se identifican las entradas hermanas con un bit menos (terminadas en 11), descartando las que apuntan al bloque 0: esas entradas terminadas en 11 **no** apuntan todas al mismo bloque (la entrada 011 apunta al bloque 4 y la entrada 111 apunta al bloque 5). Como no coinciden, **no se puede liberar** el bloque 0: se escribe el bloque vacío y no se modifica la tabla. Las entradas que apuntaban a él (001 y 101) siguen apuntando al mismo bloque, que ahora está vacío.

#### Tabla de dispersión
*(sin cambios)*

#### Archivo de datos
- **#Bloque 0** | **Bits:** 2 | **Clave R1:** [vacío] | **Clave R2:** [vacío]

### Estado final

#### Tabla de dispersión
- **Bits de dispersión:** 3
- **Sufijos:**
  - (000): #Bloque 2
  - (001): #Bloque 0
  - (010): #Bloque 2
  - (011): #Bloque 4
  - (100): #Bloque 2
  - (101): #Bloque 0
  - (110): #Bloque 2
  - (111): #Bloque 5

#### Archivo de datos
- **#Bloque 0** | **Bits:** 2 | **Clave R1:** [vacío] | **Clave R2:** [vacío]
- **#Bloque 2** | **Bits:** 1 | **Clave R1:** Colapinto (1011001100) | **Clave R2:** [vacío]
- **#Bloque 4** | **Bits:** 3 | **Clave R1:** Hamilton (1001001011) | **Clave R2:** [vacío]
- **#Bloque 5** | **Bits:** 3 | **Clave R1:** Leclerc (1010100111) | **Clave R2:** Sainz (1010001111)

---

## Resumen / reglas de oro de hashing

- **Direccionamiento estático**: el espacio total está fijo de antemano; siempre puede haber desbordes aunque la función de hash sea buena, y existen varias técnicas para resolverlos (saturación progresiva, saturación progresiva encadenada, saturación progresiva encadenada con área de desborde separado, dispersión doble).
- **Saturación progresiva encadenada con área de desborde separado**: todos los desbordes se resuelven en un área aparte, manteniendo más claves en sus direcciones base y acelerando las búsquedas. Las bajas en el área principal son directas; las bajas en el área de desborde requieren actualizar los enlaces de la cadena para no perder a los sinónimos restantes.
- **Dispersión doble**: ante un desborde, se usa una segunda función de dispersión para calcular un desplazamiento y encontrar una cubeta libre, en lugar de avanzar de a una posición. Al segundo resultado del hash conviene sumarle 1 para evitar desplazamientos nulos que generen bucles infinitos. Las bajas se marcan con una "marca de inutilización" en lugar de dejarse vacías sin más, para que las búsquedas sepan si deben seguir recorriendo la cadena de sinónimos.
- **Hashing extensible**: es direccionamiento dinámico — ante un desborde, siempre se aumenta en uno los bits de dispersión locales del bloque saturado, se crea un nuevo bloque con esos mismos bits, y se compara con los bits globales de la tabla para decidir si hace falta duplicar las direcciones. Los bloques que no participan del desborde deben quedar exactamente iguales que antes (a lo sumo, con más entradas de la tabla apuntándolos).
- **Convenciones**: a qué bloque (el viejo o el nuevo) queda apuntando la dirección que desbordó es una convención del algoritmo, no una regla rígida — lo importante es aplicar el mismo criterio de forma consistente, igual que ocurre con otras convenciones ya vistas en árboles B (por ejemplo, "la menor de las mayores" en vez de "la mayor de las menores").
- **Bajas en hashing extensible**: si la baja no deja el bloque vacío, se reescribe sin más. Si lo deja vacío, hay que evaluar si puede liberarse comparando con el o los bloques hermanos (con un bit menos de dispersión local, descartando la entrada que apuntaba al bloque vacío): si todos esos hermanos apuntan a un mismo bloque, se libera y se reduce en uno el nivel de dispersión local del bloque resultante; si no, el bloque queda vacío sin liberar.
