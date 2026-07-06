package tp3.ejercicio2;

import java.util.LinkedList;
import java.util.List;
import tp3.GeneralTree;
import tp1.ejercicio8.Queue;

public class RecorridosAG {

    public List<Integer> numerosImparesMayoresQuePreOrden(GeneralTree<Integer> a, Integer n) {
        List<Integer> resultado = new LinkedList<>();
        if (a != null && !a.isEmpty()) {
            preOrdenAux(a, n, resultado);
        }
        return resultado;
    }

    private void preOrdenAux(GeneralTree<Integer> a, Integer n, List<Integer> resultado) {
        int val = a.getData();
        if (val % 2 != 0 && val > n) {
            resultado.add(val);
        }
        for (GeneralTree<Integer> child : a.getChildren()) {
            preOrdenAux(child, n, resultado);
        }
    }

    public List<Integer> numerosImparesMayoresQueInOrden(GeneralTree<Integer> a, Integer n) {
        List<Integer> resultado = new LinkedList<>();
        if (a != null && !a.isEmpty()) {
            inOrdenAux(a, n, resultado);
        }
        return resultado;
    }

    private void inOrdenAux(GeneralTree<Integer> a, Integer n, List<Integer> resultado) {
        if (a.hasChildren()) {
            inOrdenAux(a.getChildren().get(0), n, resultado);
        }
        
        int val = a.getData();
        if (val % 2 != 0 && val > n) {
            resultado.add(val);
        }
        
        List<GeneralTree<Integer>> children = a.getChildren();
        for (int i = 1; i < children.size(); i++) {
            inOrdenAux(children.get(i), n, resultado);
        }
    }

    public List<Integer> numerosImparesMayoresQuePostOrden(GeneralTree<Integer> a, Integer n) {
        List<Integer> resultado = new LinkedList<>();
        if (a != null && !a.isEmpty()) {
            postOrdenAux(a, n, resultado);
        }
        return resultado;
    }

    private void postOrdenAux(GeneralTree<Integer> a, Integer n, List<Integer> resultado) {
        for (GeneralTree<Integer> child : a.getChildren()) {
            postOrdenAux(child, n, resultado);
        }
        int val = a.getData();
        if (val % 2 != 0 && val > n) {
            resultado.add(val);
        }
    }

    public List<Integer> numerosImparesMayoresQuePorNiveles(GeneralTree<Integer> a, Integer n) {
        List<Integer> resultado = new LinkedList<>();
        if (a == null || a.isEmpty()) {
            return resultado;
        }
        Queue<GeneralTree<Integer>> cola = new Queue<>();
        cola.enqueue(a);
        
        while (!cola.isEmpty()) {
            GeneralTree<Integer> actual = cola.dequeue();
            int val = actual.getData();
            if (val % 2 != 0 && val > n) {
                resultado.add(val);
            }
            for (GeneralTree<Integer> child : actual.getChildren()) {
                cola.enqueue(child);
            }
        }
        return resultado;
    }
}
