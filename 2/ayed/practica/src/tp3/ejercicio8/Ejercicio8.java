package tp3.ejercicio8;

import tp3.GeneralTree;

public class Ejercicio8 {
    public static void main(String[] args) {
        System.out.println("=== Probando Ejercicio 8: Navidad (Abeto Navideño) ===");

        // Árbol 1: Válido (Raíz con 3 hojas)
        //     [1]
        //    / | \
        //  [2][3][4]
        GeneralTree<Integer> a1 = new GeneralTree<>(1);
        a1.addChild(new GeneralTree<>(2));
        a1.addChild(new GeneralTree<>(3));
        a1.addChild(new GeneralTree<>(4));

        Navidad nav1 = new Navidad(a1);
        System.out.println("Árbol 1 es abeto? (esperado: Yes): " + nav1.esAbetoNavidenio());

        // Árbol 2: Inválido (Raíz con solo 2 hojas)
        //    [1]
        //   /   \
        // [2]   [3]
        GeneralTree<Integer> a2 = new GeneralTree<>(1);
        a2.addChild(new GeneralTree<>(2));
        a2.addChild(new GeneralTree<>(3));

        Navidad nav2 = new Navidad(a2);
        System.out.println("Árbol 2 es abeto? (esperado: No):  " + nav2.esAbetoNavidenio());

        // Árbol 3: Complejo Válido
        //          [1]
        //        /  |  \
        //      [2] [3] [4]
        //     / | \
        //   [5][6][7]
        GeneralTree<Integer> n2 = new GeneralTree<>(2);
        n2.addChild(new GeneralTree<>(5));
        n2.addChild(new GeneralTree<>(6));
        n2.addChild(new GeneralTree<>(7));

        GeneralTree<Integer> a3 = new GeneralTree<>(1);
        a3.addChild(n2);
        a3.addChild(new GeneralTree<>(3));
        a3.addChild(new GeneralTree<>(4));
        
        // Wait, is a3 valid?
        // Node 1 (internal) has children: node 2 (internal), node 3 (leaf), node 4 (leaf).
        // Leaves under node 1: node 3 and node 4. That is only 2 leaves!
        // So node 1 fails the condition because it only has 2 leaf children!
        // Ah! Let's check:
        // "todos los nodos internos (no hojas) tienen al menos 3 hijos que sean hojas."
        // Node 1 is an internal node. Its children are: 2 (internal), 3 (leaf), 4 (leaf).
        // Since it only has 2 children that are leaves, node 1 does not satisfy the condition.
        // So a3 is indeed NOT a Christmas fir!
        // Let's modify a3 to be valid: node 1 must have at least 3 leaf children (e.g. 3, 4, 8) and node 2 must also have 3 leaf children.
        // Let's do that!
        a3.addChild(new GeneralTree<>(8)); // Now node 1 has leaves: 3, 4, 8 (3 leaves). And child 2 is internal.
        // Node 2 has leaves: 5, 6, 7 (3 leaves).
        // Both internal nodes (1 and 2) have 3 leaf children. This is valid!

        Navidad nav3 = new Navidad(a3);
        System.out.println("Árbol 3 es abeto? (esperado: Yes): " + nav3.esAbetoNavidenio());
    }
}
