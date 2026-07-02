## 3. Sinónimo, colisión y desborde
- **Sinónimos:** Son dos o más claves distintas (`K1 ≠ K2`) para las cuales la función de dispersión genera la misma dirección (`h(K1) = h(K2)`).
- **Colisión:** Ocurre en el momento de la inserción cuando la clave que se intenta insertar es mapeada a una dirección (bloque) donde ya existe almacenado otro registro sinónimo, pero en dicha dirección **aún hay espacio** disponible.
- **Desborde (Overflow):** Ocurre cuando una clave mapea a una dirección que ya se encuentra **completamente llena** y no puede aceptar más registros.
- **Condición para colisión sin desborde:** La dirección física (el bloque o página) al que se accede debe tener **capacidad para almacenar múltiples registros**. Si cada bloque puede almacenar sólo un registro, cada colisión es automáticamente un desborde.
