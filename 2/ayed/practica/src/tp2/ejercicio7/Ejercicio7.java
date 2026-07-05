package tp2.ejercicio7;

import tp2.BinaryTree;

public class Ejercicio7 {
    public static void main(String[] args) {
        System.out.println("=== Probando Ejercicio 7: ParcialArboles (isLeftTree) ===");

        // Construir el árbol de ejemplo:
        //                    [2]
        //                   /   \
        //                 [7]   [-5]
        //                /   \       \
        //             [23]   [6]     [19]
        //             /      / \       \
        //           [-3]  [55] [11]    [4]
        //                               \
        //                               [18]

        BinaryTree<Integer> n18 = new BinaryTree<>(18);
        BinaryTree<Integer> n4 = new BinaryTree<>(4);
        n4.addRightChild(n18);
        BinaryTree<Integer> n19 = new BinaryTree<>(19);
        n19.addRightChild(n4);
        BinaryTree<Integer> n5 = new BinaryTree<>(-5);
        n5.addLeftChild(n19);

        BinaryTree<Integer> n3 = new BinaryTree<>(-3);
        BinaryTree<Integer> n23 = new BinaryTree<>(23);
        n23.addLeftChild(n3);

        BinaryTree<Integer> n55 = new BinaryTree<>(55);
        BinaryTree<Integer> n11 = new BinaryTree<>(11);
        BinaryTree<Integer> n6 = new BinaryTree<>(6);
        n6.addLeftChild(n55);
        n6.addRightChild(n11);

        BinaryTree<Integer> n7 = new BinaryTree<>(7);
        n7.addLeftChild(n23);
        n7.addRightChild(n6);

        BinaryTree<Integer> raiz = new BinaryTree<>(2);
        raiz.addLeftChild(n7);
        raiz.addRightChild(n5);

        ParcialArboles parcial = new ParcialArboles(raiz);

        System.out.println("isLeftTree(7) (esperado: true): " + parcial.isLeftTree(7));
        System.out.println("isLeftTree(2) (esperado: false): " + parcial.isLeftTree(2));
        System.out.println("isLeftTree(-5) (esperado: true): " + parcial.isLeftTree(-5));
        System.out.println("isLeftTree(19) (esperado: false): " + parcial.isLeftTree(19));
        System.out.println("isLeftTree(-3) (esperado: false): " + parcial.isLeftTree(-3));
    }
}
