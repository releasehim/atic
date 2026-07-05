package tp2.ejercicio5;

import tp2.BinaryTree;

public class ProfundidadDeArbolBinario {
    private BinaryTree<Integer> arbol;

    public ProfundidadDeArbolBinario(BinaryTree<Integer> arbol) {
        this.arbol = arbol;
    }

    public int sumaElementosProfundidad(int p) {
        return sumaElementosProfundidadAux(this.arbol, p, 0);
    }

    private int sumaElementosProfundidadAux(BinaryTree<Integer> nodo, int profundidadDestino, int profundidadActual) {
        if (nodo == null || nodo.isEmpty()) {
            return 0;
        }
        if (profundidadActual == profundidadDestino) {
            return nodo.getData();
        }
        int suma = 0;
        if (nodo.hasLeftChild()) {
            suma += sumaElementosProfundidadAux(nodo.getLeftChild(), profundidadDestino, profundidadActual + 1);
        }
        if (nodo.hasRightChild()) {
            suma += sumaElementosProfundidadAux(nodo.getRightChild(), profundidadDestino, profundidadActual + 1);
        }
        return suma;
    }
}
