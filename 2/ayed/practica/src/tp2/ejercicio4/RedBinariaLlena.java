package tp2.ejercicio4;

import tp2.BinaryTree;

public class RedBinariaLlena {
    private BinaryTree<Integer> red;

    public RedBinariaLlena(BinaryTree<Integer> red) {
        this.red = red;
    }

    public int retardoReenvio() {
        return calcularRetardo(this.red);
    }

    private int calcularRetardo(BinaryTree<Integer> arbol) {
        if (arbol == null || arbol.isEmpty()) {
            return 0;
        }
        if (arbol.isLeaf()) {
            return arbol.getData();
        }
        int retardoIzq = 0;
        int retardoDer = 0;
        if (arbol.hasLeftChild()) {
            retardoIzq = calcularRetardo(arbol.getLeftChild());
        }
        if (arbol.hasRightChild()) {
            retardoDer = calcularRetardo(arbol.getRightChild());
        }
        return arbol.getData() + Math.max(retardoIzq, retardoDer);
    }
}
