package tp2.ejercicio8;

import tp2.BinaryTree;

public class Ejercicio8 {
    public static void main(String[] args) {
        System.out.println("=== Probando Ejercicio 8: ParcialArboles (esPrefijo) ===");

        // Construir arbol1 (común a todos los ejemplos)
        //           [65]
        //          /    \
        //        [37]   [81]
        //            \      \
        //            [47]   [93]
        BinaryTree<Integer> a1_47 = new BinaryTree<>(47);
        BinaryTree<Integer> a1_37 = new BinaryTree<>(37);
        a1_37.addRightChild(a1_47);

        BinaryTree<Integer> a1_93 = new BinaryTree<>(93);
        BinaryTree<Integer> a1_81 = new BinaryTree<>(81);
        a1_81.addRightChild(a1_93);

        BinaryTree<Integer> arbol1 = new BinaryTree<>(65);
        arbol1.addLeftChild(a1_37);
        arbol1.addRightChild(a1_81);

        // Construir arbol2 (Ejemplo 1: Es prefijo)
        BinaryTree<Integer> a2_22 = new BinaryTree<>(22);
        a2_22.addLeftChild(new BinaryTree<>(11));
        a2_22.addRightChild(new BinaryTree<>(29));
        
        BinaryTree<Integer> a2_37 = new BinaryTree<>(37);
        a2_37.addLeftChild(a2_22);
        a2_37.addRightChild(new BinaryTree<>(47));

        BinaryTree<Integer> a2_93 = new BinaryTree<>(93);
        a2_93.addLeftChild(new BinaryTree<>(85));
        a2_93.addRightChild(new BinaryTree<>(94));

        BinaryTree<Integer> a2_81 = new BinaryTree<>(81);
        a2_81.addLeftChild(new BinaryTree<>(76));
        a2_81.addRightChild(a2_93);

        BinaryTree<Integer> arbol2_ej1 = new BinaryTree<>(65);
        arbol2_ej1.addLeftChild(a2_37);
        arbol2_ej1.addRightChild(a2_81);

        // Construir arbol2 (Ejemplo 2: NO es prefijo porque falta 93)
        BinaryTree<Integer> a2_22_ej2 = new BinaryTree<>(22);
        a2_22_ej2.addLeftChild(new BinaryTree<>(11));
        a2_22_ej2.addRightChild(new BinaryTree<>(29));
        
        BinaryTree<Integer> a2_37_ej2 = new BinaryTree<>(37);
        a2_37_ej2.addLeftChild(a2_22_ej2);
        a2_37_ej2.addRightChild(new BinaryTree<>(47));

        BinaryTree<Integer> a2_81_ej2 = new BinaryTree<>(81);
        a2_81_ej2.addRightChild(new BinaryTree<>(76)); // No tiene hijo derecho (falta 93)

        BinaryTree<Integer> arbol2_ej2 = new BinaryTree<>(65);
        arbol2_ej2.addLeftChild(a2_37_ej2);
        arbol2_ej2.addRightChild(a2_81_ej2);

        // Construir arbol2 (Ejemplo 3: NO es prefijo porque cambia 37 por 62)
        BinaryTree<Integer> a2_22_ej3 = new BinaryTree<>(22);
        a2_22_ej3.addLeftChild(new BinaryTree<>(11));
        a2_22_ej3.addRightChild(new BinaryTree<>(29));
        
        BinaryTree<Integer> a2_62_ej3 = new BinaryTree<>(62); // Cambia de 37 a 62
        a2_62_ej3.addLeftChild(a2_22_ej3);
        a2_62_ej3.addRightChild(new BinaryTree<>(47));

        BinaryTree<Integer> a2_93_ej3 = new BinaryTree<>(93);
        a2_93_ej3.addLeftChild(new BinaryTree<>(85));
        a2_93_ej3.addRightChild(new BinaryTree<>(94));

        BinaryTree<Integer> a2_81_ej3 = new BinaryTree<>(81);
        a2_81_ej3.addLeftChild(new BinaryTree<>(76));
        a2_81_ej3.addRightChild(a2_93_ej3);

        BinaryTree<Integer> arbol2_ej3 = new BinaryTree<>(65);
        arbol2_ej3.addLeftChild(a2_62_ej3);
        arbol2_ej3.addRightChild(a2_81_ej3);

        ParcialArboles parcial = new ParcialArboles();

        System.out.println("Ejemplo 1 (esperado: true): " + parcial.esPrefijo(arbol1, arbol2_ej1));
        System.out.println("Ejemplo 2 (esperado: false): " + parcial.esPrefijo(arbol1, arbol2_ej2));
        System.out.println("Ejemplo 3 (esperado: false): " + parcial.esPrefijo(arbol1, arbol2_ej3));
    }
}
