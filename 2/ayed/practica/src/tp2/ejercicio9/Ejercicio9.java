package tp2.ejercicio9;

import tp2.BinaryTree;

public class Ejercicio9 {
    public static void main(String[] args) {
        System.out.println("=== Probando Ejercicio 9: ParcialArboles (sumAndDif) ===");

        // Construir el árbol original del ejemplo:
        //                    [20]
        //                   /    \
        //                 [5]    [30]
        //                /  \    /   \
        //              [-5] [10][50] [-9]
        //                   /    /     \
        //                 [1]  [4]     [6]

        BinaryTree<Integer> n1 = new BinaryTree<>(1);
        BinaryTree<Integer> n10 = new BinaryTree<>(10);
        n10.addLeftChild(n1);

        BinaryTree<Integer> n4 = new BinaryTree<>(4);
        BinaryTree<Integer> n50 = new BinaryTree<>(50);
        n50.addLeftChild(n4);

        BinaryTree<Integer> n6 = new BinaryTree<>(6);
        BinaryTree<Integer> n9_neg = new BinaryTree<>(-9);
        n9_neg.addRightChild(n6);

        BinaryTree<Integer> n5_neg = new BinaryTree<>(-5);
        
        BinaryTree<Integer> n5 = new BinaryTree<>(5);
        n5.addLeftChild(n5_neg);
        n5.addRightChild(n10);

        BinaryTree<Integer> n30 = new BinaryTree<>(30);
        n30.addLeftChild(n50);
        n30.addRightChild(n9_neg);

        BinaryTree<Integer> raiz = new BinaryTree<>(20);
        raiz.addLeftChild(n5);
        raiz.addRightChild(n30);

        System.out.println("Árbol original:");
        raiz.entreNiveles(0, 3);

        ParcialArboles parcial = new ParcialArboles();
        BinaryTree<?> res = parcial.sumAndDif(raiz);

        System.out.println("\nNuevo árbol transformado (Suma | Diferencia):");
        res.entreNiveles(0, 3);
    }
}
