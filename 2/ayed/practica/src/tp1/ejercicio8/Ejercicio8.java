package tp1.ejercicio8;

public class Ejercicio8 {
    public static void main(String[] args) {
        // 1. Probar Queue estándar
        System.out.println("=== Probando Queue estándar ===");
        Queue<Integer> queue = new Queue<>();
        System.out.println("Está vacía? " + queue.isEmpty());
        queue.enqueue(10);
        queue.enqueue(20);
        queue.enqueue(30);
        System.out.println("Elementos: " + queue);
        System.out.println("Tamaño: " + queue.size());
        System.out.println("Frente (head): " + queue.head());
        System.out.println("Desencolar (dequeue): " + queue.dequeue());
        System.out.println("Elementos restantes: " + queue);
        
        // 2. Probar CircularQueue
        System.out.println("\n=== Probando CircularQueue ===");
        CircularQueue<String> circular = new CircularQueue<>();
        circular.enqueue("A");
        circular.enqueue("B");
        circular.enqueue("C");
        System.out.println("Elementos iniciales: " + circular);
        System.out.println("Rotando elemento (shift): " + circular.shift());
        System.out.println("Elementos después del shift: " + circular);
        System.out.println("Rotando elemento (shift): " + circular.shift());
        System.out.println("Elementos después del shift: " + circular);

        // 3. Probar DoubleEndedQueue
        System.out.println("\n=== Probando DoubleEndedQueue ===");
        DoubleEndedQueue<Double> doubleEnded = new DoubleEndedQueue<>();
        doubleEnded.enqueue(1.1);
        doubleEnded.enqueue(2.2);
        System.out.println("Elementos iniciales: " + doubleEnded);
        System.out.println("Encolando 0.0 al inicio (enqueueFirst)...");
        doubleEnded.enqueueFirst(0.0);
        System.out.println("Elementos después de enqueueFirst: " + doubleEnded);
        System.out.println("Desencolando: " + doubleEnded.dequeue());
        System.out.println("Elementos finales: " + doubleEnded);
    }
}
