package tp1.ejercicio6;

import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;

public class Ejercicio6 {
    public static void main(String[] args) {
        int N = 50000;
        
        System.out.println("Comparando rendimiento de ArrayList y LinkedList con N = " + N);

        // 1. Inserción al inicio
        System.out.println("\n--- Prueba 1: Inserción de " + N + " elementos al inicio (posición 0) ---");
        
        long start = System.nanoTime();
        List<Integer> arrayList = new ArrayList<>();
        for (int i = 0; i < N; i++) {
            arrayList.add(0, i);
        }
        long end = System.nanoTime();
        double timeArrayListInsert = (end - start) / 1e6;
        System.out.printf("ArrayList (insertar al inicio): %.2f ms\n", timeArrayListInsert);

        start = System.nanoTime();
        List<Integer> linkedList = new LinkedList<>();
        for (int i = 0; i < N; i++) {
            linkedList.add(0, i);
        }
        end = System.nanoTime();
        double timeLinkedListInsert = (end - start) / 1e6;
        System.out.printf("LinkedList (insertar al inicio): %.2f ms\n", timeLinkedListInsert);

        // 2. Acceso aleatorio
        System.out.println("\n--- Prueba 2: Acceso aleatorio (recuperar elemento en la mitad) 5000 veces ---");
        
        int mitad = N / 2;
        int accesos = 5000;

        start = System.nanoTime();
        for (int i = 0; i < accesos; i++) {
            @SuppressWarnings("unused")
            int val = arrayList.get(mitad);
        }
        end = System.nanoTime();
        double timeArrayListGet = (end - start) / 1e6;
        System.out.printf("ArrayList (get en la mitad): %.2f ms\n", timeArrayListGet);

        start = System.nanoTime();
        for (int i = 0; i < accesos; i++) {
            @SuppressWarnings("unused")
            int val = linkedList.get(mitad);
        }
        end = System.nanoTime();
        double timeLinkedListGet = (end - start) / 1e6;
        System.out.printf("LinkedList (get en la mitad): %.2f ms\n", timeLinkedListGet);
    }
}
