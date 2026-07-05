package tp2.ejercicio7;

import tp2.BinaryTree;

public class ParcialArboles {
    private BinaryTree<Integer> arbol;

    public ParcialArboles(BinaryTree<Integer> arbol) {
        this.arbol = arbol;
    }

    public boolean isLeftTree(int num) {
        if (this.arbol == null || this.arbol.isEmpty()) {
            return false;
        }
        BinaryTree<Integer> target = buscar(this.arbol, num);
        if (target == null) {
            return false;
        }

        int cantIzq = -1;
        int cantDer = -1;

        if (target.hasLeftChild()) {
            cantIzq = contarUnicosHijos(target.getLeftChild());
        }
        if (target.hasRightChild()) {
            cantDer = contarUnicosHijos(target.getRightChild());
        }

        return cantIzq > cantDer;
    }

    private BinaryTree<Integer> buscar(BinaryTree<Integer> nodo, int num) {
        if (nodo == null || nodo.isEmpty()) {
            return null;
        }
        if (nodo.getData() == num) {
            return nodo;
        }
        BinaryTree<Integer> encontrado = null;
        if (nodo.hasLeftChild()) {
            encontrado = buscar(nodo.getLeftChild(), num);
        }
        if (encontrado == null && nodo.hasRightChild()) {
            encontrado = buscar(nodo.getRightChild(), num);
        }
        return encontrado;
    }

    private int contarUnicosHijos(BinaryTree<Integer> nodo) {
        if (nodo == null || nodo.isEmpty()) {
            return 0;
        }
        int count = 0;
        // Si tiene exactamente un hijo
        if ((nodo.hasLeftChild() && !nodo.hasRightChild()) || (!nodo.hasLeftChild() && nodo.hasRightChild())) {
            count = 1;
        }
        if (nodo.hasLeftChild()) {
            count += contarUnicosHijos(nodo.getLeftChild());
        }
        if (nodo.hasRightChild()) {
            count += contarUnicosHijos(nodo.getRightChild());
        }
        return count;
    }
}
