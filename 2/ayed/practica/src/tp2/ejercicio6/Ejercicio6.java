package tp2.ejercicio6;

import tp2.BinaryTree;

public class Ejercicio6 {
    public static void main(String[] args) {
        System.out.println("=== Probando Ejercicio 6: Transformación de Árbol ===");

        // Construir árbol original
        //          [1]
        //         /   \
        //       [2]   [3]
        //      /     /   \
        //    [4]   [5]   [6]
        //         /   \
        //       [7]   [8]
        BinaryTree<Integer> n4 = new BinaryTree<>(4);
        BinaryTree<Integer> n2 = new BinaryTree<>(2);
        n2.addLeftChild(n4);

        BinaryTree<Integer> n7 = new BinaryTree<>(7);
        BinaryTree<Integer> n8 = new BinaryTree<>(8);
        BinaryTree<Integer> n5 = new BinaryTree<>(5);
        n5.addLeftChild(n7);
        n5.addRightChild(n8);

        BinaryTree<Integer> n6 = new BinaryTree<>(6);
        BinaryTree<Integer> n3 = new BinaryTree<>(3);
        n3.addLeftChild(n5);
        n3.addRightChild(n6);

        BinaryTree<Integer> raiz = new BinaryTree<>(1);
        raiz.addLeftChild(n2);
        raiz.addRightChild(n3);

        System.out.println("Árbol original:");
        raiz.entreNiveles(0, 3);

        Transformation trans = new Transformation(raiz);
        BinaryTree<Integer> transformado = trans.suma();

        System.out.println("\nÁrbol transformado:");
        transformado.entreNiveles(0, 3);
    }
}
