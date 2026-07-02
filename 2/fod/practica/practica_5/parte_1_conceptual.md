# Práctica 5 - Hashing (Dispersión) - Parte 1: Conceptual

## 1. Concepto de hashing y relación con archivos
El **hashing (o dispersión)** es una técnica de organización y direccionamiento directo de datos que utiliza una función (función hash) para mapear el valor de una clave (por ejemplo, DNI o ID) directamente a una dirección física o lógica dentro de un espacio de almacenamiento.
En el contexto de los **archivos**, esta técnica se utiliza en la organización directa para acceder rápidamente a los registros almacenados en disco sin necesidad de realizar búsquedas secuenciales exhaustivas o depender de índices jerárquicos (como los árboles B), acercando el costo de acceso a O(1) lecturas de disco.

## 2. Función de dispersión
Una **función de dispersión** `h(K)` toma un valor de clave `K` y devuelve una dirección dentro del rango permitido del archivo (por ejemplo, el NRR de una página o bloque). Su principal objetivo es distribuir las claves de manera lo más uniforme posible en el espacio de direcciones disponible, minimizando así las colisiones.

**Tres funciones de dispersión comunes:**
1. **Módulo (División):** Consiste en dividir la clave numérica por el tamaño del archivo (generalmente un número primo `M`) y tomar el resto como dirección: `h(K) = K mod M`. Es sencilla y muy eficaz si M se elige bien.
2. **Centros de Cuadrados:** Se eleva al cuadrado el valor de la clave numérica y se extraen los dígitos centrales del resultado para conformar la dirección. Esto ayuda a dispersar claves que difieren en pequeñas proporciones o que son muy secuenciales.
3. **Plegamiento:** Se divide la clave en partes de igual longitud y se las suma (a menudo ignorando acarreos) para generar la dirección. Resulta muy útil cuando la clave original es más grande que la cantidad de dígitos permitidos para la dirección física.

## 3. Sinónimo, colisión y desborde
- **Sinónimos:** Son dos o más claves distintas (`K1 ≠ K2`) para las cuales la función de dispersión genera la misma dirección (`h(K1) = h(K2)`).
- **Colisión:** Ocurre en el momento de la inserción cuando la clave que se intenta insertar es mapeada a una dirección (bloque) donde ya existe almacenado otro registro sinónimo, pero en dicha dirección **aún hay espacio** disponible.
- **Desborde (Overflow):** Ocurre cuando una clave mapea a una dirección que ya se encuentra **completamente llena** y no puede aceptar más registros.
- **Condición para colisión sin desborde:** La dirección física (el bloque o página) al que se accede debe tener **capacidad para almacenar múltiples registros**. Si cada bloque puede almacenar sólo un registro, cada colisión es automáticamente un desborde.

## 4. Alternativas para reducir el número de colisiones
- **Elegir o mejorar la función de dispersión:** Buscar una función que distribuya los registros lo más uniformemente posible evitando aglomeraciones.
- **Aumentar la capacidad de página/bloque (factor de bloqueo):** Mientras más registros entren en un mismo bloque, más sinónimos pueden ser acomodados como meras colisiones antes de convertirse en desbordes.
- **Aumentar el tamaño del archivo (más bloques):** Reservando un espacio de disco mayor, lo cual equivale a reducir la densidad de empaquetamiento (hay más bloques, por lo que es menos probable que caigan en el mismo).

## 5. Densidad de empaquetamiento
La **densidad de empaquetamiento** es el cociente entre el número total de registros almacenados y la capacidad total de la estructura (registros almacenables por página multiplicado por el número de páginas).
- **Consecuencias de una menor densidad:** Significa que se ha asignado mucho más espacio físico (más bloques) del que estrictamente ocupan los registros reales. Esto disminuye dramáticamente la probabilidad de colisiones y desbordes, acelerando el acceso en promedio. La contracara es un importante desperdicio de espacio en disco (se almacenan muchos huecos vacíos).

## 6. Técnicas de resolución de desbordes (Hashing estático)

- **Saturación progresiva:** Consiste en buscar secuencialmente hacia adelante (dirección + 1, dirección + 2, etc.) partiendo desde la dirección de desborde hasta encontrar el primer bloque con espacio disponible y colocar el registro allí. Es simple, pero agrava el clustering (aglomeración primaria), ya que bloques sucesivos comienzan a llenarse interfiriendo con nuevas claves mapeadas a esas direcciones.
- **Saturación progresiva encadenada:** Mejora lo anterior agregando un puntero en el bloque original hacia el bloque donde finalmente se almacenó el registro desbordado, armando una lista enlazada para los sinónimos. Esto evita buscar secuencialmente y agiliza las lecturas.
- **Saturación progresiva encadenada con área de desborde separada:** El archivo físico se divide en un área primaria y un área de desborde separada. Los desbordes no se guardan en el área primaria contaminando otras direcciones, sino que se enlazan apuntando a un espacio en el área de desborde. Elimina el clustering primario por completo.
- **Dispersión doble:** En vez de probar linealmente la dirección adyacente, se usa una segunda función hash `h2(K)` para determinar el tamaño del salto a dar cuando ocurre un desborde: `dirección_alternativa = (h1(K) + i * h2(K)) mod M`. Esto evita eficazmente la aglomeración, dispersando los desbordes uniformemente.
