package tp2.ejercicio8;

import tp2.BinaryTree;

public class ParcialArboles {

    public boolean esPrefijo(BinaryTree<Integer> arbol1, BinaryTree<Integer> arbol2) {
        if (arbol1 == null || arbol1.isEmpty()) {
            return true;
        }
        if (arbol2 == null || arbol2.isEmpty()) {
            return false;
        }
        if (!arbol1.getData().equals(arbol2.getData())) {
            return false;
        }

        boolean okIzq = true;
        if (arbol1.hasLeftChild()) {
            if (!arbol2.hasLeftChild()) {
                return false;
            }
            okIzq = esPrefijo(arbol1.getLeftChild(), arbol2.getLeftChild());
        }

        boolean okDer = true;
        if (arbol1.hasRightChild()) {
            if (!arbol2.hasRightChild()) {
                return false;
            }
            okDer = esPrefijo(arbol1.getRightChild(), arbol2.getRightChild());
        }

        return okIzq && okDer;
    }
}
