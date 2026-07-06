# Ejercicio 2: Recorridos en Árboles Generales

### Respuestas Teóricas

#### b. Modificaciones al implementar los recorridos dentro de la clase `GeneralTree<T>`

Si los métodos de recorrido se definieran directamente como métodos de instancia en la clase `GeneralTree<T>`, se requerirían las siguientes modificaciones:

1. **Firma del método:**
    * No sería necesario pasar el árbol `GeneralTree<Integer> a` como parámetro, ya que el método operaría sobre el objeto receptor (`this`).
    * Dado que `GeneralTree<T>` es una clase genérica, si queremos mantener el tipo genérico `T`, el método no sabría si los datos son números. Por lo tanto, para poder hacer operaciones aritméticas (`%` y `>`), tendríamos que realizar un casteo explícito de `data` a `Integer` (asumiendo que el árbol almacena enteros) o restringir la firma.
    * La firma quedaría así:

        ```java
        public List<Integer> numerosImparesMayoresQuePreOrden(Integer n)
        ```

2. **Implementación del método:**
    * En lugar de hacer llamadas a un método externo pasándole el nodo hijo, la recursión se invocaría enviando el mensaje directamente al nodo hijo utilizando programación orientada a objetos.
    * Por ejemplo, el método auxiliar para preorden se reescribiría de la siguiente forma:

        ```java
        public List<Integer> numerosImparesMayoresQuePreOrden(Integer n) {
            List<Integer> resultado = new LinkedList<>();
            this.preOrdenAux(n, resultado);
            return resultado;
        }

        private void preOrdenAux(Integer n, List<Integer> resultado) {
            // Asumimos que T es Integer y realizamos el casteo
            Integer val = (Integer) this.getData();
            if (val % 2 != 0 && val > n) {
                resultado.add(val);
            }
            for (GeneralTree<T> child : this.getChildren()) {
                // Invocación polimórfica en cada hijo
                child.preOrdenAux(n, resultado);
            }
        }
        ```
