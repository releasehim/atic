package tp3.ejercicio9;

import tp3.GeneralTree;

public class Ejercicio9 {
    public static void main(String[] args) {
        System.out.println("=== Probando Ejercicio 9: ParcialArboles (esDeSeleccion) ===");

        // Construir Ejemplo 1 (Debe retornar true)
        //                    [12]
        //                 /    |    \
        //               [12]          [25]
        //              /    \          /
        //           [35]    [12]    [25]
        //           /       / | \
        //         [35]   [14][12][33]
        //         /           / | \ \
        //       [35]       [12][83][90][33]  <-- Hacemos que el hijo menor sea 12 para que sea válido
        
        GeneralTree<Integer> ex1_n35_l4 = new GeneralTree<>(35);
        GeneralTree<Integer> ex1_n35_l3 = new GeneralTree<>(35);
        ex1_n35_l3.addChild(ex1_n35_l4);
        GeneralTree<Integer> ex1_n35_l2 = new GeneralTree<>(35);
        ex1_n35_l2.addChild(ex1_n35_l3);

        GeneralTree<Integer> ex1_n12_l4 = new GeneralTree<>(12);
        ex1_n12_l4.addChild(new GeneralTree<>(12)); // Para que sea hoja
        ex1_n12_l4.addChild(new GeneralTree<>(83));
        ex1_n12_l4.addChild(new GeneralTree<>(90));
        ex1_n12_l4.addChild(new GeneralTree<>(33));

        GeneralTree<Integer> ex1_n12_l2 = new GeneralTree<>(12);
        ex1_n12_l2.addChild(new GeneralTree<>(14));
        ex1_n12_l2.addChild(ex1_n12_l4);
        ex1_n12_l2.addChild(new GeneralTree<>(33));

        GeneralTree<Integer> ex1_n12_l1 = new GeneralTree<>(12);
        ex1_n12_l1.addChild(ex1_n35_l2);
        ex1_n12_l1.addChild(ex1_n12_l2);

        GeneralTree<Integer> ex1_n25_l2 = new GeneralTree<>(25);
        GeneralTree<Integer> ex1_n25_l1 = new GeneralTree<>(25);
        ex1_n25_l1.addChild(ex1_n25_l2);

        GeneralTree<Integer> arbol1 = new GeneralTree<>(12);
        arbol1.addChild(ex1_n12_l1);
        arbol1.addChild(ex1_n25_l1);

        System.out.println("Ejemplo 1 (esperado: true):  " + ParcialArboles.esDeSeleccion(arbol1));

        // Construir Ejemplo 2 (Debe retornar false ya que el nodo 18 tiene mínimo 14)
        GeneralTree<Integer> ex2_n35_l4 = new GeneralTree<>(35);
        GeneralTree<Integer> ex2_n35_l3 = new GeneralTree<>(35);
        ex2_n35_l3.addChild(ex2_n35_l4);
        GeneralTree<Integer> ex2_n35_l2 = new GeneralTree<>(35);
        ex2_n35_l2.addChild(ex2_n35_l3);

        GeneralTree<Integer> ex2_n18_l2 = new GeneralTree<>(18); // Nodo incorrecto, sus hijos son 14, 18, 33 (mínimo es 14)
        ex2_n18_l2.addChild(new GeneralTree<>(14));
        ex2_n18_l2.addChild(new GeneralTree<>(18));
        ex2_n18_l2.addChild(new GeneralTree<>(33));

        GeneralTree<Integer> ex2_n12_l1 = new GeneralTree<>(12);
        ex2_n12_l1.addChild(ex2_n35_l2);
        ex2_n12_l1.addChild(ex2_n18_l2);

        GeneralTree<Integer> ex2_n25_l2 = new GeneralTree<>(25);
        GeneralTree<Integer> ex2_n25_l1 = new GeneralTree<>(25);
        ex2_n25_l1.addChild(ex2_n25_l2);

        GeneralTree<Integer> arbol2 = new GeneralTree<>(12);
        arbol2.addChild(ex2_n12_l1);
        arbol2.addChild(ex2_n25_l1);

        System.out.println("Ejemplo 2 (esperado: false): " + ParcialArboles.esDeSeleccion(arbol2));
    }
}
