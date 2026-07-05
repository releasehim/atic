package tp2.ejercicio1;

import tp2.BinaryTree;

public class Ejercicio1 {
    public static void main(String[] args) {
        System.out.println("=== Probando clase BinaryTree ===");
        
        // Crear un árbol de prueba
        //          40
        //         /  \
        //       25    78
        //      /  \
        //    10    32
        BinaryTree<Integer> ab = new BinaryTree<>(40);
        BinaryTree<Integer> hijoIzq = new BinaryTree<>(25);
        hijoIzq.addLeftChild(new BinaryTree<>(10));
        hijoIzq.addRightChild(new BinaryTree<>(32));
        BinaryTree<Integer> hijoDer = new BinaryTree<>(78);
        
        ab.addLeftChild(hijoIzq);
        ab.addRightChild(hijoDer);

        System.out.println("Hojas contadas (esperado: 3): " + ab.contarHojas());

        System.out.println("\nImprimiendo entre niveles 0 y 2 (esperado: 40 \\n 25 78 \\n 10 32):");
        ab.entreNiveles(0, 2);

        System.out.println("\nCreando espejo del árbol...");
        BinaryTree<Integer> esp = ab.espejo();

        System.out.println("Imprimiendo espejo entre niveles 0 y 2 (esperado: 40 \\n 78 25 \\n 32 10):");
        esp.entreNiveles(0, 2);
    }
}
