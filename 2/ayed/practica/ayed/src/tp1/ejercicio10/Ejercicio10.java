package tp1.ejercicio10;

import tp1.ejercicio8.Queue;

public class Ejercicio10 {

    public static class Cliente {
        private String nombre;
        private boolean tienePrioridad;

        public Cliente(String nombre, boolean tienePrioridad) {
            this.nombre = nombre;
            this.tienePrioridad = tienePrioridad;
        }

        public String getNombre() {
            return nombre;
        }

        public boolean isTienePrioridad() {
            return tienePrioridad;
        }

        @Override
        public String toString() {
            return nombre + (tienePrioridad ? " (Prioritario)" : " (Regular)");
        }
    }

    public static void main(String[] args) {
        System.out.println("=== Simulación de Cola de Banco con Prioridad (Ejercicio 10) ===");

        // Crear las dos colas
        Queue<Cliente> colaPrioritaria = new Queue<>();
        Queue<Cliente> colaRegular = new Queue<>();

        // Simular llegada de clientes
        Cliente[] llegadas = {
            new Cliente("Juan", false),
            new Cliente("Maria (Embarazada)", true),
            new Cliente("Pedro", false),
            new Cliente("Carlos (Mayor de 70)", true),
            new Cliente("Sofia", false)
        };

        for (Cliente c : llegadas) {
            System.out.println("Llega cliente: " + c);
            if (c.isTienePrioridad()) {
                colaPrioritaria.enqueue(c);
            } else {
                colaRegular.enqueue(c);
            }
        }

        System.out.println("\nEstado de las colas:");
        System.out.println("Cola Prioritaria: " + colaPrioritaria);
        System.out.println("Cola Regular: " + colaRegular);

        System.out.println("\n--- Iniciando atención de clientes ---");
        // Atender hasta que ambas estén vacías, dando prioridad a la cola de prioridad
        while (!colaPrioritaria.isEmpty() || !colaRegular.isEmpty()) {
            Cliente atendido;
            if (!colaPrioritaria.isEmpty()) {
                atendido = colaPrioritaria.dequeue();
            } else {
                atendido = colaRegular.dequeue();
            }
            System.out.println("Atendiendo a: " + atendido);
        }
    }
}
