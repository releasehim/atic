# Introducción a las Bases de Datos

## Fundamentos de Organización de Datos

### Práctica 5

#### Hashing (Dispersión)

---

## Parte 1: Preguntas conceptuales

**1.** Defina el concepto de hashing (o dispersión). ¿Cómo se relaciona este concepto con archivos?

**2.** Explique el concepto de función de dispersión. Enumere al menos tres funciones de dispersión y explique brevemente cómo funciona cada una.

**3.** Explique los conceptos de sinónimo, colisión y desborde (overflow). ¿Qué condición es necesaria en el archivo directo para que pueda ocurrir una colisión y no un desborde?

**4.** ¿Qué alternativas existen para reducir el número de colisiones (y por ende de desbordes) en un archivo organizado mediante la técnica de hashing?

**5.** Explique brevemente qué es la densidad de empaquetamiento. ¿Cuáles son las consecuencias de tener una menor densidad de empaquetamiento en un archivo directo?

**6.** Explique brevemente cómo funcionan las siguientes técnicas de resolución de desbordes que se pueden utilizar en hashing estático.

- Saturación progresiva
- Saturación progresiva encadenada
- Saturación progresiva encadenada con área de desborde separada
- Dispersión doble

---

## Parte 2: Dispersión extensible

Nota: (+) indica una operación de inserción, (-) indica una operación de baja.

**7.** Para las siguientes claves, realice el proceso de dispersión mediante el método de hashing extensible, sabiendo que cada nodo tiene capacidad para dos registros. El número natural indica el orden de llegada de las operaciones. Se debe mostrar el estado del archivo para cada operación. Justifique brevemente ante colisión y desborde los pasos que realiza.

| # | Operación | Clave | Código binario |
| --- | --- | --- | --- |
| 1 | + | Darin | 00111111 |
| 2 | + | Alterio | 11110100 |
| 3 | + | Sbaraglia | 10100101 |
| 4 | + | De la Serna | 01010111 |
| 5 | + | Altavista | 01101011 |
| 6 | + | Grandinetti | 10101010 |
| 7 | - | Altavista | 01101011 |
| 8 | - | Sbaraglia | 10100101 |

**8.** Realice el proceso de dispersión mediante el método de hashing extensible, sabiendo que cada registro tiene capacidad para dos claves. El número natural indica el orden de llegada de las operaciones. Se debe mostrar el estado del archivo para cada operación. Justifique brevemente ante colisión y desborde los pasos que realiza.

| # | Operación | Clave | Código binario |
| --- | --- | --- | --- |
| 1 | + | Buenos Aires | ....1001 |
| 2 | + | San Juan | ....0100 |
| 3 | + | Entre Ríos | ....1110 |
| 4 | + | Corrientes | ....0010 |
| 5 | + | San Luis | ....0101 |
| 6 | + | Tucumán | ....0111 |
| 7 | + | Rio Negro | ....0011 |
| 8 | + | Jujuy | ....1111 |
| 9 | + | Salta | ....1010 |
| 10 | - | Río Negro | ....0011 |

**9.** Para las siguientes claves, realice el proceso de dispersión mediante el método de hashing extensible, sabiendo que cada nodo tiene capacidad para dos registros. El número natural indica el orden de llegada de las operaciones. Se debe mostrar el estado del archivo para cada operación. Justifique brevemente ante colisión y desborde los pasos que realiza.

| # | Operación | Clave | Código binario |
| --- | --- | --- | --- |
| 1 | + | Guillermo.B | 01100011 |
| 2 | + | Gomez | 00000001 |
| 3 | + | Gustavo.B | 01010110 |
| 4 | + | Sosa | 11110100 |
| 5 | + | Enria | 00110101 |
| 6 | + | Guli | 00101000 |
| 7 | - | Gustavo.B | 01010110 |
| 8 | - | Sosa | 11110100 |

**10.** Para las siguientes claves, realice el proceso de dispersión mediante el método de hashing extensible, sabiendo que cada nodo tiene capacidad para dos registros. El número natural indica el orden de llegada de las operaciones. Se debe mostrar el estado del archivo para cada operación. Justifique brevemente ante colisión y desborde los pasos que realiza.

| # | Operación | Clave | Código binario |
| --- | --- | --- | --- |
| 1 | + | Mansilla | 01100010 |
| 2 | + | Cetré | 10001000 |
| 3 | + | Ascacibar | 01010111 |
| 4 | + | Carrillo | 11110101 |
| 5 | + | Manyoma | 00110100 |
| 6 | + | Méndez | 00101001 |
| 7 | + | Alario | 11000101 |
| 8 | - | Mansilla | 01100010 |

**11.** Realice el proceso de dispersión mediante el método de hashing extensible, sabiendo que cada nodo tiene capacidad para dos claves. El número natural indica el orden de llegada de las operaciones. Deberá explicar los pasos que realiza en cada operación y dibujar los estados sucesivos correspondientes (inclusive el estado inicial).

| # | Operación | Clave | Código binario |
| --- | --- | --- | --- |
| 1 | + | Aconcagua | 10100111 |
| 2 | + | Kilimanjaro | 10101010 |
| 3 | + | Mont Blanc | 00111110 |
| 4 | + | Cervino | 01101111 |
| 5 | + | Etna | 00110101 |
| 6 | + | Chañi | 11110000 |
| 7 | + | Cho Oyu | 01011101 |
| 8 | + | Vinicunca | 01011011 |
| 9 | - | Chañi | 11110000 |
| 10 | - | Cervino | 01101111 |

**12.** Considere un archivo organizado mediante hashing extensible, donde cada nodo tiene capacidad para dos claves. Identifique qué opción representa un estado válido del archivo y justifique por qué las demás opciones son inválidas.

**Opción A**

Tabla de dispersión (Bits: 2)

| Sufijo | #Bloque |
| --- | --- |
| 00 | 0 |
| 01 | 0 |
| 10 | 2 |
| 11 | 1 |

Archivo de datos

| #Blo | Bits | Clave R1 | Clave R2 |
| --- | --- | --- | --- |
| 0 | 2 | Jose (11001100) | Luis (00001100) |
| 1 | 1 | Diego (000000011) | |
| 2 | 2 | Maria (10101010) | |

**Opción B**

Tabla de dispersión (Bits: 2)

| Sufijo | #Bloque |
| --- | --- |
| 00 | 0 |
| 01 | 1 |
| 10 | 2 |
| 11 | 1 |

Archivo de datos

| #Blo | Bits | Clave R1 | Clave R2 |
| --- | --- | --- | --- |
| 0 | 2 | Jose (11001100) | Luis (00001100) |
| 1 | 1 | Diego (000000011) | |
| 2 | 2 | Maria (10101010) | |

**Opción C**

Tabla de dispersión (Bits: 2)

| Sufijo | #Bloque |
| --- | --- |
| 00 | 2 |
| 01 | 1 |
| 10 | 0 |
| 11 | 1 |

Archivo de datos

| #Blo | Bits | Clave R1 | Clave R2 |
| --- | --- | --- | --- |
| 0 | 2 | Jose (11001100) | Luis (00001100) |
| 1 | 1 | Diego (000000011) | |
| 2 | 2 | Maria (10101010) | |

**Opción D**

Tabla de dispersión (Bits: 2)

| Sufijo | #Bloque |
| --- | --- |
| 00 | 2 |
| 01 | 1 |
| 10 | 0 |
| 11 | 1 |

Archivo de datos

| #Blo | Bits | Clave R1 | Clave R2 |
| --- | --- | --- | --- |
| 0 | 2 | Jose (11001100) | Luis (00001100) |
| 1 | 2 | Diego (000000011) | |
| 2 | 2 | Maria (10101010) | |

**13.** Indique cuál de las siguientes opciones representa un estado inicial válido en un archivo organizado mediante la técnica de hashing extensible. Justifique su respuesta.

**Opción A**

Tabla de dispersión (Bits: 1)

| Sufijo | #Bloque |
|---|---|
| - | 0 |

Archivo de datos

| #Blo | Bits | Clave R1 | Clave R2 |
|---|---|---|---|
| 0 | 0 | | |

**Opción B**

Tabla de dispersión (Bits: 0)

| Sufijo | #Bloque |
|---|---|
| - | 0 |

Archivo de datos

| #Blo | Bits | Clave R1 | Clave R2 |
|---|---|---|---|
| 0 | 0 | Boca (10001000) | |

**Opción C**

Tabla de dispersión (Bits: 0)

| Sufijo | #Bloque |
|---|---|
| - | 0 |

Archivo de datos

| #Blo | Bits | Clave R1 | Clave R2 |
|---|---|---|---|
| 0 | 0 | | |

**Opción D**

Tabla de dispersión (Bits: 1)

| Sufijo | #Bloque |
| --- | --- |
| 0 | 0 |
| 1 | 1 |

Archivo de datos

| #Blo | Bits | Clave R1 | Clave R2 |
| --- | --- | --- | --- |
| 0 | 1 | | |
| 1 | 1 | | |

**14.** Se tiene el siguiente archivo organizado mediante la técnica de hashing extensible:

Tabla de dispersión (Bits: 2)

| Sufijo | #Bloque |
| --- | --- |
| 00 | 0 |
| 01 | 1 |
| 10 | 2 |
| 11 | 1 |

Archivo de datos

| #Blo | Bits | Clave R1 | Clave R2 |
| --- | --- | --- | --- |
| 0 | 2 | Blanco (11001100) | Rojo (00001100) |
| 1 | 1 | Rosa (000000011) | |
| 2 | 2 | Azul (10101010) | |

¿Qué sucede al agregar la clave Verde (00111100) en la estructura dada? Indique la opción correcta.

a. No produce desborde y se inserta sin problemas en el bloque #2.
b. Produce desborde en el bloque #0, pero como hay lugar en el bloque #2, se inserta en ese bloque.
c. Produce desborde en el bloque #0, se crea un nuevo bloque en el archivo de datos (bloque #3) sin necesidad de duplicar la cantidad de direcciones en la tabla de dispersión.
d. Produce desborde en el bloque #0, se crea un nuevo bloque en el archivo de datos (bloque #3) y se duplica la cantidad de direcciones en la tabla de dispersión.

**15.** Se tiene un archivo de datos organizado mediante la técnica de hashing extensible, donde cada nodo tiene capacidad para dos claves. ¿Cuál es el estado correcto del archivo al intentar insertar las claves Python (01110011), Java (10100111) y PHP (01111111)? Justifique su respuesta.

**Opción A**

Tabla de dispersión (Bits: 2)

| Sufijo | #Bloque |
| --- | --- |
| 00 | 0 |
| 01 | 1 |
| 10 | 0 |
| 11 | 2 |

Archivo de datos

| #Blo | Bits | Clave R1 | Clave R2 |
| --- | --- | --- | --- |
| 0 | 1 | | |
| 1 | 2 | Python (01110011) | |
| 2 | 2 | Java (10100111) | PHP (01111111) |

**Opción B**

Tabla de dispersión (Bits: 3)

| Sufijo | #Bloque |
| --- | --- |
| 00 | 0 |
| 01 | 1 |
| 10 | 0 |
| 11 | 2 |

Archivo de datos

| #Blo | Bits | Clave R1 | Clave R2 |
| --- | --- | --- | --- |
| 0 | 1 | | |
| 1 | 2 | Python (01110011) | |
| 2 | 3 | Java (10100111) | PHP (01111111) |

**Opción C**

Tabla de dispersión (Bits: 3)

| Sufijo | #Bloque |
| --- | --- |
| 000 | 0 |
| 001 | 1 |
| 010 | 0 |
| 011 | 2 |
| 100 | 0 |
| 101 | 1 |
| 110 | 0 |
| 111 | 3 |

Archivo de datos

| #Blo | Bits | Clave R1 | Clave R2 |
| --- | --- | --- | --- |
| 0 | 1 | | |
| 1 | 2 | | |
| 2 | 3 | Python (01110011) | |
| 3 | 3 | Java (10100111) | PHP (01111111) |

**Opción D**

Tabla de dispersión (Bits: 3)

| Sufijo | #Bloque |
| --- | --- |
| 000 | 0 |
| 001 | 0 |
| 010 | 0 |
| 011 | 1 |
| 100 | 0 |
| 101 | 0 |
| 110 | 0 |
| 111 | 2 |

Archivo de datos

| #Blo | Bits | Clave R1 | Clave R2 |
| --- | --- | --- | --- |
| 0 | 1 | | |
| 1 | 3 | Python (01110011) | |
| 2 | 3 | Java (10100111) | PHP (01111111) |
