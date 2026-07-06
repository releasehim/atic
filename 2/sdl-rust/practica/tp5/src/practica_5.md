# Práctica 5: Archivos, Testing, Manejo de errores

**UNLP. Facultad de Informática.**  
**Seminario de Lenguajes opción Rust - Cursada 2026**

---

### 1. En base al ejercicio 7 del TP#3 (Concesionario de autos), implemente lo siguiente

*Nota: El ejercicio 7 del TP#3 consiste en un Concesionario con capacidad máxima y autos con atributos como marca, modelo, precio, año y color.*

1. **a.** Al agregar un auto, si se supera el límite de la concesionaria, debe arrojar un error propio con un mensaje de contexto.
2. **b.** Haga todos los tests correspondientes para probar en profundidad los métodos que agregan un auto y eliminan un auto de la concesionaria, obteniendo el mayor porcentaje de *coverage* sobre el código que realiza las operaciones.
3. **c.** Una vez hecho el punto anterior, debe hacer que los autos de la concesionaria se almacenen en un archivo en formato JSON. Agregue y modifique lo que considere necesario para que:
    * Al agregar un nuevo auto, se abra el archivo de autos guardados y lo agregue a dicho archivo.
    * Al eliminar un auto, se debe eliminar este del archivo.
    * No debe modificar los tests hechos en el punto b. Si puede agregar más en caso de que haga nueva funcionalidad.

---

### 2. En base al ejercicio 8 del TP#3 (Playlist de canciones), implemente lo siguiente

*Nota: El ejercicio 8 del TP#3 consiste en una Playlist con canciones (título, artista, género).*

1. **a.** Realice todos los tests de la funcionalidad implementada obteniendo un *coverage* de por lo menos 90%.
2. **b.** Una vez obtenido dicho *coverage*, las canciones de la playlist deben ser guardadas en un archivo en formato JSON, por lo tanto, las operaciones que agreguen, quiten o modifiquen la playlist deben estar respaldadas sobre dicho archivo.
    * No debe modificar los tests hechos en el punto a. Si puede agregar más en caso de que haga métodos nuevos.
    * Recuerde también que se debe seguir manteniendo un *coverage* de al menos 90%.

---

### 3. En base al ejercicio 9 del TP#3 (Veterinarias y atención de mascotas), implemente lo siguiente

*Nota: El ejercicio 9 del TP#3 consiste en un sistema de atención de mascotas con cola de espera, registro de atenciones y dueños.*

1. **a.** Realice todos los tests de la funcionalidad implementada obteniendo un *coverage* de por lo menos 90%.
2. **b.** Ahora el registro de atenciones debe persistir en un archivo en formato JSON, es decir, todas las operaciones de lectura, agregar y modificación de atenciones se realizan sobre un archivo.
    * No debe modificar los tests hechos en el punto a. Si puede agregar más en caso de que haga métodos nuevos para cumplir con este punto.
    * Recuerde también que se debe seguir manteniendo un *coverage* de al menos 90%.

---

### 4. En base al ejercicio 10 del TP#3 (Biblioteca y sistema de préstamos), implemente lo siguiente

*Nota: El ejercicio 10 del TP#3 consiste en un sistema de préstamos de libros con control de copias y gestión de clientes.*

1. **a.** Realice todos los tests de la funcionalidad implementada obteniendo un *coverage* de por lo menos 90%.
2. **b.** Tanto los libros con sus copias como la administración de préstamos se realizan sobre archivos en formato JSON. Realice las modificaciones pertinentes para poder hacerlo así.
    * No debe modificar los tests hechos en el punto a. Si puede agregar más en caso de que haga métodos nuevos para cumplir con este punto.
    * Recuerde también que se debe seguir manteniendo un *coverage* de al menos 90%.

---

### 5. En base al ejercicio 3 del TP#4 (Plataforma de streaming "StreamingRust"), implemente lo siguiente

*Nota: El ejercicio 3 del TP#4 consiste en un sistema de suscripciones (Basic, Classic, Super) y medios de pago.*

1. **a.** Realice todos los tests de la funcionalidad implementada obteniendo un *coverage* de por lo menos 90%.
2. **b.** Todas las suscripciones deben almacenarse en un archivo en formato JSON. Implemente lo necesario para que toda la funcionalidad de las suscripciones se realice guardando, leyendo o modificando archivos.
    * No debe modificar los tests hechos en el punto a. Si puede agregar más en caso de que haga métodos nuevos para cumplir con este punto.
    * Recuerde también que se debe seguir manteniendo un *coverage* de al menos 90%.

---

### 6. En base al ejercicio 5 del TP#4 (Plataforma de criptoactivos XYZ), implemente lo siguiente

*Nota: El ejercicio 5 del TP#4 consiste en un sistema de intercambio de criptomonedas con balances de usuarios, transacciones (compras, ventas, retiros) y blockchains.*

1. **a.** Realice todos los tests de la funcionalidad implementada obteniendo un *coverage* de por lo menos 90%.
2. **b.** Todos los balances de los usuarios, así como las transacciones, deben persistir en archivos en formato JSON.
    * No debe modificar los tests hechos en el punto a. Si puede agregar más en caso de que haga métodos nuevos para cumplir con este punto.
    * Recuerde también que se debe seguir manteniendo un *coverage* de al menos 90%.
