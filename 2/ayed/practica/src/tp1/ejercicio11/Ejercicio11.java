package tp1.ejercicio11;

import tp1.ejercicio8.CircularQueue;

public class Ejercicio11 {

    public static void main(String[] args) {
        System.out.println("=== Simulación de Recorrido de Colectivo (Ejercicio 11) ===");

        // Crear la cola circular de paradas
        CircularQueue<String> paradas = new CircularQueue<>();
        
        // Agregar las paradas de la línea de colectivo (ej. Línea Este)
        paradas.enqueue("Parada 1: Plaza Moreno (12 y 54)");
        paradas.enqueue("Parada 2: Plaza Paso (13 y 44)");
        paradas.enqueue("Parada 3: Plaza Italia (7 y 44)");
        paradas.enqueue("Parada 4: Terminal de Ómnibus (4 y 42)");
        paradas.enqueue("Parada 5: Plaza Rocha (7 y 60)");

        System.out.println("Paradas del recorrido: " + paradas);

        System.out.println("\n--- Simulación del Colectivo en Movimiento ---");
        // Simular que el colectivo pasa por las paradas de forma circular (8 visitas)
        for (int i = 1; i <= 8; i++) {
            String paradaActual = paradas.head();
            System.out.println("Visita " + i + ": El colectivo está en: " + paradaActual);
            
            // Avanzar a la siguiente parada rotando la cola
            paradas.shift();
            System.out.println("  [Avanzando...] Se rotó la parada visitada al final.");
        }
        
        System.out.println("\nEstado final de las paradas:");
        System.out.println(paradas);
    }
}
