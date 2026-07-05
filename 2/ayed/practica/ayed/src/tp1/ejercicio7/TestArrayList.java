package tp1.ejercicio7;

import java.util.ArrayList;

public class TestArrayList {
    public static void main(String[] args) {
        ArrayList<Integer> lista = new ArrayList<>();
        
        if (args.length > 0) {
            for (String arg : args) {
                try {
                    lista.add(Integer.parseInt(arg));
                } catch (NumberFormatException e) {
                    System.err.println("Argumento no válido: " + arg);
                }
            }
        } else {
            System.out.println("No se pasaron argumentos por línea de comandos.");
            // Usar una secuencia por defecto para demostrar su funcionamiento si no hay argumentos
            System.out.println("Usando secuencia por defecto: 10, 20, 30, 40, 50");
            lista.add(10);
            lista.add(20);
            lista.add(30);
            lista.add(40);
            lista.add(50);
        }

        System.out.println("Imprimiendo elementos de la lista:");
        for (Integer num : lista) {
            System.out.println(num);
        }
    }
}
