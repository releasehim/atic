package tp2.ejercicio5;

import tp2.BinaryTree;

public class Ejercicio5 {
    public static void main(String[] args) {
        System.out.println("=== Probando Ejercicio 5: Suma Elementos Profundidad ===");

        // Construir el árbol:
        //            [4]          (Profundidad 0)
        //           /   \
        //         [2]   [6]       (Profundidad 1)
        //        /   \     \
        //      [1]   [3]   [8]    (Profundidad 2)
        BinaryTree<Integer> ab = new BinaryTree<>(4);
        BinaryTree<Integer> hijoIzq = new BinaryTree<>(2);
        hijoIzq.addLeftChild(new BinaryTree<>(1));
        hijoIzq.addRightChild(new BinaryTree<>(3));
        BinaryTree<Integer> hijoDer = new BinaryTree<>(6);
        hijoDer.addRightChild(new BinaryTree<>(8));
        
        ab.addLeftChild(hijoIzq);
        ab.addRightChild(hijoDer);

        ProfundidadDeArbolBinario pab = new ProfundidadDeArbolBinario(ab);

        System.out.println("Suma a profundidad 0 (esperado: 4): " + pab.sumaElementosProfundidad(0));
        System.out.println("Suma a profundidad 1 (esperado: 8): " + pab.sumaElementosProfundidad(1));
        System.out.println("Suma a profundidad 2 (esperado: 12): " + pab.sumaElementosProfundidad(2));
        System.out.println("Suma a profundidad 3 (esperado: 0): " + pab.sumaElementosProfundidad(3));
    }
}
