package tp3.ejercicio11;

import tp3.GeneralTree;

public class Ejercicio11 {
    public static void main(String[] args) {
        System.out.println("=== Probando Ejercicio 11: ParcialArboles (Creciente) ===");

        // Construir Ejemplo 1 (Creciente, debe retornar true)
        //                     [2]           (1 nodo)
        //                    /   \
        //                 [1]   [25]        (2 nodos)
        //                /  \      \
        //             [5]  [4]    [13]     (3 nodos)
        //            /    / | \
        //         [18]  [7][11][3]         (4 nodos)
        //         /         / | \ \
        //      [83]     [33][12][17][9]    (5 nodos)
        
        GeneralTree<Integer> ex1_n83 = new GeneralTree<>(83);
        GeneralTree<Integer> ex1_n18 = new GeneralTree<>(18);
        ex1_n18.addChild(ex1_n83);

        GeneralTree<Integer> ex1_n7 = new GeneralTree<>(7);
        
        GeneralTree<Integer> ex1_n33 = new GeneralTree<>(33);
        GeneralTree<Integer> ex1_n12 = new GeneralTree<>(12);
        GeneralTree<Integer> ex1_n17 = new GeneralTree<>(17);
        GeneralTree<Integer> ex1_n9 = new GeneralTree<>(9);
        
        GeneralTree<Integer> ex1_n11 = new GeneralTree<>(11);
        ex1_n11.addChild(ex1_n33);
        ex1_n11.addChild(ex1_n12);
        ex1_n11.addChild(ex1_n17);
        ex1_n11.addChild(ex1_n9);

        GeneralTree<Integer> ex1_n3 = new GeneralTree<>(3);

        GeneralTree<Integer> ex1_n5 = new GeneralTree<>(5);
        ex1_n5.addChild(ex1_n18);

        GeneralTree<Integer> ex1_n4 = new GeneralTree<>(4);
        ex1_n4.addChild(ex1_n7);
        ex1_n4.addChild(ex1_n11);
        ex1_n4.addChild(ex1_n3);

        GeneralTree<Integer> ex1_n13 = new GeneralTree<>(13);

        GeneralTree<Integer> ex1_n1 = new GeneralTree<>(1);
        ex1_n1.addChild(ex1_n5);
        ex1_n1.addChild(ex1_n4);

        GeneralTree<Integer> ex1_n25 = new GeneralTree<>(25);
        ex1_n25.addChild(ex1_n13);

        GeneralTree<Integer> arbol1 = new GeneralTree<>(2);
        arbol1.addChild(ex1_n1);
        arbol1.addChild(ex1_n25);

        System.out.println("Ejemplo 1 (esperado: true):  " + ParcialArboles.resolver(arbol1));

        // Construir Ejemplo 2 (No Creciente, debe retornar false)
        // En el nivel 3 hay 3 nodos (18, 7, 3) en lugar de 4.
        GeneralTree<Integer> ex2_n83 = new GeneralTree<>(83);
        GeneralTree<Integer> ex2_n18 = new GeneralTree<>(18);
        ex2_n18.addChild(ex2_n83);

        GeneralTree<Integer> ex2_n7 = new GeneralTree<>(7);

        GeneralTree<Integer> ex2_n33 = new GeneralTree<>(33);
        GeneralTree<Integer> ex2_n12 = new GeneralTree<>(12);
        GeneralTree<Integer> ex2_n17 = new GeneralTree<>(17);
        GeneralTree<Integer> ex2_n9 = new GeneralTree<>(9);

        // Ojo: En este ejemplo, el nodo 11 no está en el nivel 3, por ende sus hijos quedan a nivel 4.
        // Pero para simplificar el árbol fallido, quitamos el nodo 11 y hacemos que sus hijos cuelguen
        // de otra parte o simplemente simulamos el nivel 3 con 3 nodos.
        GeneralTree<Integer> ex2_n3 = new GeneralTree<>(3);
        ex2_n3.addChild(ex2_n33);
        ex2_n3.addChild(ex2_n12);
        ex2_n3.addChild(ex2_n17);
        ex2_n3.addChild(ex2_n9);

        GeneralTree<Integer> ex2_n5 = new GeneralTree<>(5);
        ex2_n5.addChild(ex2_n18);

        GeneralTree<Integer> ex2_n4 = new GeneralTree<>(4);
        ex2_n4.addChild(ex2_n7);
        ex2_n4.addChild(ex2_n3);

        GeneralTree<Integer> ex2_n13 = new GeneralTree<>(13);

        GeneralTree<Integer> ex2_n1 = new GeneralTree<>(1);
        ex2_n1.addChild(ex2_n5);
        ex2_n1.addChild(ex2_n4);

        GeneralTree<Integer> ex2_n25 = new GeneralTree<>(25);
        ex2_n25.addChild(ex2_n13);

        GeneralTree<Integer> arbol2 = new GeneralTree<>(2);
        arbol2.addChild(ex2_n1);
        arbol2.addChild(ex2_n25);

        System.out.println("Ejemplo 2 (esperado: false): " + ParcialArboles.resolver(arbol2));
    }
}
