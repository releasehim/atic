package tp3.ejercicio5;

import tp3.GeneralTree;

public class Ejercicio5 {
    public static void main(String[] args) {
        System.out.println("=== Probando Ejercicio 5: esAncestro ===");

        // Construir el árbol:
        //            [A]
        //          /  |  \
        //        [B] [C] [D]
        //       / \       \
        //     [E] [F]     [G]
        
        GeneralTree<String> nE = new GeneralTree<>("E");
        GeneralTree<String> nF = new GeneralTree<>("F");
        GeneralTree<String> nB = new GeneralTree<>("B");
        nB.addChild(nE);
        nB.addChild(nF);

        GeneralTree<String> nC = new GeneralTree<>("C");

        GeneralTree<String> nG = new GeneralTree<>("G");
        GeneralTree<String> nD = new GeneralTree<>("D");
        nD.addChild(nG);

        GeneralTree<String> raiz = new GeneralTree<>("A");
        raiz.addChild(nB);
        raiz.addChild(nC);
        raiz.addChild(nD);

        System.out.println("esAncestro('A', 'E') (esperado: true):  " + raiz.esAncestro("A", "E"));
        System.out.println("esAncestro('B', 'F') (esperado: true):  " + raiz.esAncestro("B", "F"));
        System.out.println("esAncestro('B', 'G') (esperado: false): " + raiz.esAncestro("B", "G"));
        System.out.println("esAncestro('C', 'A') (esperado: false): " + raiz.esAncestro("C", "A"));
        System.out.println("esAncestro('A', 'A') (esperado: false): " + raiz.esAncestro("A", "A"));
        System.out.println("esAncestro('X', 'E') (esperado: false): " + raiz.esAncestro("X", "E"));
    }
}
