package tp1.ejercicio2;

import java.util.Scanner;

public class Ejercicio2 {

    public static int[] obtenerMultiplos(int n) {
        int[] multiplos = new int[n];
        for (int i = 0; i < n; i++) {
            multiplos[i] = n * (i + 1);
        }
        return multiplos;
    }

    public static void main(String[] args) {
        Scanner s = new Scanner(System.in);
        System.out.print("Ingrese un número entero n (o escriba algo que no sea entero para salir): ");
        if (s.hasNextInt()) {
            int n = s.nextInt();
            if (n < 0) {
                System.out.println("Por favor ingrese un número mayor o igual a 0.");
            } else {
                int[] res = obtenerMultiplos(n);
                System.out.print("f(" + n + ") = [");
                for (int i = 0; i < res.length; i++) {
                    System.out.print(res[i]);
                    if (i < res.length - 1) {
                        System.out.print(", ");
                    }
                }
                System.out.println("]");
            }
        } else {
            System.out.println("\nNo se ingresó un entero por teclado. Corriendo pruebas automáticas por defecto...");
            int[] testValues = {1, 5, 8};
            for (int val : testValues) {
                int[] res = obtenerMultiplos(val);
                System.out.print("f(" + val + ") = [");
                for (int i = 0; i < res.length; i++) {
                    System.out.print(res[i]);
                    if (i < res.length - 1) {
                        System.out.print(", ");
                    }
                }
                System.out.println("]");
            }
        }
        s.close();
    }
}
