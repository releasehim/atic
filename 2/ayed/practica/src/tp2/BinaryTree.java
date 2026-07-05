package tp2;

import tp1.ejercicio8.Queue;

public class BinaryTree<T> {
    private T data;
    private BinaryTree<T> leftChild;
    private BinaryTree<T> rightChild;

    public BinaryTree() {
        this.data = null;
        this.leftChild = null;
        this.rightChild = null;
    }

    public BinaryTree(T data) {
        this.data = data;
        this.leftChild = null;
        this.rightChild = null;
    }

    public T getData() {
        return data;
    }

    public void setData(T data) {
        this.data = data;
    }

    public BinaryTree<T> getLeftChild() {
        if (this.leftChild == null) {
            throw new RuntimeException("No tiene hijo izquierdo");
        }
        return leftChild;
    }

    public BinaryTree<T> getRightChild() {
        if (this.rightChild == null) {
            throw new RuntimeException("No tiene hijo derecho");
        }
        return rightChild;
    }

    public void addLeftChild(BinaryTree<T> child) {
        this.leftChild = child;
    }

    public void addRightChild(BinaryTree<T> child) {
        this.rightChild = child;
    }

    public void removeLeftChild() {
        this.leftChild = null;
    }

    public void removeRightChild() {
        this.rightChild = null;
    }

    public boolean isEmpty() {
        return this.data == null && this.leftChild == null && this.rightChild == null;
    }

    public boolean isLeaf() {
        return !isEmpty() && this.leftChild == null && this.rightChild == null;
    }

    public boolean hasLeftChild() {
        return this.leftChild != null;
    }

    public boolean hasRightChild() {
        return this.rightChild != null;
    }

    @Override
    public String toString() {
        if (isEmpty()) return "";
        return this.data.toString();
    }

    public int contarHojas() {
        if (this.isEmpty()) {
            return 0;
        }
        if (this.isLeaf()) {
            return 1;
        }
        int hojas = 0;
        if (this.hasLeftChild()) {
            hojas += this.getLeftChild().contarHojas();
        }
        if (this.hasRightChild()) {
            hojas += this.getRightChild().contarHojas();
        }
        return hojas;
    }

    public BinaryTree<T> espejo() {
        if (this.isEmpty()) {
            return new BinaryTree<>();
        }
        BinaryTree<T> esp = new BinaryTree<>(this.getData());
        if (this.hasLeftChild()) {
            esp.addRightChild(this.getLeftChild().espejo());
        }
        if (this.hasRightChild()) {
            esp.addLeftChild(this.getRightChild().espejo());
        }
        return esp;
    }

    public void entreNiveles(int n, int m) {
        if (this.isEmpty() || n > m) return;
        Queue<BinaryTree<T>> cola = new Queue<>();
        cola.enqueue(this);
        int nivelActual = 0;
        
        while (!cola.isEmpty() && nivelActual <= m) {
            int nodosNivel = cola.size();
            for (int i = 0; i < nodosNivel; i++) {
                BinaryTree<T> actual = cola.dequeue();
                if (nivelActual >= n && nivelActual <= m) {
                    System.out.print(actual.getData() + " ");
                }
                if (actual.hasLeftChild()) {
                    cola.enqueue(actual.getLeftChild());
                }
                if (actual.hasRightChild()) {
                    cola.enqueue(actual.getRightChild());
                }
            }
            if (nivelActual >= n && nivelActual <= m) {
                System.out.println();
            }
            nivelActual++;
        }
    }
}
