package tp3;

import java.util.LinkedList;
import java.util.List;
import tp1.ejercicio8.Queue;

public class GeneralTree<T> {
    private T data;
    private List<GeneralTree<T>> children;

    public GeneralTree() {
        this.data = null;
        this.children = new LinkedList<>();
    }

    public GeneralTree(T data) {
        this.data = data;
        this.children = new LinkedList<>();
    }

    public GeneralTree(T data, List<GeneralTree<T>> children) {
        this.data = data;
        this.children = (children != null) ? children : new LinkedList<>();
    }

    public T getData() {
        return data;
    }

    public void setData(T data) {
        this.data = data;
    }

    public List<GeneralTree<T>> getChildren() {
        return children;
    }

    public void setChildren(List<GeneralTree<T>> children) {
        this.children = (children != null) ? children : new LinkedList<>();
    }

    public void addChild(GeneralTree<T> child) {
        if (child != null) {
            this.children.add(child);
        }
    }

    public void removeChild(GeneralTree<T> child) {
        if (child != null) {
            this.children.remove(child);
        }
    }

    public boolean hasChildren() {
        return this.children != null && !this.children.isEmpty();
    }

    public boolean isLeaf() {
        return !hasChildren();
    }

    public boolean isEmpty() {
        return this.data == null && !hasChildren();
    }

    // Ejercicio 3.a
    public int altura() {
        if (this.isEmpty()) {
            return -1;
        }
        if (this.isLeaf()) {
            return 0;
        }
        int maxAlt = -1;
        for (GeneralTree<T> child : this.getChildren()) {
            maxAlt = Math.max(maxAlt, child.altura());
        }
        return maxAlt + 1;
    }

    // Ejercicio 3.b
    public int nivel(T dato) {
        return nivelAux(this, dato, 0);
    }

    private int nivelAux(GeneralTree<T> arbol, T dato, int nivelActual) {
        if (arbol == null || arbol.isEmpty()) {
            return -1;
        }
        if (arbol.getData() != null && arbol.getData().equals(dato)) {
            return nivelActual;
        }
        for (GeneralTree<T> child : arbol.getChildren()) {
            int n = nivelAux(child, dato, nivelActual + 1);
            if (n != -1) {
                return n;
            }
        }
        return -1;
    }

    // Ejercicio 3.c
    public int ancho() {
        if (this.isEmpty()) {
            return 0;
        }
        Queue<GeneralTree<T>> cola = new Queue<>();
        cola.enqueue(this);
        int maxAncho = 0;

        while (!cola.isEmpty()) {
            int nodosNivel = cola.size();
            maxAncho = Math.max(maxAncho, nodosNivel);
            for (int i = 0; i < nodosNivel; i++) {
                GeneralTree<T> actual = cola.dequeue();
                for (GeneralTree<T> child : actual.getChildren()) {
                    cola.enqueue(child);
                }
            }
        }
        return maxAncho;
    }

    // Ejercicio 5
    public boolean esAncestro(T a, T b) {
        if (this.isEmpty()) return false;
        GeneralTree<T> nodoA = buscar(this, a);
        if (nodoA == null) return false;
        for (GeneralTree<T> child : nodoA.getChildren()) {
            if (buscar(child, b) != null) {
                return true;
            }
        }
        return false;
    }

    private GeneralTree<T> buscar(GeneralTree<T> arbol, T dato) {
        if (arbol == null || arbol.isEmpty()) {
            return null;
        }
        if (arbol.getData() != null && arbol.getData().equals(dato)) {
            return arbol;
        }
        for (GeneralTree<T> child : arbol.getChildren()) {
            GeneralTree<T> res = buscar(child, dato);
            if (res != null) {
                return res;
            }
        }
        return null;
    }
}
