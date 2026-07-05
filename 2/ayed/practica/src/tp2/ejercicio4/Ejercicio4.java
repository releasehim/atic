package tp2.ejercicio4;

import tp2.BinaryTree;

public class Ejercicio4 {
    public static void main(String[] args) {
        System.out.println("=== Probando Ejercicio 4: Red Binaria Llena ===");

        // Construir la red binaria llena del ejemplo (altura 3, 4 niveles de nodos)
        //                  [10]
        //                 /    \
        //               [3]    [5]
        //              /  \    /  \
        //            [9]  [4] [7]  [6]
        //           /  \  / \ / \  / \
        //         [12] [8][1][2][3][5][4][2]
        
        BinaryTree<Integer> n12 = new BinaryTree<>(12);
        BinaryTree<Integer> n8 = new BinaryTree<>(8);
        BinaryTree<Integer> n9 = new BinaryTree<>(9);
        n9.addLeftChild(n12);
        n9.addRightChild(n8);

        BinaryTree<Integer> n1 = new BinaryTree<>(1);
        BinaryTree<Integer> n2 = new BinaryTree<>(2);
        BinaryTree<Integer> n4 = new BinaryTree<>(4);
        n4.addLeftChild(n1);
        n4.addRightChild(n2);

        BinaryTree<Integer> n3_l = new BinaryTree<>(3);
        BinaryTree<Integer> n5_l = new BinaryTree<>(5);
        BinaryTree<Integer> n7 = new BinaryTree<>(7);
        n7.addLeftChild(n3_l);
        n7.addRightChild(n5_l);

        BinaryTree<Integer> n4_r = new BinaryTree<>(4);
        BinaryTree<Integer> n2_r = new BinaryTree<>(2);
        BinaryTree<Integer> n6 = new BinaryTree<>(6);
        n6.addLeftChild(n4_r);
        n6.addRightChild(n2_r);

        BinaryTree<Integer> n3 = new BinaryTree<>(3);
        n3.addLeftChild(n9);
        n3.addRightChild(n4);

        BinaryTree<Integer> n5 = new BinaryTree<>(5);
        n5.addLeftChild(n7);
        n5.addRightChild(n6);

        BinaryTree<Integer> raiz = new BinaryTree<>(10);
        raiz.addLeftChild(n3);
        raiz.addRightChild(n5);

        RedBinariaLlena red = new RedBinariaLlena(raiz);
        int maxRetardo = red.retardoReenvio();

        System.out.println("El mayor retardo de la red es (esperado: 34): " + maxRetardo);
    }
}
