## 5. Densidad de empaquetamiento

La **densidad de empaquetamiento** es el cociente entre el número total de registros almacenados y la capacidad total de la estructura (registros almacenables por página multiplicado por el número de páginas).

- **Consecuencias de una menor densidad:** Significa que se ha asignado mucho más espacio físico (más bloques) del que estrictamente ocupan los registros reales. Esto disminuye dramáticamente la probabilidad de colisiones y desbordes, acelerando el acceso en promedio. La contracara es un importante desperdicio de espacio en disco (se almacenan muchos huecos vacíos).
