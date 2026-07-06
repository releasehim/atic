package tp3.ejercicio2;

import java.util.List;
import tp3.GeneralTree;

public class Ejercicio2 {
    public static void main(String[] args) {
        System.out.println("=== Probando Ejercicio 2: Recorridos en Árbol General ===");

        // Construir el árbol:
        //            [7]
        //          /  |  \
        //        [3] [9] [11]
        //       / \   |
        //     [5] [8][13]
        
        GeneralTree<Integer> n5 = new GeneralTree<>(5);
        GeneralTree<Integer> n8 = new GeneralTree<>(8);
        GeneralTree<Integer> n3 = new GeneralTree<>(3);
        n3.addChild(n5);
        n3.addChild(n8);

        GeneralTree<Integer> n13 = new GeneralTree<>(13);
        GeneralTree<Integer> n9 = new GeneralTree<>(9);
        n9.addChild(n13);

        GeneralTree<Integer> n11 = new GeneralTree<>(11);

        GeneralTree<Integer> raiz = new GeneralTree<>(7);
        raiz.addChild(n3);
        raiz.addChild(n9);
        raiz.addChild(n11);

        RecorridosAG rec = new RecorridosAG();
        Integer n = 6;

        System.out.println("Elementos impares mayores que " + n + ":");

        List<Integer> pre = rec.numerosImparesMayoresQuePreOrden(raiz, n);
        System.out.println("PreOrden:   " + pre + " (Esperado: [7, 9, 13, 11])");

        List<Integer> in = rec.numerosImparesMayoresQueInOrden(raiz, n);
        System.out.println("InOrden:    " + in + " (Esperado: [7, 13, 9, 11])");

        List<Integer> post = rec.numerosImparesMayoresQuePostOrden(raiz, n);
        System.out.println("PostOrden:  " + post + " (Esperado: [13, 9, 11, 7])");

        List<Integer> niv = rec.numerosImparesMayoresQuePorNiveles(raiz, n);
        System.out.println("PorNiveles: " + niv + " (Esperado: [7, 9, 11, 13])"); // Wait, levels: 7 (yes), 3(no), 9(yes), 11(yes), 5(yes), 8(no), 13(yes). So [7, 9, 11, 5, 13] or [7, 9, 11, 5, 13]
    }
}
