package tp3.ejercicio3;

import tp3.GeneralTree;

public class Ejercicio3 {
    public static void main(String[] args) {
        System.out.println("=== Probando Ejercicio 3: Métodos de GeneralTree (altura, nivel, ancho) ===");

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

        System.out.println("Altura del árbol (esperado: 2): " + raiz.altura());
        System.out.println("Ancho del árbol (esperado: 3): " + raiz.ancho());

        System.out.println("\nNiveles de los nodos:");
        System.out.println("Nivel de A (esperado: 0): " + raiz.nivel("A"));
        System.out.println("Nivel de B (esperado: 1): " + raiz.nivel("B"));
        System.out.println("Nivel de C (esperado: 1): " + raiz.nivel("C"));
        System.out.println("Nivel de E (esperado: 2): " + raiz.nivel("E"));
        System.out.println("Nivel de G (esperado: 2): " + raiz.nivel("G"));
        System.out.println("Nivel de Z (no existe, esperado: -1): " + raiz.nivel("Z"));
    }
}
