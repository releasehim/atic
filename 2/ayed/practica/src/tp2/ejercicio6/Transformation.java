package tp2.ejercicio6;

import tp2.BinaryTree;

public class Transformation {
    private BinaryTree<Integer> arbol;

    public Transformation(BinaryTree<Integer> arbol) {
        this.arbol = arbol;
    }

    public BinaryTree<Integer> suma() {
        if (this.arbol == null || this.arbol.isEmpty()) {
            return new BinaryTree<>();
        }
        BinaryTree<Integer> nuevo = new BinaryTree<>();
        transform(this.arbol, nuevo);
        return nuevo;
    }

    private int transform(BinaryTree<Integer> original, BinaryTree<Integer> nuevo) {
        if (original == null || original.isEmpty()) {
            return 0;
        }
        int sumIzq = 0;
        int sumDer = 0;

        if (original.hasLeftChild()) {
            BinaryTree<Integer> nuevoIzq = new BinaryTree<>();
            nuevo.addLeftChild(nuevoIzq);
            sumIzq = transform(original.getLeftChild(), nuevoIzq);
        }
        if (original.hasRightChild()) {
            BinaryTree<Integer> nuevoDer = new BinaryTree<>();
            nuevo.addRightChild(nuevoDer);
            sumDer = transform(original.getRightChild(), nuevoDer);
        }

        nuevo.setData(sumIzq + sumDer);
        return original.getData() + sumIzq + sumDer;
    }
}
