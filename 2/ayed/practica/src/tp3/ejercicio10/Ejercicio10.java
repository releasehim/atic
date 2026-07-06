package tp3.ejercicio10;

import java.util.List;
import tp3.GeneralTree;

public class Ejercicio10 {
    public static void main(String[] args) {
        System.out.println("=== Probando Ejercicio 10: ParcialArboles (resolver) ===");

        // Construir el árbol de ejemplo:
        //   Nivel 0:                [1]
        //                          / | \
        //   Nivel 1:           [0]  [1] [1]
        //                     / | \  / \   \
        //   Nivel 2:       [1] [1] [1] [0] [0]
        //                 / | \
        //   Nivel 3:    [0] [0] [1]
        //                     \
        //   Nivel 4:           [1]

        GeneralTree<Integer> n1_l4 = new GeneralTree<>(1);

        GeneralTree<Integer> n0_l3_1 = new GeneralTree<>(0);
        GeneralTree<Integer> n0_l3_2 = new GeneralTree<>(0);
        n0_l3_2.addChild(n1_l4); // El hijo en el nivel 4
        GeneralTree<Integer> n1_l3 = new GeneralTree<>(1);

        GeneralTree<Integer> n1_l2_1 = new GeneralTree<>(1);
        n1_l2_1.addChild(n0_l3_1);
        n1_l2_1.addChild(n0_l3_2);
        n1_l2_1.addChild(n1_l3);

        GeneralTree<Integer> n1_l2_2 = new GeneralTree<>(1);
        GeneralTree<Integer> n1_l2_3 = new GeneralTree<>(1);

        GeneralTree<Integer> n0_l1 = new GeneralTree<>(0);
        n0_l1.addChild(n1_l2_1);
        n0_l1.addChild(n1_l2_2);
        n0_l1.addChild(n1_l2_3);

        GeneralTree<Integer> n1_l2_4 = new GeneralTree<>(1);
        GeneralTree<Integer> n0_l2_5 = new GeneralTree<>(0);
        GeneralTree<Integer> n1_l1_2 = new GeneralTree<>(1);
        n1_l1_2.addChild(n1_l2_4);
        n1_l1_2.addChild(n0_l2_5);

        GeneralTree<Integer> n0_l2_6 = new GeneralTree<>(0);
        GeneralTree<Integer> n1_l1_3 = new GeneralTree<>(1);
        n1_l1_3.addChild(n0_l2_6);

        GeneralTree<Integer> raiz = new GeneralTree<>(1);
        raiz.addChild(n0_l1);
        raiz.addChild(n1_l1_2);
        raiz.addChild(n1_l1_3);

        List<Integer> resultado = ParcialArboles.resolver(raiz);
        System.out.println("Camino filtrado de valor máximo (esperado: [1, 1, 1]): " + resultado);
    }
}
