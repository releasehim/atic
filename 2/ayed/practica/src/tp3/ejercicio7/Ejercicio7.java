package tp3.ejercicio7;

import java.util.List;
import tp3.GeneralTree;

public class Ejercicio7 {
    public static void main(String[] args) {
        System.out.println("=== Probando Ejercicio 7: Caminos ===");

        // Construir el árbol de ejemplo:
        //                     [12]
        //                  /    |    \
        //              [17]    [9]    [15]
        //              /  \     |     /   \
        //           [10] [6]  [8]  [14]  [18]
        //                 |         /  \
        //                [1]      [16] [7]

        GeneralTree<Integer> n1 = new GeneralTree<>(1);
        GeneralTree<Integer> n10 = new GeneralTree<>(10);
        GeneralTree<Integer> n6 = new GeneralTree<>(6);
        n6.addChild(n1);
        GeneralTree<Integer> n17 = new GeneralTree<>(17);
        n17.addChild(n10);
        n17.addChild(n6);

        GeneralTree<Integer> n8 = new GeneralTree<>(8);
        GeneralTree<Integer> n9 = new GeneralTree<>(9);
        n9.addChild(n8);

        GeneralTree<Integer> n16 = new GeneralTree<>(16);
        GeneralTree<Integer> n7 = new GeneralTree<>(7);
        GeneralTree<Integer> n14 = new GeneralTree<>(14);
        n14.addChild(n16);
        n14.addChild(n7);
        
        GeneralTree<Integer> n18 = new GeneralTree<>(18);
        GeneralTree<Integer> n15 = new GeneralTree<>(15);
        n15.addChild(n14);
        n15.addChild(n18);

        GeneralTree<Integer> raiz = new GeneralTree<>(12);
        raiz.addChild(n17);
        raiz.addChild(n9);
        raiz.addChild(n15);

        Caminos caminos = new Caminos(raiz);
        List<Integer> res = caminos.caminoAHojaMasLejana();

        System.out.println("Camino a la hoja más lejana (esperado: [12, 17, 6, 1]): " + res);
    }
}
