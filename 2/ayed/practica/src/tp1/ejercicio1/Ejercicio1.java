package tp1.ejercicio1;

public class Ejercicio1 {

    // a. Con un for
    public static void imprimirConFor(int a, int b) {
        if (a <= b) {
            for (int i = a; i <= b; i++) {
                System.out.println(i);
            }
        } else {
            for (int i = a; i >= b; i--) {
                System.out.println(i);
            }
        }
    }

    // b. Con un while
    public static void imprimirConWhile(int a, int b) {
        if (a <= b) {
            int i = a;
            while (i <= b) {
                System.out.println(i);
                i++;
            }
        } else {
            int i = a;
            while (i >= b) {
                System.out.println(i);
                i--;
            }
        }
    }

    // c. Sin estructuras de control iterativas (recursión)
    public static void imprimirSinIteracion(int a, int b) {
        System.out.println(a);
        if (a < b) {
            imprimirSinIteracion(a + 1, b);
        } else if (a > b) {
            imprimirSinIteracion(a - 1, b);
        }
    }

    public static void main(String[] args) {
        System.out.println("--- Probando con a = 1, b = 5 ---");
        System.out.println("Método A (for):");
        imprimirConFor(1, 5);

        System.out.println("Método B (while):");
        imprimirConWhile(1, 5);

        System.out.println("Método C (recursión):");
        imprimirSinIteracion(1, 5);

        System.out.println("--- Probando con a = 5, b = 2 ---");
        System.out.println("Método A (for):");
        imprimirConFor(5, 2);

        System.out.println("Método B (while):");
        imprimirConWhile(5, 2);

        System.out.println("Método C (recursión):");
        imprimirSinIteracion(5, 2);
    }
}
