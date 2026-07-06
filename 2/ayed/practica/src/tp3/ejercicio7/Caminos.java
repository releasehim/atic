package tp3.ejercicio7;

import java.util.ArrayList;
import java.util.List;
import tp3.GeneralTree;

public class Caminos {
    private GeneralTree<Integer> arbol;

    public Caminos(GeneralTree<Integer> arbol) {
        this.arbol = arbol;
    }

    public List<Integer> caminoAHojaMasLejana() {
        List<Integer> caminoMax = new ArrayList<>();
        if (this.arbol != null && !this.arbol.isEmpty()) {
            List<Integer> caminoAct = new ArrayList<>();
            buscarCaminoMax(this.arbol, caminoAct, caminoMax);
        }
        return caminoMax;
    }

    private void buscarCaminoMax(GeneralTree<Integer> nodo, List<Integer> caminoAct, List<Integer> caminoMax) {
        caminoAct.add(nodo.getData());

        if (nodo.isLeaf()) {
            if (caminoAct.size() > caminoMax.size()) {
                caminoMax.clear();
                caminoMax.addAll(caminoAct);
            }
        } else {
            for (GeneralTree<Integer> child : nodo.getChildren()) {
                buscarCaminoMax(child, caminoAct, caminoMax);
            }
        }

        caminoAct.remove(caminoAct.size() - 1); // Backtracking
    }
}
