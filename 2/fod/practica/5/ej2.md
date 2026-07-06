## 2. Función de dispersión

Una **función de dispersión** `h(K)` toma un valor de clave `K` y devuelve una dirección dentro del rango permitido del archivo (por ejemplo, el NRR de una página o bloque). Su principal objetivo es distribuir las claves de manera lo más uniforme posible en el espacio de direcciones disponible, minimizando así las colisiones.

**Tres funciones de dispersión comunes:**

1. **Módulo (División):** Consiste en dividir la clave numérica por el tamaño del archivo (generalmente un número primo `M`) y tomar el resto como dirección: `h(K) = K mod M`. Es sencilla y muy eficaz si M se elige bien.
2. **Centros de Cuadrados:** Se eleva al cuadrado el valor de la clave numérica y se extraen los dígitos centrales del resultado para conformar la dirección. Esto ayuda a dispersar claves que difieren en pequeñas proporciones o que son muy secuenciales.
3. **Plegamiento:** Se divide la clave en partes de igual longitud y se las suma (a menudo ignorando acarreos) para generar la dirección. Resulta muy útil cuando la clave original es más grande que la cantidad de dígitos permitidos para la dirección física.
